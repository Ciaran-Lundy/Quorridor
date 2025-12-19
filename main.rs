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
    UpJump,
    DownJump,
    LeftJump,
    RightJump,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
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
            moves.extend(self.get_special_moves());
            moves.extend(self.get_wall_moves());
            moves
        }
    }

    fn make_move(&mut self, mov: &Self::Move) {
        let success = match mov {
            Move::Up => { move_player(self, 0, -2); true }      // Move by 2 in 18x18 grid
            Move::Down => { move_player(self, 0, 2); true }    // Move by 2 in 18x18 grid
            Move::Left => { move_player(self, -2, 0); true }    // Move by 2 in 18x18 grid
            Move::Right => { move_player(self, 2, 0); true }    // Move by 2 in 18x18 grid
            Move::UpJump => { move_player(self, 0, -4); true }
            Move::DownJump => { move_player(self, 0, 4); true }
            Move::LeftJump => { move_player(self, -4, 0); true }
            Move::RightJump => { move_player(self, 4, 0); true }
            Move::UpLeft => { move_player(self, -2, 2); true }
            Move::UpRight => { move_player(self, -2, 2); true }
            Move::DownLeft => { move_player(self, 2, -2); true }
            Move::DownRight => { move_player(self, 2, -2); true }
            Move::LeftUp => { move_player(self, -2, 2); true }
            Move::LeftDown => { move_player(self, 2, -2); true }
            Move::RightUp => { move_player(self, -2, 2); true }
            Move::RightDown => { move_player(self, 2, -2); true }
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

fn get_ai_move(game: &Quorridor) -> Move {
    println!("\nAI is thinking...");
    let mut mcts = MCTSManager::new(
        game.clone(), 
        mcts_impl::MyMCTS, 
        mcts_impl::MyEvaluator, 
        UCTPolicy::new(1.414),  // Standard UCT exploration constant
        ApproxTable::new(8192)
    );
    mcts.playout_n_parallel(100000, 4);
    
    // Print available moves and their evaluations
    let available = game.available_moves();
    println!("\nMove evaluations:");
    for mov in &available {
        let mut test_game = game.clone();
        test_game.make_move(mov);
        
        let p0_dist = quorridor::shortest_path_to_goal(&test_game, 0).unwrap_or(1000);
        let p1_dist = quorridor::shortest_path_to_goal(&test_game, 1).unwrap_or(1000);
        let score = (p1_dist as i64 - p0_dist as i64) * 1000;
        
        match mov {
            Move::Up => println!("  Up: score = {}", score),
            Move::Down => println!("  Down: score = {}", score),
            Move::Left => println!("  Left: score = {}", score),
            Move::Right => println!("  Right: score = {}", score),
            _ => {}
        }
    }
    
    match mcts.best_move() {
        Some(mov) => {
            // Calculate score for the chosen move
            let mut test_game = game.clone();
            test_game.make_move(&mov);
            let p0_dist = quorridor::shortest_path_to_goal(&test_game, 0).unwrap_or(1000);
            let p1_dist = quorridor::shortest_path_to_goal(&test_game, 1).unwrap_or(1000);
            let chosen_score = (p1_dist as i64 - p0_dist as i64) * 1000;
            
            match &mov {
                Move::PlaceWall(x, y, o) => {
                    // Convert back to display coordinates (divide by 2)
                    println!("\nAI chose: PlaceWall({}, {}, {:?}) with score = {}", x/2, y/2, o, chosen_score);
                }
                _ => {
                    println!("\nAI chose: {:?} with score = {}", mov, chosen_score);
                }
            }
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
    let mut game = Quorridor::default();
    
    println!("=== Quorridor ===");
    println!("Player 0 (A) starts at bottom, needs to reach top (y=8)");
    println!("Player 1 (H - you) starts at top, needs to reach bottom (y=0)");
    println!("Wall placement: Use coordinates 0-6 (e.g., 'w 4 3 h' for horizontal wall)");
    
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

 
