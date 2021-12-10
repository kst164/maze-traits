use crate::Maze;
use crate::MazeSolver;

use std::collections::{HashMap, VecDeque};
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;

/// A maze solver that uses the [breadth first
/// search](https://en.wikipedia.org/wiki/Breadth-first_search) search algorithm
#[derive(Default)]
pub struct BreadthFirstSearch();

impl BreadthFirstSearch {
    /// Returns a new instance of `BreadthFirstSearch`
    pub fn new() -> Self {
        Self()
    }
}

impl<M: Maze> MazeSolver<M> for BreadthFirstSearch
where
    M::Key: Hash + Clone + Debug,
{
    fn solve(&mut self, maze: &M, start: &M::Key, goal: &M::Key) -> Vec<M::Key> {
        // Solving from goal, otherwise returned Vec has to reversed at the end.

        // keys: accessed nodes
        // values: accessed from where
        let mut accessed = HashMap::new();
        let mut queue = VecDeque::new();

        let mut solved = false;
        queue.push_back(goal.clone());

        'outer: while let Some(node) = queue.pop_front() {
            for neighbour in maze.neighbours(&node) {
                if accessed.contains_key(&neighbour) {
                    continue;
                }

                accessed.insert(neighbour.clone(), node.clone());

                if &neighbour == start {
                    solved = true;
                    break 'outer;
                }

                queue.push_back(neighbour);
            }
        }

        if solved {
            let mut took_goal = false;

            // Use accessed to build iterator from start, stopping when we reach the goal
            std::iter::successors(Some(start.clone()), |node| accessed.get(node).cloned())
                .take_while(|node| {
                    let old_val = took_goal;
                    if node == goal {
                        took_goal = true;
                    }
                    !old_val
                })
                .collect()
        } else {
            vec![]
        }
    }
}
