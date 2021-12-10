use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg64;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::{MazeGeneratable, MazeGenerator};

/// Generate a maze using [randomized depth first search](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_depth-first_search)
pub struct DepthFirstSearch();

impl DepthFirstSearch {
    /// Returns a new dfs based generator
    pub fn new() -> Self {
        Self()
    }
}

impl<M: MazeGeneratable> MazeGenerator<M> for DepthFirstSearch
where
    M::Key: Hash + Clone + Debug,
{
    fn generate_from_seed(
        &mut self,
        maze: &mut M,
        start: &M::Key,
        goal: &M::Key,
        seed: [u8; 32],
    ) -> bool {
        maze.add_all_walls();

        let mut rng = Pcg64::from_seed(seed);
        let mut accessed = HashSet::with_capacity(maze.node_count());
        let mut stack = vec![start.clone()];

        // Pop the top element off of the stack
        while let Some(current) = stack.pop() {
            accessed.insert(current.clone());

            let adjacent_unaccessed = maze
                .adjacent(&current) // adjacent
                .into_iter()
                .filter(|n| !accessed.contains(n)) // unaccessed
                .collect::<Vec<_>>();

            if adjacent_unaccessed.len() == 0 {
                // Reached a dead end so we'll take a step back to the previous node
                continue;
            }

            // Not a dead end, so pick a random unvisited node and step onto it, removing the wall
            // in the process
            let random_adjacent = adjacent_unaccessed.choose(&mut rng).cloned().unwrap();
            maze.remove_wall(&current, &random_adjacent);

            // Put the current node back on the stack so that we can backtrack when we actually do
            // reach one.
            stack.push(current);

            // Put the new node at the top of the stack, i.e., step onto this node
            stack.push(random_adjacent);
        }

        // We walked to every reachable node, removing walls as we went. If we still didn't reach
        // the goal, that means its completely unreachable from the start, even with no walls.
        accessed.contains(goal)
    }
}
