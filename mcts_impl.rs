use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;
use std::sync::{Arc, Mutex};

use crate::quorridor::{Quorridor, shortest_path_to_goal, GRID_WIDTH, GRID_HEIGHT};
use crate::moves::Move;
use crate::policy_network::PolicyNetwork;

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
 
#[derive(Clone)]
pub struct MyEvaluator {
    network: Option<Arc<Mutex<PolicyNetwork>>>,
}

impl MyEvaluator {
    pub fn new() -> Self {
        MyEvaluator { network: None }
    }
    
    pub fn with_network(network: Arc<Mutex<PolicyNetwork>>) -> Self {
        MyEvaluator { network: Some(network) }
    }
}
 
impl Evaluator<MyMCTS> for MyEvaluator {
    type StateEvaluation = i64;
 
    fn evaluate_new_state(&self, state: &Quorridor, moves: &Vec<Move>,
        _: Option<SearchHandle<MyMCTS>>)
        -> (Vec<()>, i64) {   
        // Turn number is available via state.turn_number
        
        // Check for terminal states
        if state.player_pieces[0].y >= (GRID_HEIGHT - 2) as i64 {
        return (vec![(); moves.len()], 100000);  // Player 0 wins
        }
        if state.player_pieces[1].y <= 1 {
            return (vec![(); moves.len()], -100000);  // Player 1 wins
        }
        
        // Use network evaluation if available, otherwise fall back to heuristic
        let score = if let Some(network) = &self.network {
            let net = network.lock().unwrap();
            let value = net.evaluate(state);
            (value * 10000.0) as i64  // Scale to match heuristic range
        } else {
            // Use actual BFS shortest path distance to goal
            let p0_distance = shortest_path_to_goal(state, 0);
            let p1_distance = shortest_path_to_goal(state, 1);
            
            // Use path distance heuristic
            match (p0_distance, p1_distance) {
                (None, _) => -500000000,
                (_, None) => -500000000,
                (Some(d0), Some(d1)) => (d1 as i64 - d0 as i64) * 1000,
            }
        };
        
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
