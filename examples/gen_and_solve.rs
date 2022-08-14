use maze_traits::generators::DepthFirstSearch;
use maze_traits::mazes::SquareGrid;
use maze_traits::solvers::BreadthFirstSearch;
use maze_traits::MazeGenerator;
//use maze_traits::MazeMut;
use maze_traits::MazeSolver;

fn main() {
    let rows = 15;
    let cols = 15;

    let mut m = SquareGrid::new(rows, cols);

    DepthFirstSearch::new().generate(&mut m, &(0, 0), &(rows - 1, cols - 1));

    let keys = BreadthFirstSearch::new().solve(&m, &(0, 0), &(rows - 1, cols - 1));

    println!("{}", m.to_string_with_keys(&keys));
}
