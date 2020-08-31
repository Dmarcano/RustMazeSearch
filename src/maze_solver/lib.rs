use std::path::Path;
use std::fs::File;
use std::collections::VecDeque;

use super::maze::*;
use super::solver::*;
use super::display::*;

/**
 * Runs the program for searching a maze with a type of search algorthtm
 */
pub fn run (path : &Path, algo : SearchAlgo) { 
    let file = File::open(path).unwrap();
    let maze = MazePositionMaze::new(file);

    let display = MazePositionDisplay::new(&maze, 10);

    match algo {
        SearchAlgo::BFS => {search_maze(VecDeque::new(), &maze, display);}
        SearchAlgo::DFS => {search_maze(Vec::new(), &maze, display);}
    };
}

/**
 * An enum of the currently supported maze search Algorithms
 */
pub enum SearchAlgo { 
    /**
     * Depth first search: Searches deep into a branch of a path until a dead end is found
     * Then repeats for another path until all paths are exhausted
    */ 
    DFS, 
    // Breadth first search: Searches each branch of all paths equally deep until all paths are exhauset
    BFS
}