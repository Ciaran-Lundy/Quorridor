#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quorridor {
    pub player_pieces: [Piece; 2],
    pub active_player: usize,
    pub walls: [Wall; 18],
    pub walls_remaining: [usize; 2],
}

trait Coordinates {
    fn coords(&self) -> (i64, i64);
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Piece {
    pub x: i64,
    pub y: i64,
}

impl Coordinates for Piece {
    fn coords(&self) -> (i64, i64) { (self.x, self.y) }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self { Orientation::Horizontal }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WallPlacementResult {
    Success,
    NoWallsRemaining,
    Crosses,
    Overlaps,
    BlocksPath,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Wall {
    pub x: i64,
    pub y: i64,
    pub orientation: Orientation,
}

impl Wall {
    pub fn positions(&self) -> [(i64, i64); 2] {
        match self.orientation {
            Orientation::Horizontal => [(self.x, self.y), (self.x + 1, self.y)],
            Orientation::Vertical => [(self.x, self.y), (self.x, self.y + 1)],
        }
    }
    
    pub fn crosses(&self, other: &Wall) -> bool {
        if self.orientation == other.orientation {
            return false;
        }
        
        match (self.orientation, other.orientation) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                let vx = other.x;
                let vy = other.y;
                vx >= self.x && vx <= self.x + 1 && self.y >= vy && self.y <= vy + 1
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                other.crosses(self)
            }
            _ => false,
        }
    }
}

impl Coordinates for Wall {
    fn coords(&self) -> (i64, i64) { (self.x, self.y) }
}

pub fn move_player_up(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_y = current_y + 1;
    
    if candidate_y < 9 && !wall_collision(game, current_x, candidate_y) && !player_collision(game, idx, current_x, candidate_y) {
        game.player_pieces[idx].y = candidate_y;
    }
}

pub fn move_player_left(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_x = current_x - 1;
    
    if candidate_x >= 0 && !wall_collision(game, candidate_x, current_y) && !player_collision(game, idx, candidate_x, current_y) {
        game.player_pieces[idx].x = candidate_x;
    }
}

pub fn move_player_right(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_x = current_x + 1;
    
    if candidate_x < 9 && !wall_collision(game, candidate_x, current_y) && !player_collision(game, idx, candidate_x, current_y) {
        game.player_pieces[idx].x = candidate_x;
    }
}

pub fn move_player_down(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_y = current_y - 1;
    
    if candidate_y >= 0 && !wall_collision(game, current_x, candidate_y) && !player_collision(game, idx, current_x, candidate_y) {
        game.player_pieces[idx].y = candidate_y;
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
            
            if wall_collision(game, nx, ny) {
                continue;
            }
            
            if player_collision(game, player_idx, nx, ny) {
                continue;
            }
            
            visited.insert((nx, ny));
            queue.push_back((nx, ny, dist + 1));
        }
    }
    
    None
}

pub fn both_players_have_path(game: &Quorridor) -> bool {
    shortest_path_to_goal(game, 0).is_some() && shortest_path_to_goal(game, 1).is_some()
}

pub fn place_wall(game: &mut Quorridor, x: i64, y: i64, orientation: Orientation) -> WallPlacementResult {
    let idx = game.active_player;
    
    if game.walls_remaining[idx] == 0 {
        return WallPlacementResult::NoWallsRemaining;
    }
    
    let new_wall = Wall { x, y, orientation };
    
    let crosses_existing = game.walls.iter().any(|w| {
        if w.x == 99 { return false; }
        new_wall.crosses(w)
    });
    
    if crosses_existing {
        return WallPlacementResult::Crosses;
    }
    
    let overlaps = game.walls.iter().any(|w| {
        if w.x == 99 { return false; }
        
        if w.orientation == new_wall.orientation {
            let new_positions = new_wall.positions();
            return w.positions().iter().any(|pos| new_positions.contains(pos));
        }
        
        false
    });
    
    if overlaps {
        return WallPlacementResult::Overlaps;
    }
    
    let wall_index = if idx == 0 {
        9 - game.walls_remaining[idx]
    } else {
        9 + (9 - game.walls_remaining[idx])
    };
    
    let old_wall = game.walls[wall_index];
    game.walls[wall_index] = new_wall;
    
    if both_players_have_path(game) {
        game.walls_remaining[idx] -= 1;
        return WallPlacementResult::Success;
    } else {
        game.walls[wall_index] = old_wall;
        return WallPlacementResult::BlocksPath;
    }
}

fn wall_collision(game: &Quorridor, x: i64, y: i64) -> bool {
    for wall in &game.walls {
        if wall.x == 99 { continue; }
        
        let positions = wall.positions();
        match wall.orientation {
            Orientation::Horizontal => {
                for (wx, wy) in positions {
                    if x == wx && (y == wy || y == wy - 1) {
                        return true;
                    }
                }
            }
            Orientation::Vertical => {
                for (wx, wy) in positions {
                    if y == wy && (x == wx || x == wx - 1) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn player_collision(game: &Quorridor, player_idx: usize, x: i64, y: i64) -> bool {
    let opponent_idx = 1 - player_idx;
    game.player_pieces[opponent_idx].x == x && game.player_pieces[opponent_idx].y == y
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game() -> Quorridor {
        Quorridor {
            player_pieces: [Piece { x: 4, y: 0 }, Piece { x: 4, y: 8 }],
            active_player: 0,
            walls: [Wall { x: 99, y: 99, orientation: Orientation::Horizontal }; 18],
            walls_remaining: [9, 9],
        }
    }

    #[test]
    fn test_horizontal_wall_in_bounds() {
        let mut game = create_test_game();
        let result = place_wall(&mut game, 3, 4, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::Success);
        assert_eq!(game.walls_remaining[0], 8);
    }

    #[test]
    fn test_vertical_wall_in_bounds() {
        let mut game = create_test_game();
        let result = place_wall(&mut game, 3, 4, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Success);
        assert_eq!(game.walls_remaining[0], 8);
    }

    #[test]
    fn test_horizontal_wall_at_max_x() {
        let mut game = create_test_game();
        let result = place_wall(&mut game, 7, 4, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::Success);
    }

    #[test]
    fn test_horizontal_wall_at_max_y() {
        let mut game = create_test_game();
        let result = place_wall(&mut game, 3, 8, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::Success);
    }

    #[test]
    fn test_vertical_wall_at_max_y() {
        let mut game = create_test_game();
        let result = place_wall(&mut game, 3, 7, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Success);
    }

    #[test]
    fn test_horizontal_wall_overlap_same_position() {
        let mut game = create_test_game();
        place_wall(&mut game, 3, 4, Orientation::Horizontal);
        let result = place_wall(&mut game, 3, 4, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::Overlaps);
    }

    #[test]
    fn test_horizontal_wall_overlap_adjacent() {
        let mut game = create_test_game();
        place_wall(&mut game, 3, 4, Orientation::Horizontal);
        let result = place_wall(&mut game, 4, 4, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::Overlaps);
    }

    #[test]
    fn test_vertical_wall_overlap_same_position() {
        let mut game = create_test_game();
        place_wall(&mut game, 3, 4, Orientation::Vertical);
        let result = place_wall(&mut game, 3, 4, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Overlaps);
    }

    #[test]
    fn test_vertical_wall_overlap_adjacent() {
        let mut game = create_test_game();
        place_wall(&mut game, 3, 4, Orientation::Vertical);
        let result = place_wall(&mut game, 3, 5, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Overlaps);
    }

    #[test]
    fn test_perpendicular_walls_at_same_point_cross() {
        let mut game = create_test_game();
        place_wall(&mut game, 3, 4, Orientation::Horizontal);
        let result = place_wall(&mut game, 3, 4, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Crosses);
    }

    #[test]
    fn test_walls_crossing_detected() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 5, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Crosses);
    }

    #[test]
    fn test_walls_crossing_opposite_order() {
        let mut game = create_test_game();
        place_wall(&mut game, 5, 6, Orientation::Vertical);
        let result = place_wall(&mut game, 4, 7, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::Crosses);
    }

    #[test]
    fn test_walls_not_crossing_far_vertical() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 6, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Success);
    }

    #[test]
    fn test_no_walls_remaining() {
        let mut game = create_test_game();
        game.walls_remaining[0] = 0;
        let result = place_wall(&mut game, 3, 4, Orientation::Horizontal);
        assert_eq!(result, WallPlacementResult::NoWallsRemaining);
    }

    #[test]
    fn test_wall_positions_horizontal() {
        let wall = Wall { x: 3, y: 4, orientation: Orientation::Horizontal };
        let positions = wall.positions();
        assert_eq!(positions, [(3, 4), (4, 4)]);
    }

    #[test]
    fn test_wall_positions_vertical() {
        let wall = Wall { x: 3, y: 4, orientation: Orientation::Vertical };
        let positions = wall.positions();
        assert_eq!(positions, [(3, 4), (3, 5)]);
    }

    #[test]
    fn test_wall_crosses_method_horizontal_vertical() {
        let h_wall = Wall { x: 4, y: 7, orientation: Orientation::Horizontal };
        let v_wall = Wall { x: 5, y: 6, orientation: Orientation::Vertical };
        assert!(h_wall.crosses(&v_wall));
        assert!(v_wall.crosses(&h_wall));
    }

    #[test]
    fn test_wall_crosses_method_no_crossing() {
        let h_wall = Wall { x: 4, y: 7, orientation: Orientation::Horizontal };
        let v_wall = Wall { x: 6, y: 6, orientation: Orientation::Vertical };
        assert!(!h_wall.crosses(&v_wall));
        assert!(!v_wall.crosses(&h_wall));
    }

    #[test]
    fn test_wall_crosses_same_orientation_returns_false() {
        let h_wall1 = Wall { x: 4, y: 7, orientation: Orientation::Horizontal };
        let h_wall2 = Wall { x: 5, y: 7, orientation: Orientation::Horizontal };
        assert!(!h_wall1.crosses(&h_wall2));
    }

    #[test]
    fn test_multiple_walls_placement() {
        let mut game = create_test_game();
        assert_eq!(place_wall(&mut game, 0, 0, Orientation::Horizontal), WallPlacementResult::Success);
        assert_eq!(place_wall(&mut game, 2, 0, Orientation::Horizontal), WallPlacementResult::Success);
        assert_eq!(place_wall(&mut game, 0, 2, Orientation::Vertical), WallPlacementResult::Success);
        assert_eq!(place_wall(&mut game, 2, 2, Orientation::Vertical), WallPlacementResult::Success);
        assert_eq!(game.walls_remaining[0], 5);
    }

    #[test]
    fn test_wall_placement_alternating_players() {
        let mut game = create_test_game();
        assert_eq!(place_wall(&mut game, 3, 4, Orientation::Horizontal), WallPlacementResult::Success);
        assert_eq!(game.walls_remaining[0], 8);
        
        game.active_player = 1;
        assert_eq!(place_wall(&mut game, 5, 4, Orientation::Horizontal), WallPlacementResult::Success);
        assert_eq!(game.walls_remaining[1], 8);
    }

    #[test]
    fn test_edge_case_wall_at_origin() {
        let mut game = create_test_game();
        assert_eq!(place_wall(&mut game, 0, 0, Orientation::Horizontal), WallPlacementResult::Success);
    }

    #[test]
    fn test_edge_case_wall_at_max_corner() {
        let mut game = create_test_game();
        assert_eq!(place_wall(&mut game, 7, 7, Orientation::Vertical), WallPlacementResult::Success);
    }

    #[test]
    fn test_cross_detection_at_start_of_horizontal() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 4, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Crosses);
    }

    #[test]
    fn test_cross_detection_at_end_of_horizontal() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 5, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Crosses);
    }

    #[test]
    fn test_no_cross_just_before_horizontal() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 3, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Success);
    }

    #[test]
    fn test_no_cross_just_after_horizontal() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 6, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Success);
    }

    #[test]
    fn test_walls_crossing_at_corner() {
        let mut game = create_test_game();
        place_wall(&mut game, 4, 7, Orientation::Horizontal);
        let result = place_wall(&mut game, 4, 6, Orientation::Vertical);
        assert_eq!(result, WallPlacementResult::Crosses);
    }

    #[test]
    fn test_shortest_path_exists_initial() {
        let game = create_test_game();
        assert!(shortest_path_to_goal(&game, 0).is_some());
        assert!(shortest_path_to_goal(&game, 1).is_some());
    }

    #[test]
    fn test_shortest_path_length_initial() {
        let game = create_test_game();
        let p0_path = shortest_path_to_goal(&game, 0);
        assert!(p0_path.is_some());
        assert_eq!(p0_path.unwrap(), 9);
        
        let p1_path = shortest_path_to_goal(&game, 1);
        assert!(p1_path.is_some());
        assert_eq!(p1_path.unwrap(), 9);
    }
}
