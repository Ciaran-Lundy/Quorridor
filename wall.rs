use crate::quorridor::Quorridor;
use std::sync::Arc;

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
    pub fn positions(&self) -> [(i64, i64); 3] {
        match self.orientation {
            Orientation::Horizontal => [(self.x + 1, self.y), (self.x + 2, self.y), (self.x + 3, self.y)],
            Orientation::Vertical => [(self.x, self.y + 1), (self.x, self.y + 2), (self.x, self.y + 3)],
        }
    }
}

pub fn place_wall(game: &mut Quorridor, x: i64, y: i64, orientation: Orientation) -> WallPlacementResult {
    let idx = game.active_player;

    // Check if player has walls remaining
    if game.walls_remaining[idx] == 0 {
        return WallPlacementResult::NoWallsRemaining;
    }

    let wall = Wall { x, y, orientation };
    // Place the wall - use Arc::make_mut for copy-on-write
    let grid = Arc::make_mut(&mut game.grid);
    for (px, py) in wall.positions() {
        grid[py as usize][px as usize] = true;
    }
    
    game.walls_remaining[idx] -= 1;
    WallPlacementResult::Success
}
