use crate::quorridor::Quorridor;

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
}

pub fn place_wall(game: &mut Quorridor, x: i64, y: i64, orientation: Orientation) -> WallPlacementResult {
    let idx = game.active_player;
    
    let new_wall = Wall { x, y, orientation };
    let walls_placed = 9 - game.walls_remaining[idx];
    let wall_index = if idx == 0 {
        walls_placed
    } else {
        9 + walls_placed
    };
    
    game.walls[wall_index] = new_wall;
    game.walls_remaining[idx] -= 1;
    WallPlacementResult::Success
}
