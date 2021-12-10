//! A collections of traits and structs related to mazes, solving mazes, and generating mazes.

#![deny(missing_docs)]

/// A collection of a few maze generators.
///
/// Can be enabled using the `rand` feature
pub mod generators;

/// A collection of a few types of grid mazes
pub mod mazes;

/// A collection of a few maze solvers
pub mod solvers;

/// The base trait for a Maze where nodes are connected by edges, and each edge may or may not be a
/// wall.
pub trait Maze {
    /// Key type that identifies a node. eg: `(u32, u32)` for a grid maze
    type Key: Eq;

    /// Returns list of keys of all nodes (eg: squares in a grid) in the maze
    fn nodes(&self) -> Vec<Self::Key>;

    /// Returns list of all adjacent nodes, which may or may not be separated by a wall
    fn adjacent(&self, key: &Self::Key) -> Vec<Self::Key>;

    /// Returns whether there is a wall between neighbours `k1` and `k2`
    ///
    /// Returns `None` if `k1` and `k2` are not neighbouring nodes.
    fn has_wall(&self, k1: &Self::Key, k2: &Self::Key) -> Option<bool>;

    /// Returns list of all adjacent accessible nodes
    #[inline]
    fn neighbours(&self, key: &Self::Key) -> Vec<Self::Key> {
        self.adjacent(key)
            .into_iter()
            .filter(|k| self.has_wall(key, k) == Some(false))
            .collect()
    }

    /// Takes a [`MazeSolver`], start, and goal, and returns a `Vec` with the order of nodes to
    /// travel from start to goal.
    fn solve(
        &self,
        s: &mut impl MazeSolver<Self>,
        start: &Self::Key,
        goal: &Self::Key,
    ) -> Vec<Self::Key> {
        s.solve(self, start, goal)
    }

    /// Total number of nodes in the maze. Ideally would be overridden
    fn node_count(&self) -> usize {
        self.nodes().len()
    }
}

/// A mutable [`Maze`]
pub trait MazeMut: Maze {
    /// Add a wall between neighbouring nodes `k1` and `k2`
    ///
    /// Returns `None` if they aren't neighbours, otherwise returns whether the wall already
    /// existed.
    fn add_wall(&mut self, k1: &Self::Key, k2: &Self::Key) -> Option<bool>;

    /// Remove the wall between neighbouring nodes `k1` aand `k2`
    ///
    /// Returns `None` if they aren't neighbours, otherwise returns whether the wall already
    /// existed.
    fn remove_wall(&mut self, k1: &Self::Key, k2: &Self::Key) -> Option<bool>;
}

/// A generatable [`Maze`]
///
/// All generatable [`Maze`] types must implement these helper functions
pub trait MazeGeneratable: MazeMut {
    /// Add all possible walls, so that any two adjacent nodes are separated by a wall
    fn add_all_walls(&mut self);

    /// List all possible walls
    fn possible_walls(&self) -> Vec<(Self::Key, Self::Key)>;
}

/// The base trait for a [`Maze`] solver
pub trait MazeSolver<M: Maze + ?Sized> {
    /// Takes a [`Maze`], start, and goal, and returns a `Vec` with the order of nodes to
    /// travel from start to goal.
    fn solve(&mut self, maze: &M, start: &M::Key, goal: &M::Key) -> Vec<M::Key>;
}

/// The base trait for a [`Maze`] generator
pub trait MazeGenerator<M: MazeMut + ?Sized> {
    /// Takes a maze, start, goal, and seed, and creates a solvable maze
    ///
    /// Returns false if it is impossible to create a solvable maze, otherwise true. This is only
    /// possible with generalized graphs, grid mazes like the ones in this crate will always return
    /// true.
    fn generate_from_seed(
        &mut self,
        maze: &mut M,
        start: &M::Key,
        goal: &M::Key,
        seed: [u8; 32],
    ) -> bool;

    /// Takes a maze, start, and goal, and creates a solvable maze
    ///
    /// Returns false if it is impossible to create a solvable maze, otherwise true. This is only
    /// possible with generalized graphs, grid mazes like the ones in this crate will always return
    /// true.
    fn generate(&mut self, maze: &mut M, start: &M::Key, goal: &M::Key) -> bool {
        self.generate_from_seed(maze, start, goal, rand::random())
    }
}
