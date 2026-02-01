use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::quorridor::{Quorridor, shortest_path_to_goal, GRID_HEIGHT};

pub fn log_game_metrics(game: &Quorridor, filename: &str) {
    // Calculate metrics
    let turn = game.turn_number;
    
    // Manhattan distance to goal for each player (in moves, not grid units)
    // Players move by 2 grid units per turn, so divide by 2
    let p0_manhattan = ((GRID_HEIGHT - 2) as i64 - game.player_pieces[0].y) / 2;
    let p1_manhattan = (game.player_pieces[1].y - 1) / 2;
    
    // Shortest path (BFS) to goal for each player
    let p0_shortest = shortest_path_to_goal(game, 0).unwrap_or(0);
    let p1_shortest = shortest_path_to_goal(game, 1).unwrap_or(0);
    
    // Number of walls placed
    let p0_walls_placed = 10 - game.walls_remaining[0];
    let p1_walls_placed = 10 - game.walls_remaining[1];
    
    // Check if file is empty (needs header)
    let needs_header = std::fs::metadata(filename)
        .map(|m| m.len() == 0)
        .unwrap_or(true);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Unable to open metrics file");
    
    if needs_header {
        writeln!(file, "turn,p0_manhattan,p1_manhattan,p0_shortest_path,p1_shortest_path,p0_walls_placed,p1_walls_placed,is_terminal")
            .expect("Unable to write header");
    }
    
    // Check if game is terminal
    let is_terminal = if game.game_over() { 1 } else { 0 };
    
    // Write metrics
    writeln!(
        file,
        "{},{},{},{},{},{},{},{}",
        turn,
        p0_manhattan,
        p1_manhattan,
        p0_shortest,
        p1_shortest,
        p0_walls_placed,
        p1_walls_placed,
        is_terminal
    ).expect("Unable to write metrics");
}

pub fn create_metrics_file(filename: &str) {
    // Create or truncate the file
    std::fs::write(filename, "").expect("Unable to create metrics file");
}
