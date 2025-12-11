#[cfg(test)]
mod tests {
    use crate::quorridor::*;
    use crate::piece::Piece;
    use crate::wall::{Wall, Orientation, WallPlacementResult};

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
