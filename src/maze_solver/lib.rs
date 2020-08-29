use std::path::Path;
use std::fs::File;
use std::collections::VecDeque;

use super::maze::*;
use super::solver::*;

pub fn run (path : &Path) { 
    let file = File::open(path).unwrap();
    let maze = Maze::new(file);
    let exploration = Vec::new();
    search_maze(exploration, &maze, print_cell);
}

pub enum SearchAlgo { 
    DFS, 
    BFS
}

fn print_cell(val : &MazePosition) { 
    println!("{:?}", val )
}