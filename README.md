A modular library to represent, solve, and generate mazes.

A maze is represented similar to a graph, where each node is a position in a maze. For example, in a
SquareGrid maze, each node is a square in the maze. Each node is adjacent to some neighbours, and
there may or may not be a wall between two adjacent nodes.

A solver takes a maze structure, along with a start and end node, and tries to find a path from the
start node to the end node, if possible.

A generator takes an instance of a maze, along with a start and end node, and creates a new maze
where there is a path from the start to the end.

An important point is that even with differently shaped mazes, such as hexagonal/triangular grids,
or even 3 dimensional mazes, the same solvers and generators can be used, as long as they implement
the necessary traits (`Maze` for solvers, `MazeGeneratable` for generators).

The example code in `examples/gen_and_solve.rs` creates and solves a 15x15 square maze.

```bash
cargo run --quiet --example gen_and_solve
```
Output:
```
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| * |                           |                       |   |
+   +---+---+   +---+---+   +   +   +---+---+---+---+   +   +
| *   *   * |   |           |       |       |           |   |
+---+---+   +   +   +---+---+---+---+   +---+   +---+---+   +
|   | *   * |   |       |       |               |       |   |
+   +   +---+   +---+   +   +---+   +---+---+---+   +   +   +
|   | * |       |   |   |       |                   |   |   |
+   +   +   +---+   +   +---+   +---+---+---+---+---+   +   +
|   | * |           |       |           |       |           |
+   +   +---+---+---+---+   +---+   +   +   +   +   +---+---+
|   | *   *   *   *   * |       |   |   |   |       |       |
+   +---+---+---+---+   +   +   +---+   +   +---+---+   +   +
| *   *   *   * | *   * |   |       |   |   |           |   |
+   +---+---+   +   +---+---+---+   +   +   +   +---+---+   +
| * |       | *   * |           |   |   |   |   |   |       |
+   +---+   +---+---+   +   +   +   +   +   +   +   +   +   +
| *   *   * |           |   |       |       |       |   |   |
+---+---+   +---+   +---+   +---+---+---+---+---+   +   +   +
|       | *   * |       |                           |   |   |
+---+   +---+   +---+   +---+---+---+---+---+---+---+   +---+
|           | *   * |       |                       |       |
+   +---+---+---+   +---+   +---+---+   +---+---+   +---+   +
|   |           | *   * |           |   |               |   |
+   +   +   +   +---+   +---+---+   +   +---+---+---+---+   +
|   |   |   |       | * |           |   | *   *   *   * |   |
+   +   +   +---+---+   +---+   +---+   +   +---+---+   +   +
|       |           | *   * |           | * |   | *   * |   |
+   +---+---+---+   +---+   +---+---+---+   +   +   +---+   +
|               |         *   *   *   *   * |     *   *   * |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```
