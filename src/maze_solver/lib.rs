use std::path::Path;
use std::fs::File;
use std::collections::VecDeque;

use super::maze::*;
use super::solver::*;

pub fn run (path : &Path, algo : SearchAlgo) { 
    let file = File::open(path).unwrap();
    let maze = Maze::new(file);

    match algo {
        SearchAlgo::BFS => {search_maze(VecDeque::new(), &maze, print_cell);}
        SearchAlgo::DFS => {search_maze(Vec::new(), &maze, print_cell);}
    };
}


pub enum SearchAlgo { 
    DFS, 
    BFS
}

fn print_cell(val : &MazePosition) { 
    println!("{:?}", val )
}