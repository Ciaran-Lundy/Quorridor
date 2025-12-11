use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;

use itertools::iproduct;
use std::io::{self, Write};

mod piece;
mod wall;
mod quorridor;
mod tests;
mod mcts_impl;

use piece::Piece;
use wall::{Wall, Orientation, WallPlacementResult, place_wall};
use quorridor::{Quorridor, move_player};
 
#[derive(Clone, Debug, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    PlaceWall(i64, i64, Orientation),
}

 
impl GameState for Quorridor {
    type Move = Move;
    type Player = usize;
    type MoveList = Vec<Move>;

    fn current_player(&self) -> Self::Player {
        self.active_player
    }

    fn available_moves(&self) -> Vec<Move> {
        if self.game_over() {
            vec![]
        } else {
            let mut moves = self.get_movement_moves();
            moves.extend(self.get_wall_moves());
            moves
        }
    }

    fn make_move(&mut self, mov: &Self::Move) {
        let success = match mov {
            Move::Up => { move_player(self, 0, 1); true }
            Move::Down => { move_player(self, 0, -1); true }
            Move::Left => { move_player(self, -1, 0); true }
            Move::Right => { move_player(self, 1, 0); true }
            Move::PlaceWall(x, y, orientation) => {
                place_wall(self, *x, *y, *orientation) == WallPlacementResult::Success
            }
        };
        
        if success {
            self.active_player = 1 - self.active_player;
        }
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
        mcts_impl::MyMCTS, 
        mcts_impl::MyEvaluator, 
        UCTPolicy::new(1.414),  // Standard UCT exploration constant
        ApproxTable::new(8192)
    );
    mcts.playout_n_parallel(1000, 4);
    
    match mcts.best_move() {
        Some(mov) => {
            println!("AI chose: {:?}", mov);
            mov
        }
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
    mov
}


fn get_human_move(game: &Quorridor) -> Move {
    
    let available = game.available_moves();
    
    println!("\nYour turn! Available moves:");
    println!("  u - Up");
    println!("  d - Down");
    println!("  l - Left");
    println!("  r - Right");
    println!("  w x y h - Place horizontal wall at (x, y)");
    println!("  w x y v - Place vertical wall at (x, y)");
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

 
