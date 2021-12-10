use maze_traits::generators::DepthFirstSearch;
use maze_traits::mazes::SquareGrid;
use maze_traits::solvers::BreadthFirstSearch;
use maze_traits::MazeGenerator;
//use maze_traits::MazeMut;
use maze_traits::MazeSolver;

fn main() {
    let rows = 1000;
    let cols = 1000;

    let mut m = SquareGrid::new(rows, cols);

    /*for i in 0..3 {
        m.remove_wall(&(i, 0), &(i + 1, 0));
    }
    for i in 0..3 {
        m.remove_wall(&(3, i), &(3, i + 1));
    }
    m.remove_wall(&(1, 0), &(1, 1));*/

    DepthFirstSearch::new().generate(&mut m, &(0, 0), &(rows - 1, cols - 1));

    //println!("{}", m);

    let keys = BreadthFirstSearch::new().solve(&m, &(0, 0), &(rows - 1, cols - 1));

    println!("{}", m.to_string_with_keys(&keys));
}
