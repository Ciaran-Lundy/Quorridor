use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::env;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

use quorridor::{Quorridor, Move, mcts_impl::{MyMCTS, MyEvaluator}, Orientation, policy_network::PolicyNetwork, log_game_metrics, create_metrics_file};

#[derive(Serialize, Deserialize, Debug)]
struct TrainingExample {
    // Board state encoded as flat array
    player0_pos: (i64, i64),
    player1_pos: (i64, i64),
    walls: Vec<(usize, usize)>,  // (x, y) positions of walls
    active_player: usize,
    walls_remaining: [usize; 2],
    
    // MCTS policy (visit counts for each move)
    move_indices: Vec<usize>,  // Index mapping for moves
    visit_counts: Vec<usize>,  // How many times MCTS visited each move
    
    // Game outcome from current player's perspective
    outcome: f32,  // 1.0 = win, -1.0 = loss, 0.0 = draw (unlikely in Quorridor)
}

fn encode_move(mov: &Move) -> usize {
    // Simple encoding: 0-3 basic moves, 4-11 special moves, 12+ wall moves
    match mov {
        Move::Up => 0,
        Move::Down => 1,
        Move::Left => 2,
        Move::Right => 3,
        Move::UpJump => 4,
        Move::DownJump => 5,
        Move::LeftJump => 6,
        Move::RightJump => 7,
        Move::UpLeft => 8,
        Move::UpRight => 9,
        Move::DownLeft => 10,
        Move::DownRight => 11,
        Move::LeftUp => 12,
        Move::LeftDown => 13,
        Move::RightUp => 14,
        Move::RightDown => 15,
        Move::PlaceWall(x, y, orientation) => {
            // Wall moves: base 16 + position encoding
            let pos_idx = (*x / 2) * 10 + (*y / 2);  // Grid position
            let orient = match orientation {
                Orientation::Horizontal => 0,
                Orientation::Vertical => 1,
            };
            16 + (pos_idx * 2 + orient) as usize
        }
    }
}

fn play_self_play_game(game_number: usize, num_playouts: u32, evaluator: &MyEvaluator, use_parallel: bool, log_metrics: bool, metrics_file: &str, max_turns: usize) -> Vec<TrainingExample> {
    let mut game = Quorridor::default();
    let mut examples = Vec::new();
    let mut move_history = Vec::new();
    
    println!("Game {}: Starting self-play...", game_number);
    
    let mut turn = 0;
    while !game.game_over() && turn < max_turns {
        turn += 1;
        
        // Log metrics if enabled
        if log_metrics {
            log_game_metrics(&game, metrics_file);
        }
        
        // Run MCTS to get move distribution
        let mut mcts = MCTSManager::new(
            game.clone(),
            MyMCTS,
            evaluator.clone(),
            UCTPolicy::new(1.414),  // Standard exploration
            ApproxTable::new(8192)
        );
        
        if use_parallel {
            mcts.playout_n_parallel(num_playouts, 4);
        } else {
            mcts.playout_n(num_playouts as u64);
        }
        
        // Get visit counts for all moves (MCTS policy)
        let available_moves = game.available_moves();
        let mut move_indices = Vec::new();
        let mut visit_counts = Vec::new();
        
        // Extract visit counts from MCTS tree
        let tree = mcts.tree();
        for (i, mov) in available_moves.iter().enumerate() {
            move_indices.push(encode_move(mov));
            // For now, use dummy visit counts (would need MCTS API to get real ones)
            visit_counts.push(1);  // TODO: Get actual visit counts from MCTS
        }
        
        // Store training example (we'll set outcome after game ends)
        let mut walls_vec = Vec::new();
        for y in 0..quorridor::GRID_HEIGHT {
            for x in 0..quorridor::GRID_WIDTH {
                if game.grid[y][x] {
                    walls_vec.push((x, y));
                }
            }
        }
        
        let example = TrainingExample {
            player0_pos: (game.player_pieces[0].x, game.player_pieces[0].y),
            player1_pos: (game.player_pieces[1].x, game.player_pieces[1].y),
            walls: walls_vec,
            active_player: game.active_player,
            walls_remaining: game.walls_remaining,
            move_indices,
            visit_counts,
            outcome: 0.0,  // Will be filled in after game ends
        };
        
        examples.push(example);
        
        // Make the move chosen by MCTS
        if let Some(chosen_move) = mcts.best_move() {
            move_history.push((game.active_player, chosen_move.clone()));
            game.make_move(&chosen_move);
        } else {
            println!("  No moves available!");
            break;
        }
        
        if turn % 10 == 0 {
            print!(".");
            std::io::stdout().flush().unwrap();
        }
    }
    
    println!();
    
    // Determine winner
    let winner = if game.player_pieces[0].y >= (quorridor::GRID_HEIGHT - 2) as i64 {
        0
    } else if game.player_pieces[1].y <= 1 {
        1
    } else {
        println!("  Game ended without winner (turn limit)");
        return Vec::new();  // Discard games that don't finish
    };
    
    println!("  Game ended: Player {} wins in {} turns", winner, turn);
    
    // Fill in outcomes for all examples
    for (i, example) in examples.iter_mut().enumerate() {
        let example_player = example.active_player;
        example.outcome = if example_player == winner { 1.0 } else { -1.0 };
    }
    
    examples
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let use_network = args.contains(&"--network".to_string());
    let log_metrics = args.contains(&"--log-metrics".to_string());
    
    // Parse num_games, num_playouts, and max_turns from args
    let num_games = args.iter()
        .position(|a| a == "--games")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    
    let num_playouts = args.iter()
        .position(|a| a == "--playouts")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1000);
    
    let max_turns = args.iter()
        .position(|a| a == "--max-turns")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(200);
    
    if log_metrics {
        create_metrics_file("selfplay_metrics.csv");
        println!("Logging metrics to selfplay_metrics.csv\n");
    }
    
    let (evaluator, use_parallel) = if use_network {
        println!("Loading policy network from model.pt...");
        
        let mut network = PolicyNetwork::new();
        match network.vs_mut().load("model.safetensors") {
            Ok(_) => {
                println!("Network loaded successfully!");
                let evaluator = MyEvaluator::with_network(Arc::new(Mutex::new(network)));
                (evaluator, false)  // Use single-threaded MCTS with network
            },
            Err(e) => {
                eprintln!("Warning: Could not load model.pt: {}", e);
                eprintln!("Falling back to heuristic evaluation");
                (MyEvaluator::new(), true)
            }
        }
    } else {
        println!("Using heuristic evaluation (use --network to enable neural network)");
        (MyEvaluator::new(), true)
    };
    
    println!("=== Quorridor Self-Play Data Generation ===\n");
    
    println!("Configuration:");
    println!("  Games: {}", num_games);
    println!("  MCTS playouts per move: {}", num_playouts);
    println!("  Max turns per game: {}", max_turns);
    println!("  MCTS mode: {}", if use_parallel { "parallel (4 threads)" } else { "single-threaded" });
    println!();
    println!("Usage: selfplay [--games N] [--playouts N] [--max-turns N] [--network] [--log-metrics]");
    println!();
    
    let mut all_examples = Vec::new();
    
    for game_num in 1..=num_games {
        let examples = play_self_play_game(game_num, num_playouts, &evaluator, use_parallel, log_metrics, "selfplay_metrics.csv", max_turns);
        println!("  Generated {} training examples", examples.len());
        all_examples.extend(examples);
    }
    
    // Save to file
    println!("\nSaving {} training examples to file...", all_examples.len());
    
    let file = File::create("training_data.json").expect("Failed to create file");
    let mut writer = BufWriter::new(file);
    
    serde_json::to_writer_pretty(&mut writer, &all_examples)
        .expect("Failed to write training data");
    
    println!("Done! Training data saved to training_data.json");
    println!("\nNext steps:");
    println!("1. Run: cargo run --bin train");
    println!("2. Train network to predict MCTS policy (visit_counts) and outcome");
    println!("3. Iterate: better network → use in MCTS → generate better data");
}
