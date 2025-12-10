#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quorridor {
    pub player_pieces: [Piece; 2],
    pub active_player: usize,
    pub walls: [Wall; 18],
    pub walls_remaining: [usize; 2],  // Each player can place up to 9 walls
    //pub number_of_walls_remaining: i32
}

trait Coordinates {
    fn coords(&self) -> (i64, i64);
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Piece {
    pub x: i64,
    pub y: i64,
    //pub number_of_walls_remaining: i32
}

impl Coordinates for Piece {
    fn coords(&self) -> (i64, i64) { (self.x, self.y) }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Wall {
    pub x: i64,
    pub y: i64,
}

impl Coordinates for Wall {
    fn coords(&self) -> (i64, i64) { (self.x, self.y) }
}

pub fn wall_collision(game: &Quorridor, x: i64, y: i64) -> bool {
    game.walls.iter().any(|wall| wall.coords() == (x, y))
}

pub fn player_collision(game: &Quorridor, player_idx: usize, x: i64, y: i64) -> bool {
    for (i, piece) in game.player_pieces.iter().enumerate() {
        if i != player_idx && piece.coords() == (x, y) {
            return true;
        }
    }
    false
}

pub fn move_player_left(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_x = current_x - 1;
    
    // Check bounds and collisions
    if candidate_x >= 0 && !wall_collision(game, candidate_x, current_y) && !player_collision(game, idx, candidate_x, current_y) {
        game.player_pieces[idx].x = candidate_x;
    }
}

pub fn move_player_right(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_x = current_x + 1;
    
    // Check wall between current and candidate position
    if !wall_collision(game, candidate_x, current_y) && !player_collision(game, idx, candidate_x, current_y) {
        game.player_pieces[idx].x = candidate_x;
    }
}

pub fn move_player_up(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_y = current_y + 1;
    
    // Check wall between current and candidate position
    if !wall_collision(game, current_x, candidate_y) && !player_collision(game, idx, current_x, candidate_y) {
        game.player_pieces[idx].y = candidate_y;
    }
}

pub fn move_player_down(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_y = current_y - 1;
    
    // Check bounds and collisions
    if candidate_y >= 0 && !wall_collision(game, current_x, candidate_y) && !player_collision(game, idx, current_x, candidate_y) {
        game.player_pieces[idx].y = candidate_y;
    }
}

pub fn place_wall(game: &mut Quorridor, x: i64, y: i64) {
    let idx = game.active_player;
    
    // Check if player has walls remaining
    if game.walls_remaining[idx] > 0 && !game.walls.iter().any(|w| w.coords() == (x, y)) {
        if idx == 0 {
            game.walls[0 + (9 - game.walls_remaining[idx])] = Wall { x, y };
        } else {
            game.walls[9 + (9 - game.walls_remaining[idx])] = Wall { x, y };
        }
        game.walls_remaining[idx] -= 1;
    }
}