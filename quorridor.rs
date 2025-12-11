use crate::piece::Piece;
use crate::wall::{Wall, Orientation};
use itertools::iproduct;

pub use crate::piece::move_player;
pub use crate::wall::place_wall;

#[derive(Clone, Debug, PartialEq)]
pub struct Quorridor {
    pub player_pieces: [Piece; 2],
    pub active_player: usize,
    pub walls: [Wall; 18],
    pub walls_remaining: [usize; 2],
}


impl Quorridor {
    pub fn wall_collision(&self, target_x: i64, target_y: i64) -> bool {
        let current_x = self.player_pieces[self.active_player].x;
        let current_y = self.player_pieces[self.active_player].y;
        
        for wall in &self.walls {
            if wall.x == 99 { continue; }
            
            match wall.orientation {
                Orientation::Horizontal => {
                    if (current_y == wall.y - 1 && target_y == wall.y) || 
                       (current_y == wall.y && target_y == wall.y - 1) {
                        if current_x >= wall.x && current_x <= wall.x + 1 {
                            return true;
                        }
                    }
                }
                Orientation::Vertical => {
                    if (current_x == wall.x - 1 && target_x == wall.x) || 
                       (current_x == wall.x && target_x == wall.x - 1) {
                        if current_y >= wall.y && current_y <= wall.y + 1 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn player_collision(&self, player_idx: usize, x: i64, y: i64) -> bool {
        let opponent_idx = 1 - player_idx;
        self.player_pieces[opponent_idx].x == x && self.player_pieces[opponent_idx].y == y
    }
    
    pub fn wall_crosses(&self, x: i64, y: i64, orientation: Orientation) -> bool {
        self.walls.iter().any(|other| {
            if other.x == 99 { return false; }
            if orientation == other.orientation {
                return false;
            }
            match (orientation, other.orientation) {
                (Orientation::Horizontal, Orientation::Vertical) => {
                    other.x >= x && other.x <= x + 1 && y >= other.y && y <= other.y + 1
                }
                (Orientation::Vertical, Orientation::Horizontal) => {
                    other.x >= x && other.x <= x + 1 && other.y >= y && other.y <= y + 1
                }
                _ => false,
            }
        })
    }
    
    pub fn wall_overlaps(&self, x: i64, y: i64, orientation: Orientation) -> bool {
        let new_positions = match orientation {
            Orientation::Horizontal => [(x, y), (x + 1, y)],
            Orientation::Vertical => [(x, y), (x, y + 1)],
        };
        
        self.walls.iter().any(|other| {
            if other.x == 99 { return false; }
            if other.orientation != orientation { return false; }
            
            other.positions().iter().any(|pos| new_positions.contains(pos))
        })
    }
    
    pub fn both_players_have_path(&self) -> bool {
        has_path_to_goal(self, 0) && has_path_to_goal(self, 1)
    }
    
    pub fn wall_blocks_path(&mut self, x: i64, y: i64, orientation: Orientation) -> bool {
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
    }

    pub fn get_movement_moves(&self) -> Vec<crate::Move> {
        let mut moves = Vec::new();
        let current_x = self.player_pieces[self.active_player].x;
        let current_y = self.player_pieces[self.active_player].y;
        for (x, y, mov) in [(0, 1, crate::Move::Up), 
                            (0, -1, crate::Move::Down), 
                            (-1, 0, crate::Move::Left), 
                            (1, 0, crate::Move::Right)] {
            let new_x = current_x + x;
            let new_y = current_y + y;
            if new_x >= 0 && new_x < 9 && new_y >= 0 && new_y < 9 {
                if !self.wall_collision(current_x + x, current_y + y) && !self.player_collision(self.active_player, current_x + x + x, current_y + y + y) {
                    moves.push(mov);
                }
            }
        }
        moves
    }

    fn validate_wall_move(&self, x: i64, y: i64, orientation: &Orientation) -> bool {
        if self.walls_remaining[self.active_player] == 0 {
            return false;
        }
        if orientation == &Orientation::Horizontal && x == 8 {
            return false;
        }
        if orientation == &Orientation::Vertical && y == 8 {
            return false;
        }
        if self.wall_crosses(x, y, *orientation) {
            return false;
        }
        if self.wall_overlaps(x, y, *orientation) {
            return false;
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
        
        for (x, y, orientation) in iproduct!(0..8, 0..9, [Orientation::Horizontal, Orientation::Vertical].iter()) {
            if !self.validate_wall_move(x, y, orientation) {
                continue;
            }
            moves.push(crate::Move::PlaceWall(x, y, orientation.clone()));
            }
        moves
    }

    pub fn game_over(&self) -> bool {
            let player_0_progress_on_board = self.player_pieces[0].y;
            let player_1_progress_on_board = 9 - self.player_pieces[1].y;
            player_0_progress_on_board == 9 || player_1_progress_on_board == 9
    }
}

impl Default for Quorridor {
    fn default() -> Self {
        Quorridor {
            player_pieces: [Piece::default(); 2],
            active_player: 0,
            walls: [Wall::default(); 18],
            walls_remaining: [9, 9],
        }
    }
}


pub fn shortest_path_to_goal(game: &Quorridor, player_idx: usize) -> Option<usize> {
    use std::collections::{VecDeque, HashSet};
    
    let start = game.player_pieces[player_idx];
    let goal_y = if player_idx == 0 { 8 } else { 0 };
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start.x, start.y, 0usize));
    visited.insert((start.x, start.y));
    
    while let Some((x, y, dist)) = queue.pop_front() {
        if y == goal_y {
            return Some(dist);
        }
        
        let moves = [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ];
        
        for (nx, ny) in moves {
            if nx < 0 || nx >= 9 || ny < 0 || ny >= 9 {
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
    let goal_y = if player_idx == 0 { 8 } else { 0 };
    
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    stack.push((start.x, start.y));
    visited.insert((start.x, start.y));
    
    while let Some((x, y)) = stack.pop() {
        if y == goal_y {
            return true;
        }
        
        let moves = [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ];
        
        for (nx, ny) in moves {
            if nx < 0 || nx >= 9 || ny < 0 || ny >= 9 {
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