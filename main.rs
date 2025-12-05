use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;
use itertools::Itertools;

mod quorridor;
use quorridor::{Quorridor, 
                Piece,
                Wall,
                move_player_up,
                move_player_down,
                move_player_left,
                move_player_right,
                place_wall
               };
 
#[derive(Clone, Debug, PartialEq)]
pub enum Move {
    Up, Down, Left, Right, PlaceWall
}
 
impl GameState for Quorridor {
    type Move = Move;
    type Player = usize;
    type MoveList = Vec<Move>;

    fn current_player(&self) -> Self::Player {
        self.active_player
    }
    fn available_moves(&self) -> Vec<Move> {
        let player_0_progress_on_board = self.player_pieces[0].y;
        let player_1_progress_on_board = 8 - self.player_pieces[1].y;
        if player_0_progress_on_board == 8 || player_1_progress_on_board == 8 {
            vec![]
        } else {
            let mut moves: Vec<Move> = vec![];
            if self.player_pieces[self.active_player].y < 8 {
                moves.push(Move::Up);
            }
            if self.player_pieces[self.active_player].y > 0 {
                moves.push(Move::Down);
            }
            if self.player_pieces[self.active_player].x > 0 {
                moves.push(Move::Left);
            }
            if self.player_pieces[self.active_player].x < 8 {
                moves.push(Move::Right);
            }
            if self.walls_remaining[self.active_player] > 0 {
                moves.push(Move::PlaceWall);
            }
            return moves;
        }
    }

    fn make_move(&mut self, mov: &Self::Move) {
        let it = (0..8).combinations(2);
        let combinations_vec: Vec<Vec<i32>> = it.collect::<Vec<_>>();
        match *mov {
            Move::Up => move_player_up(self),
            Move::Down => move_player_down(self),
            Move::Left => move_player_left(self),
            Move::Right => move_player_right(self),
            Move::PlaceWall => place_wall(self, combinations_vec[0][0] as i64, combinations_vec[0][1] as i64),
        }
        // Switch to the other player after a move
        self.active_player = 1 - self.active_player;
    }
}
 
impl TranspositionHash for Quorridor {
    fn hash(&self) -> u64 {
        self.player_pieces[self.current_player()].y as u64
    }
}
 
struct MyEvaluator;
 
impl Evaluator<MyMCTS> for MyEvaluator {
    type StateEvaluation = i64;
 
    fn evaluate_new_state(&self, state: &Quorridor, moves: &Vec<Move>,
        _: Option<SearchHandle<MyMCTS>>)
        -> (Vec<()>, i64) {
        if state.player_pieces[0].y == 8 && state.player_pieces[1].y != 0 {
            return (vec![(); moves.len()], 1);
        }
        println!("Evaluating state: {:?}", state.player_pieces);
        return (vec![(); moves.len()], 0);
    }
    fn interpret_evaluation_for_player(&self, evaln: &i64, _player: &usize) -> i64 {
        *evaln
    }
    fn evaluate_existing_state(&self, _: &Quorridor,  evaln: &i64, _: SearchHandle<MyMCTS>) -> i64 {
        *evaln
    }
}
 
#[derive(Default)]
struct MyMCTS;
 
impl MCTS for MyMCTS {
    type State = Quorridor;
    type Eval = MyEvaluator;
    type NodeData = ();
    type ExtraThreadData = ();
    type TreePolicy = UCTPolicy;
    type TranspositionTable = ApproxTable<Self>;

    fn cycle_behaviour(&self) -> CycleBehaviour<Self> {
        CycleBehaviour::UseCurrentEvalWhenCycleDetected
    }
}
 
fn main() {
    let game = Quorridor{player_pieces: [Piece{x: 0, y: 0}, Piece{x: 8, y: 8}],
                        active_player: 0,
                        walls: [Wall{x: 99, y: 99}; 18],
                        walls_remaining: [9, 9]};
    let mut mcts = MCTSManager::new(game, MyMCTS, MyEvaluator, UCTPolicy::new(0.5),
        ApproxTable::new(1024));
    mcts.playout_n_parallel(10000, 4); // 10000 playouts, 4 search threads
    mcts.tree().debug_moves();
    assert_eq!(mcts.best_move().unwrap(), Move::Right);
    //assert_eq!(mcts.principal_variation(50),
    //    vec![Move::Forward; 50]);
    //assert_eq!(mcts.principal_variation_states(5),
    //    vec![
    //        Quorridor{x: 0, y: 0},
    //        Quorridor{x: 0, y: 1},
    //        Quorridor{x: 0, y: 2},
    //        Quorridor{x: 0, y: 3},
    //        Quorridor{x: 0, y: 4},
    //        Quorridor{x: 0, y: 5}]);
}