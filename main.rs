use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;

use itertools::iproduct;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use quorridor::{Quorridor, Move, Piece, Wall, mcts_impl::MyEvaluator, move_player, place_wall, Orientation, WallPlacementResult, GRID_HEIGHT, policy_network::PolicyNetwork};
 
fn display_board(game: &Quorridor) {
    let mut board = [
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  0"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  1"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  2"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  3"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  4"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  5"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  6"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  7"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  8"],
        ["|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   ", "|", "   "],
        ["+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "---", "+", "  9"],
        ["0", "   ", "1", "   ", "2", "   ", "3", "   ", "4", "   ", "5", "   ", "6", "   ", "7", "   ", "8", "   ", "9", "   "]];
    for (i, line) in game.grid.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            if cell {
                let cell_length = board[i][j].len();
                let wall_symbol = if cell_length == 3 { " x " } else { "x" };
                board[i][j] = &wall_symbol;
            }
        }
    }
    for (idx, piece) in game.player_pieces.iter().enumerate() {
        board[piece.y as usize][piece.x as usize] = if idx == 0 { " A " } else { " H " };
    }
    for row in &board {
        println!("{}", row.concat());
    }        //return false;
}

fn get_ai_move(game: &Quorridor, evaluator: &MyEvaluator, use_parallel: bool) -> Move {
    println!("\nAI is thinking...");
    
    let mut mcts = MCTSManager::new(
        game.clone(), 
        quorridor::mcts_impl::MyMCTS, 
        evaluator.clone(),
        UCTPolicy::new(0.5),  // Lower value = more exploitation (picks highest scoring moves)
        ApproxTable::new(8192)
    );
    
    if use_parallel {
        mcts.playout_n_parallel(1000, 4);  // Parallel mode (heuristic only)
    } else {
        mcts.playout_n(10000);  // Single-threaded (for network evaluation)
    }
   
    //quorridor::mcts_impl::print_stats();
    
    println!("\nEvaluation of moves:");
    mcts.tree().debug_moves();

    match mcts.best_move() {
        Some(mov) => {
            println!("AI chose: {:?}", mov);
            mov
        },
        None => panic!("No moves available for AI!"),
    }
}


fn capture_input() -> Option<Move> {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    let mov = match input {
        "u" => Some(Move::Up),
        "d" => Some(Move::Down),
        "l" => Some(Move::Left),
        "r" => Some(Move::Right),
        "uu" => Some(Move::UpJump),
        "dd" => Some(Move::DownJump),
        "ll" => Some(Move::LeftJump),
        "rr" => Some(Move::RightJump),
        "ul" => Some(Move::UpLeft),
        "ur" => Some(Move::UpRight),
        "dl" => Some(Move::DownLeft),
        "dr" => Some(Move::DownRight),
        "lu" => Some(Move::LeftUp),
        "ld" => Some(Move::LeftDown),
        "ru" => Some(Move::RightUp),
        "rd" => Some(Move::RightDown),
        _ if input.starts_with("w ") => {
            let parts: Vec<&str> = input.split_whitespace().collect();
            
            if parts.len() == 4 {
                if let (Ok(x), Ok(y)) = (parts[1].parse::<i64>(), parts[2].parse::<i64>()) {

                    let grid_x = x * 2;
                    let grid_y = y * 2;
                    let orientation = match parts[3] {
                        "h" => Some(Orientation::Horizontal),
                        "v" => Some(Orientation::Vertical),
                        _ => None,
                    };
                    if let Some(orient) = orientation {
                        Some(Move::PlaceWall(grid_x, grid_y, orient))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };
    mov
}


fn get_human_move(game: &Quorridor) -> Move {
    
    let available = game.available_moves();
    
    println!("\\nYour turn! Available moves:");
    println!("  u - Up");
    println!("  d - Down");
    println!("  l - Left");
    println!("  r - Right");
    println!("  w x y h - Place horizontal wall at (x, y) where x,y are 0-9");
    println!("  w x y v - Place vertical wall at (x, y) where x,y are 0-9");
    loop {
        let mov = capture_input();
        
        if let Some(mov) = mov {
            // Validate move is legal
            
            if available.contains(&mov) {
                return mov;
            } else {
                println!("Invalid input! Try again.");
            }
        }
    }
}



fn main() {
    let args: Vec<String> = std::env::args().collect();
    let use_network = args.contains(&"--network".to_string());
    
    // Load network if requested
    let (evaluator, use_parallel) = if use_network {
        println!("Loading trained network from model.pt...");
        let mut network = PolicyNetwork::new();
        match network.load("model.safetensors") {
            Ok(_) => {
                println!("Network loaded successfully!\n");
                (MyEvaluator::with_network(Arc::new(Mutex::new(network))), false)
            }
            Err(e) => {
                println!("Failed to load model.pt: {}\n", e);
                println!("Falling back to heuristic evaluation.\n");
                (MyEvaluator::new(), true)
            }
        }
    } else {
        (MyEvaluator::new(), true)
    };
    
    let mut game = Quorridor::default();
    
    println!("=== Quorridor ===");
    println!("Player 0 (A) starts at bottom, needs to reach top (y=8)");
    println!("Player 1 (H - you) starts at top, needs to reach bottom (y=0)");
    println!("Wall placement: Use coordinates 0-6 (e.g., 'w 4 3 h' for horizontal wall)");
    if use_network {
        println!("AI Mode: Trained Neural Network (single-threaded MCTS)");
    } else {
        println!("AI Mode: Path Distance Heuristic (parallel MCTS)");
    }
    println!();
    
    loop {
        display_board(&game);
        
        // Check for winner
        if game.game_over() {
            if game.player_pieces[0].y >= (GRID_HEIGHT - 2) as i64 {
                println!("Player 0 (A) wins!");
            } else {
                println!("Player 1 (H) wins!");
            }
            break;
        }
        let mov = if game.active_player == 0 {
            get_ai_move(&game, &evaluator, use_parallel)
        } else {
            get_human_move(&game)
        };
        
        game.make_move(&mov);
    }
}

 
