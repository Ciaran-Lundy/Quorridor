use crate::piece::Piece;
use crate::wall::{Wall, Orientation};
use itertools::iproduct;

pub use crate::piece::move_player;
pub use crate::wall::place_wall;

// Grid size constants
pub const GRID_WIDTH: usize = 19;
pub const GRID_HEIGHT: usize = 19;

// Players occupy odd positions (1,3,5,...,GRID_HEIGHT-1)
// Walls occupy even positions (0,2,4,...,GRID_WIDTH-2)
// Player position (1,1) represents square 0,0 in old system

#[derive(Clone, Debug, PartialEq)]
pub struct Quorridor {
    pub player_pieces: [Piece; 2],
    pub active_player: usize,
    pub grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],  // true = wall present
    pub walls_remaining: [usize; 2],
}


impl Quorridor {

    pub fn wall_collision(&self, _target_x: i64, _target_y: i64) -> bool {
        self.grid[_target_y as usize][_target_x as usize]
    }

    pub fn player_collision(&self, player_idx: usize, x: i64, y: i64) -> bool {
        let opponent_idx = 1 - player_idx;
        self.player_pieces[opponent_idx].x == x && self.player_pieces[opponent_idx].y == y
    }

    pub fn get_movement_moves(&self) -> Vec<crate::Move> {
        let mut moves = Vec::new();
        let current_x = self.player_pieces[self.active_player].x;
        let current_y = self.player_pieces[self.active_player].y;
        for (dx, dy, mov) in [(0, -1, crate::Move::Up), 
                              (0, 1, crate::Move::Down), 
                              (-1, 0, crate::Move::Left), 
                              (1, 0, crate::Move::Right)] {
            let target_x = current_x + dx + dx;
            let target_y = current_y + dy + dy;
            
            // Players occupy odd positions from 1 to GRID_HEIGHT-1
            if target_x < 1 || target_x >= GRID_HEIGHT as i64 || target_y < 1 || target_y >= GRID_HEIGHT as i64 {
                continue;
            }
            
            if !self.wall_collision(current_x + dx, current_y + dy) {
                if !self.player_collision(self.active_player, target_x, target_y) {
                    moves.push(mov);
                }
            }
        }
        moves
    }

    pub fn get_special_moves(&self) -> Vec<crate::Move> {
        let mut moves = Vec::new();
        let current_x = self.player_pieces[self.active_player].x;
        let current_y = self.player_pieces[self.active_player].y;
        for (dx, dy, mov) in [
                              (0, -1, crate::Move::UpJump), 
                              (0, 1, crate::Move::DownJump), 
                              (-1, 0, crate::Move::LeftJump), 
                              (1, 0, crate::Move::RightJump),] {
            let target_x = current_x + dx + dx + dx + dx;
            let target_y = current_y + dy + dy + dy + dy;
            
            if target_x < 1 || target_x >= GRID_HEIGHT as i64 || target_y < 1 || target_y >= GRID_HEIGHT as i64 {
                continue;
            }
            if !self.wall_collision(current_x + dx, current_y + dy) {
                if self.player_collision(self.active_player, current_x + dx + dx, current_y + dy + dy) {
                    if !self.wall_collision(current_x + dx + dx + dx, current_y + dy + dy + dy) {
                        moves.push(mov);
                    }

                }
            }
        }
        for (dx, dy, dx1, dy1, mov) in [
                        (0, -1, -1, 0, crate::Move::UpLeft), 
                        (0, -1, 1, 0, crate::Move::UpRight),
                        (0, 1, -1, 0, crate::Move::DownLeft),
                        (0, 1, 1, 0, crate::Move::DownRight),
                        (-1, 0, 0, -1, crate::Move::LeftUp),
                        (-1, 0, 0, 1, crate::Move::LeftDown),
                        (1, 0, 0, -1, crate::Move::RightUp),
                        (1, 0, 0, 1, crate::Move::RightDown)] {
            let target_x = current_x + dx + dx + dx1 + dx1;
            let target_y = current_y + dy + dy + dy1 + dy1;
            
            if target_x < 1 || target_x >= GRID_HEIGHT as i64 || target_y < 1 || target_y >= GRID_HEIGHT as i64 {
                continue;
            }

            if !self.wall_collision(current_x + dx, current_y + dy) {
                if self.player_collision(self.active_player, current_x + dx + dx, current_y + dy + dy) {
                    if self.wall_collision(current_x + dx + dx + dx, current_y + dy + dy + dy) {
                        if !self.wall_collision(current_x + dx + dx + dx1, current_y + dy + dy + dy1) {    
                            moves.push(mov);
                        }

                    }
                }
            }
        }
        moves
    }
    
    pub fn both_players_have_path(&self) -> bool {
        has_path_to_goal(self, 0) && has_path_to_goal(self, 1)
    }
    
    pub fn wall_blocks_path(&mut self, _x: i64, _y: i64, _orientation: Orientation) -> bool {
        // TODO: Implement with 18x18 grid
        false
        /*
        let idx = self.active_player;
        let walls_placed = 9 - self.walls_remaining[idx];
        let wall_index = if idx == 0 {
            walls_placed
        } else {
            9 + walls_placed
        };
        
        let new_wall = Wall { x, y, orientation };
        let old_wall = self.walls[wall_index];
        self.walls[wall_index] = new_wall;
        
        let blocks = !self.both_players_have_path();
        
        self.walls[wall_index] = old_wall;
        blocks
        */
    }

    fn validate_wall_move(&self, x: i64, y: i64, orientation: &Orientation) -> bool {
        if self.walls_remaining[self.active_player] == 0 {
            return false;
        }

        let candidate_wall = Wall { x, y, orientation: *orientation };
        for (_x, _y) in candidate_wall.positions() {
            if _x > GRID_WIDTH as i64 - 1|| _y > GRID_HEIGHT as i64 -1 {
                return false;
            }
            if self.wall_collision(_x, _y) {
                return false;
            }

        }
        if self.clone().wall_blocks_path(x, y, *orientation) {
            return false;
        }
        true
    }

    pub fn get_wall_moves(&self) -> Vec<crate::Move> {
        let mut moves = Vec::new();
        
        if self.walls_remaining[self.active_player] == 0 {
            return moves;
        }

        for (x, y, orientation) in iproduct!((0..(GRID_WIDTH - 0) as i64).step_by(2), (0..(GRID_HEIGHT - 0) as i64).step_by(2), [Orientation::Horizontal, Orientation::Vertical].iter()) {
            if !self.validate_wall_move(x, y, orientation) {
                continue;
            }
            moves.push(crate::Move::PlaceWall(x, y, orientation.clone()));
        }
        moves
    }

    pub fn game_over(&self) -> bool {
        self.player_pieces[0].y >= (GRID_HEIGHT - 1) as i64 || self.player_pieces[1].y <= 1
    }
}

impl Default for Quorridor {
    fn default() -> Self {
        let mid_x = ((GRID_WIDTH / 2) | 1) as i64;  // Ensure odd position
        Quorridor {
            player_pieces: [
                Piece { x: mid_x, y: 1 },   // Player 0 starts at bottom middle
                Piece { x: mid_x, y: (GRID_HEIGHT - 2) as i64 }   // Player 1 starts at top middle
            ],
            active_player: 0,
            grid: [[false; GRID_WIDTH]; GRID_HEIGHT],  // No walls initially
            walls_remaining: [10, 10],
        }
    }
}


pub fn shortest_path_to_goal(game: &Quorridor, player_idx: usize) -> Option<usize> {
    use std::collections::{VecDeque, HashSet};
    
    let start = game.player_pieces[player_idx];
    let goal_y = if player_idx == 0 { (GRID_HEIGHT - 2) as i64 } else { 1 };
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start.x, start.y, 0usize));
    visited.insert((start.x, start.y));
    
    while let Some((x, y, dist)) = queue.pop_front() {
        if y == goal_y {
            return Some(dist);
        }
        
        // Move by 2 in grid (odd position to odd position)
        let moves = [
            (x + 2, y),
            (x - 2, y),
            (x, y + 2),
            (x, y - 2),
        ];
        
        for (nx, ny) in moves {
            // Stay within odd positions (1 to GRID_HEIGHT-1)
            if nx < 1 || nx > (GRID_WIDTH - 2) as i64 || ny < 1 || ny > (GRID_HEIGHT - 2) as i64 {
                continue;
            }
            
            if visited.contains(&(nx, ny)) {
                continue;
            }
            
            if game.wall_collision(nx, ny) {
                continue;
            }
            
            if game.player_collision(player_idx, nx, ny) {
                continue;
            }
            
            visited.insert((nx, ny));
            queue.push_back((nx, ny, dist + 1));
        }
    }
    
    None
}

pub fn has_path_to_goal(game: &Quorridor, player_idx: usize) -> bool {

    use std::collections::HashSet;
    
    let start = game.player_pieces[player_idx];
    let goal_y = if player_idx == 0 { (GRID_HEIGHT - 1) as i64 } else { 1 };
    
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    stack.push((start.x, start.y));
    visited.insert((start.x, start.y));
    
    while let Some((x, y)) = stack.pop() {
        if y == goal_y {
            return true;
        }
        
        // Move by 2 in grid (odd position to odd position)
        let moves = [
            (x + 2, y),
            (x - 2, y),
            (x, y + 2),
            (x, y - 2),
        ];
        
        for (nx, ny) in moves {
            // Stay within odd positions (1 to GRID_HEIGHT-1)
            if nx < 1 || nx > (GRID_WIDTH - 2) as i64 || ny < 1 || ny > (GRID_HEIGHT - 2) as i64 {
                continue;
            }
            
            if visited.contains(&(nx, ny)) {
                continue;
            }
            
            if game.wall_collision(nx, ny) {
                continue;
            }
            
            if game.player_collision(player_idx, nx, ny) {
                continue;
            }
            
            visited.insert((nx, ny));
            stack.push((nx, ny));
        }
    }
    
    false
}