use crate::quorridor::Quorridor;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Piece {
    pub x: i64,
    pub y: i64,
}

pub fn move_player(game: &mut Quorridor, x: i64, y: i64) {
    let idx = game.active_player;
    game.player_pieces[idx].x = game.player_pieces[idx].x + x;
    game.player_pieces[idx].y = game.player_pieces[idx].y + y;
}
