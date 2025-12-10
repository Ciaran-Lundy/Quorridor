use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;

mod quorridor;
use quorridor::{Quorridor, 
                Piece,
                Wall,
                Orientation,
                move_player_up,
                move_player_down,
                move_player_left,
                move_player_right,
                place_wall,
                shortest_path_to_goal
               };
 
#[derive(Clone, Debug, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    PlaceWall(i64, i64, Orientation),  // Wall position (x, y, orientation)
}
 
impl GameState for Quorridor {
    type Move = Move;
    type Player = usize;
    type MoveList = Vec<Move>;

    fn current_player(&self) -> Self::Player {
        self.active_player
    }
    fn available_moves(&self) -> Vec<Move> {
        let player_0_progress_on_board = self.player_pieces[0].y;
        let player_1_progress_on_board = 8 - self.player_pieces[1].y;
        if player_0_progress_on_board == 8 || player_1_progress_on_board == 8 {
            vec![]
        } else {
            let mut moves: Vec<Move> = vec![];
            if self.player_pieces[self.active_player].y < 9 {
                moves.push(Move::Up);
            }
            if self.player_pieces[self.active_player].y > 0 {
                moves.push(Move::Down);
            }
            if self.player_pieces[self.active_player].x > 0 {
                moves.push(Move::Left);
            }
            if self.player_pieces[self.active_player].x < 8 {
                moves.push(Move::Right);
            }
            // Generate wall placements - consider positions near both players
            if self.walls_remaining[self.active_player] > 0 {
                let player_x = self.player_pieces[self.active_player].x;
                let player_y = self.player_pieces[self.active_player].y;
                let opponent = 1 - self.active_player;
                let opponent_x = self.player_pieces[opponent].x;
                let opponent_y = self.player_pieces[opponent].y;
                
                // Consider walls within range of either player
                for x in 0..8 {  // 0..8 since horizontal walls need x+1 to be valid
                    for y in 0..8 {  // 0..8 since vertical walls need y+1 to be valid
                        // Within 4 squares of current player or opponent
                        let near_player = (x - player_x).abs() <= 4 && (y - player_y).abs() <= 4;
                        let near_opponent = (x - opponent_x).abs() <= 4 && (y - opponent_y).abs() <= 4;
                        
                        if near_player || near_opponent {
                            // Try both orientations
                            let h_wall = Wall { x, y, orientation: Orientation::Horizontal };
                            let v_wall = Wall { x, y, orientation: Orientation::Vertical };
                            
                            // Check if horizontal wall would conflict
                            let h_conflict = self.walls.iter().any(|w| {
                                if w.x == 99 { return false; }
                                w.positions().iter().any(|pos| h_wall.positions().contains(pos))
                            });
                            if !h_conflict {
                                moves.push(Move::PlaceWall(x, y, Orientation::Horizontal));
                            }
                            
                            // Check if vertical wall would conflict
                            let v_conflict = self.walls.iter().any(|w| {
                                if w.x == 99 { return false; }
                                w.positions().iter().any(|pos| v_wall.positions().contains(pos))
                            });
                            if !v_conflict {
                                moves.push(Move::PlaceWall(x, y, Orientation::Vertical));
                            }
                        }
                    }
                }
            }
            return moves;
        }
    }

    fn make_move(&mut self, mov: &Self::Move) {
        match mov {
            Move::Up => move_player_up(self),
            Move::Down => move_player_down(self),
            Move::Left => move_player_left(self),
            Move::Right => move_player_right(self),
            Move::PlaceWall(x, y, orientation) => {
                place_wall(self, *x, *y, *orientation);
            }
        }
        // Switch to the other player after a move
        self.active_player = 1 - self.active_player;
    }
}
 
impl TranspositionHash for Quorridor {
    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;
        
        hash ^= self.active_player as u64;
        // Hash both players' positions
        hash = hash.wrapping_mul(31).wrapping_add(((self.player_pieces[0].x as u64) << 32) | (self.player_pieces[0].y as u64));
        hash = hash.wrapping_mul(31).wrapping_add(((self.player_pieces[1].x as u64) << 32) | (self.player_pieces[1].y as u64));
        
        // Hash all walls
        for wall in &self.walls {
            if wall.x != 99 && wall.y != 99 {  // Skip uninitialized walls
                hash = hash.wrapping_mul(31).wrapping_add((wall.x as u64) << 4 | (wall.y as u64));
            }
        }

        hash
    }
}
 
struct MyEvaluator;
 
impl Evaluator<MyMCTS> for MyEvaluator {
    type StateEvaluation = i64;
 
    fn evaluate_new_state(&self, state: &Quorridor, moves: &Vec<Move>,
        _: Option<SearchHandle<MyMCTS>>)
        -> (Vec<()>, i64) {
        // Check for terminal states
        if state.player_pieces[0].y >= 8 {
            return (vec![(); moves.len()], 100000);  // Player 0 wins
        }
        if state.player_pieces[1].y <= 0 {
            return (vec![(); moves.len()], -100000);  // Player 1 wins
        }
        
        // Use actual BFS shortest path distance to goal
        let p0_distance = shortest_path_to_goal(state, 0).unwrap_or(1000);
        let p1_distance = shortest_path_to_goal(state, 1).unwrap_or(1000);
        
        // Lower distance is better - higher score for player 0 when p1 is farther
        let score = (p1_distance as i64 - p0_distance as i64) * 1000;
        
        (vec![(); moves.len()], score)
    }
    
    fn interpret_evaluation_for_player(&self, evaln: &i64, player: &usize) -> i64 {
        // Return evaluation from the given player's perspective
        if *player == 0 {
            *evaln
        } else {
            -evaln  // Flip sign for player 1
        }
    }
    
    fn evaluate_existing_state(&self, _: &Quorridor,  evaln: &i64, _: SearchHandle<MyMCTS>) -> i64 {
        *evaln
    }
}
 
#[derive(Default)]
struct MyMCTS;
 
impl MCTS for MyMCTS {
    type State = Quorridor;
    type Eval = MyEvaluator;
    type NodeData = ();
    type ExtraThreadData = ();
    type TreePolicy = UCTPolicy;
    type TranspositionTable = ApproxTable<Self>;

    fn cycle_behaviour(&self) -> CycleBehaviour<Self> {
        CycleBehaviour::UseCurrentEvalWhenCycleDetected
    }
}
 
fn display_board(game: &Quorridor) {
    // Build board representation
    let mut player_grid = [[' '; 9]; 9];  // 9x9 grid for players
    let mut h_wall_grid = [[false; 9]; 9];  // Horizontal walls between rows
    let mut v_wall_grid = [[false; 9]; 9];  // Vertical walls between columns
    
    // Place players on the grid
    player_grid[game.player_pieces[0].y as usize][game.player_pieces[0].x as usize] = 'A';
    player_grid[game.player_pieces[1].y as usize][game.player_pieces[1].x as usize] = 'H';
    
    // Place walls on the grid
    for wall in &game.walls {
        if wall.x != 99 && wall.y != 99 {
            match wall.orientation {
                Orientation::Horizontal => {
                    // Horizontal wall blocks movement up/down between rows
                    if wall.x >= 0 && wall.x < 8 && wall.y >= 0 && wall.y < 9 {
                        h_wall_grid[wall.y as usize][wall.x as usize] = true;
                        h_wall_grid[wall.y as usize][(wall.x + 1) as usize] = true;
                    }
                }
                Orientation::Vertical => {
                    // Vertical wall blocks movement left/right between columns
                    if wall.x >= 0 && wall.x < 9 && wall.y >= 0 && wall.y < 8 {
                        v_wall_grid[wall.y as usize][wall.x as usize] = true;
                        v_wall_grid[(wall.y + 1) as usize][wall.x as usize] = true;
                    }
                }
            }
        }
    }
    
    // Display the board
    println!("\n   0   1   2   3   4   5   6   7   8   9 (wall X coords)");
    println!(" 9 +---+---+---+---+---+---+---+---+---+");
    
    for y in (0..9).rev() {
        // Print player row with vertical walls
        print!("   ");
        for x in 0..9 {
            if v_wall_grid[y][x] {
                print!("X");
            } else {
                print!("|");
            }
            print!(" {} ", player_grid[y][x]);
        }
        println!("|");
        
        // Print horizontal wall row
        if y > 0 {
            print!("{:2} +", y);
            for x in 0..9 {
                if h_wall_grid[y - 1][x] {
                    print!("XXX+");
                } else {
                    print!("---+");
                }
            }
            println!();
        }
    }
    println!(" 0 +---+---+---+---+---+---+---+---+---+");
    
    println!("\nPlayer 0 (A): ({}, {}) - Walls: {}", 
             game.player_pieces[0].x, game.player_pieces[0].y, game.walls_remaining[0]);
    println!("Player 1 (H): ({}, {}) - Walls: {}", 
             game.player_pieces[1].x, game.player_pieces[1].y, game.walls_remaining[1]);
    println!("Current player: {}", game.active_player);
    println!("\nNote: Walls block movement. Place walls at coordinates between player spaces.");
}

fn get_ai_move(game: &Quorridor) -> Move {
    println!("\nAI is thinking...");
    let mut mcts = MCTSManager::new(
        game.clone(), 
        MyMCTS, 
        MyEvaluator, 
        UCTPolicy::new(1.414),  // Standard UCT exploration constant
        ApproxTable::new(8192)
    );
    mcts.playout_n_parallel(50000, 4);
    
    match mcts.best_move() {
        Some(mov) => {
            println!("AI chose: {:?}", mov);
            mov
        }
        None => panic!("No moves available for AI!"),
    }
}

fn get_human_move(game: &Quorridor) -> Move {
    use std::io::{self, Write};
    
    let available = game.available_moves();
    let wall_moves: Vec<_> = available.iter()
        .filter(|m| matches!(m, Move::PlaceWall(_, _, _)))
        .collect();
    
    println!("\nYour turn! Available moves:");
    println!("  u - Up");
    println!("  d - Down");
    println!("  l - Left");
    println!("  r - Right");
    println!("  w x y h - Place horizontal wall at (x, y)");
    println!("  w x y v - Place vertical wall at (x, y)");
    if !wall_moves.is_empty() {
        println!("\nAvailable wall positions (showing first 10): {:?}", 
                 wall_moves.iter().take(10).collect::<Vec<_>>());
    }
    
    loop {
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
            _ if input.starts_with("w ") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() == 4 {
                    if let (Ok(x), Ok(y)) = (parts[1].parse::<i64>(), parts[2].parse::<i64>()) {
                        let orientation = match parts[3] {
                            "h" => Some(Orientation::Horizontal),
                            "v" => Some(Orientation::Vertical),
                            _ => None,
                        };
                        if let Some(orient) = orientation {
                            Some(Move::PlaceWall(x, y, orient))
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
        
        if let Some(mov) = mov {
            // Validate move is legal
            let available = game.available_moves();
            if available.contains(&mov) {
                return mov;
            } else {
                println!("Invalid move! Try again.");
            }
        } else {
            println!("Invalid input! Try again.");
        }
    }
}

fn main() {
    let mut game = Quorridor {
        player_pieces: [Piece{x: 4, y: 0}, Piece{x: 4, y: 8}],
        active_player: 0,
        walls: [Wall{x: 99, y: 99, orientation: Orientation::Horizontal}; 18],
        walls_remaining: [9, 9]
    };
    
    println!("=== Quorridor ===");
    println!("Player 0 (A) starts at bottom, needs to reach y=8");
    println!("Player 1 (H - you) starts at top, needs to reach y=0");
    
    loop {
        display_board(&game);
        
        // Check for winner
        if game.player_pieces[0].y == 8 {
            println!("\n*** AI (Player 0) wins! ***");
            break;
        }
        if game.player_pieces[1].y == 0 {
            println!("\n*** You (Player 1) win! ***");
            break;
        }
        
        let mov = if game.active_player == 0 {
            get_ai_move(&game)
        } else {
            get_human_move(&game)
        };
        
        game.make_move(&mov);
    }
}