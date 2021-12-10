use crate::{Maze, MazeGeneratable, MazeMut};

use std::fmt;

/// A simple maze. A grid of squares represent the nodes, and the common edges between the squares
/// may or may not be a wall.
pub struct SquareGrid {
    rows: usize,
    cols: usize,

    // Couple useless values at the edges, but left alone for convenience
    right_walls: Vec<Vec<bool>>, // right_walls[row][col]: Is there a wall to the right?
    down_walls: Vec<Vec<bool>>,

    unmodified: bool,
}

impl SquareGrid {
    /// Returns a new `SquareGrid` with `rows` rows and `cols` columns, with all walls filled
    pub fn new(rows: usize, cols: usize) -> Self {
        let vertical_walls = vec![vec![true; cols]; rows];
        let horizontal_walls = vec![vec![true; cols]; rows];

        Self {
            rows,
            cols,
            right_walls: vertical_walls,
            down_walls: horizontal_walls,
            unmodified: false,
        }
    }

    /// Returns an immutable reference to the wall between `k1` and `k2` if the wall exists
    fn get_wall(&self, k1: &<Self as Maze>::Key, k2: &<Self as Maze>::Key) -> Option<&bool> {
        let (r1, c1) = *k1;
        let (r2, c2) = *k2;

        if r1 == r2 {
            if c1 + 1 == c2 {
                return self.right_walls.get(r1)?.get(c1);
            } else if c2 + 1 == c1 {
                return self.right_walls.get(r1)?.get(c2);
            }
        }

        if c1 == c2 {
            if r1 + 1 == r2 {
                return self.down_walls.get(r1)?.get(c1);
            } else if r2 + 1 == r1 {
                return self.down_walls.get(r2)?.get(c1);
            }
        }

        None
    }

    /// Returns a mutable reference to the wall between `k1` and `k2` if the wall exists
    fn get_wall_mut(
        &mut self,
        k1: &<Self as Maze>::Key,
        k2: &<Self as Maze>::Key,
    ) -> Option<&mut bool> {
        let (r1, c1) = *k1;
        let (r2, c2) = *k2;

        let return_val = {
            if r1 == r2 {
                if c1 + 1 == c2 {
                    self.right_walls.get_mut(r1)?.get_mut(c1)
                } else if c2 + 1 == c1 {
                    self.right_walls.get_mut(r1)?.get_mut(c2)
                } else {
                    None
                }
            } else if c1 == c2 {
                if r1 + 1 == r2 {
                    self.down_walls.get_mut(r1)?.get_mut(c1)
                } else if r2 + 1 == r1 {
                    self.down_walls.get_mut(r2)?.get_mut(c1)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if return_val.is_some() {
            // Returning a mutable reference, meaning its modified
            self.unmodified = false;
        }

        return_val
    }

    /// Returns a `Vec<String>` representing the maze.
    ///
    /// A 4x4 `SquareGrid`, for example, looks like:
    ///
    /// `
    /// +---+---+---+---+
    /// |   |   |   |   |
    /// +   +---+---+---+
    /// |       |   |   |
    /// +   +---+---+---+
    /// |   |   |   |   |
    /// +   +---+---+---+
    /// |               |
    /// +---+---+---+---+
    /// `
    fn get_lines(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(2 * self.rows + 1);

        let mut top_border = String::with_capacity(4 * self.cols + 1);
        for _ in 0..self.cols {
            top_border += "+---";
        }
        top_border += "+";
        out.push(top_border);

        for row in 0..self.rows {
            let mut main_line = String::with_capacity(4 * self.cols + 1);
            let mut bottom_border = String::with_capacity(4 * self.cols + 1);

            main_line += "|";
            bottom_border += "+";

            for col in 0..self.cols {
                main_line += {
                    if self.right_walls[row][col] {
                        "   |"
                    } else {
                        "    "
                    }
                };
                bottom_border += {
                    if self.down_walls[row][col] {
                        "---+"
                    } else {
                        "   +"
                    }
                };
            }

            out.push(main_line);
            out.push(bottom_border);
        }

        out
    }

    /// Returns a string representing the maze, with `*`s at the positions of each element in `keys`.
    ///
    /// A 4x4 maze with solved keys, for example:
    ///
    /// ```
    /// +---+---+---+---+
    /// | * |   |   |   |
    /// +   +---+---+---+
    /// | *     |   |   |
    /// +   +---+---+---+
    /// | * |   |   |   |
    /// +   +---+---+---+
    /// | *   *   *   * |
    /// +---+---+---+---+
    /// ```
    pub fn to_string_with_keys(&self, keys: &Vec<<Self as Maze>::Key>) -> String {
        let mut lines = self.get_lines();
        for &(row, col) in keys {
            if row < self.rows && col < self.cols {
                lines[2 * row + 1].replace_range((4 * col + 2)..(4 * col + 3), "*");
            }
        }

        let mut out = String::with_capacity((2 * self.rows + 1) * (4 * self.cols + 1));
        for line in lines {
            out += &line;
            out += "\n";
        }
        out
    }
}

impl Maze for SquareGrid {
    type Key = (usize, usize);

    fn nodes(&self) -> Vec<Self::Key> {
        let mut out = Vec::with_capacity(self.rows * self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.push((row, col));
            }
        }
        out
    }

    fn adjacent(&self, key: &Self::Key) -> Vec<Self::Key> {
        let mut out = Vec::with_capacity(4);
        let (row, col) = *key;

        if row >= self.rows || col >= self.cols {
            return out;
        }

        if row > 0 {
            out.push((row - 1, col));
        }
        if row < self.rows - 1 {
            out.push((row + 1, col));
        }

        if col > 0 {
            out.push((row, col - 1));
        }
        if col < self.cols - 1 {
            out.push((row, col + 1));
        }

        out
    }

    fn has_wall(&self, k1: &Self::Key, k2: &Self::Key) -> Option<bool> {
        self.get_wall(k1, k2).copied()
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.rows * self.cols
    }
}

impl MazeMut for SquareGrid {
    fn add_wall(&mut self, k1: &Self::Key, k2: &Self::Key) -> Option<bool> {
        self.get_wall_mut(k1, k2)
            .map(|b| std::mem::replace(b, true))
    }

    fn remove_wall(&mut self, k1: &Self::Key, k2: &Self::Key) -> Option<bool> {
        self.get_wall_mut(k1, k2)
            .map(|b| std::mem::replace(b, false))
    }
}

impl MazeGeneratable for SquareGrid {
    fn add_all_walls(&mut self) {
        if self.unmodified {
            return;
        }
        let _old_maze = std::mem::replace(self, Self::new(self.rows, self.cols));
    }

    fn possible_walls(&self) -> Vec<(Self::Key, Self::Key)> {
        let mut out = Vec::with_capacity(2 * self.rows * self.cols - (self.rows + self.cols));

        for row in 0..self.rows {
            for col in 0..self.cols {
                // If not on last row, there's a wall below
                if row + 1 != self.rows {
                    out.push(((row, col), (row + 1, col)));
                }

                // If not on last column, there's a wall to the right
                if col + 1 != self.cols {
                    out.push(((row, col), (row, col + 1)));
                }
            }
        }

        out
    }
}

impl fmt::Display for SquareGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity((2 * self.rows + 1) * (4 * self.cols + 2));
        for line in self.get_lines() {
            out += &line;
            out += "\n";
        }

        write!(f, "{}", out)
    }
}
