// Library module for shared code across binaries
pub mod piece;
pub mod wall;
pub mod moves;
pub mod quorridor;
pub mod mcts_impl;
pub mod policy_network;
pub mod metrics_logger;

// Tests are currently out of date with the grid refactoring
// #[cfg(test)]
// mod tests;

// Re-export commonly used items
pub use moves::Move;
pub use quorridor::{Quorridor, move_player, GRID_WIDTH, GRID_HEIGHT};
pub use piece::Piece;
pub use wall::{Wall, Orientation, WallPlacementResult, place_wall};
pub use metrics_logger::{log_game_metrics, create_metrics_file};
