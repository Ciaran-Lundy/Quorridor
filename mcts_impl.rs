use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;

use crate::quorridor::{Quorridor, shortest_path_to_goal, GRID_WIDTH, GRID_HEIGHT};
use crate::Move;

impl TranspositionHash for Quorridor {
    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;
        
        hash ^= self.active_player as u64;
        // Hash both players' positions
        hash = hash.wrapping_mul(31).wrapping_add(((self.player_pieces[0].x as u64) << 32) | (self.player_pieces[0].y as u64));
        hash = hash.wrapping_mul(31).wrapping_add(((self.player_pieces[1].x as u64) << 32) | (self.player_pieces[1].y as u64));
        
        // Hash the grid (only even positions where walls can be)
        for y in (0..GRID_HEIGHT).step_by(2) {
            for x in (0..GRID_WIDTH).step_by(2) {
                if self.grid[y][x] {
                    hash = hash.wrapping_mul(31).wrapping_add((x as u64) << 8 | (y as u64));
                }
            }
        }

        hash
    }
}
 
pub struct MyEvaluator;
 
impl Evaluator<MyMCTS> for MyEvaluator {
    type StateEvaluation = i64;
 
    fn evaluate_new_state(&self, state: &Quorridor, moves: &Vec<Move>,
        _: Option<SearchHandle<MyMCTS>>)
        -> (Vec<()>, i64) {
        // Check for terminal states
        if state.player_pieces[0].y >= (GRID_HEIGHT - 1) as i64 {
            return (vec![(); moves.len()], 100000);  // Player 0 wins
        }
        if state.player_pieces[1].y <= 1 {
            return (vec![(); moves.len()], -100000);  // Player 1 wins
        }
        
        // Use actual BFS shortest path distance to goal
        let p0_distance = shortest_path_to_goal(state, 0).unwrap_or(1000);
        let p1_distance = shortest_path_to_goal(state, 1).unwrap_or(1000);
        
        // Lower distance is better - higher score for player 0 when p1 is farther
        let score = (p1_distance as i64 - p0_distance as i64) * 1000;
        
        (vec![(); moves.len()], score)
    }
    
    fn interpret_evaluation_for_player(&self, evaln: &i64, player: &usize) -> i64 {
        // Return evaluation from the given player's perspective
        if *player == 0 {
            *evaln
        } else {
            -evaln  // Flip sign for player 1
        }
    }
    
    fn evaluate_existing_state(&self, _: &Quorridor,  evaln: &i64, _: SearchHandle<MyMCTS>) -> i64 {
        *evaln
    }
}
 
#[derive(Default)]
pub struct MyMCTS;
 
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
