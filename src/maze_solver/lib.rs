use std::path::Path;
use std::fs::File;

use super::maze::*;
use super::solver::*;

pub fn run (path : &Path) { 
    let file = File::open(path).unwrap();
    let maze = Maze::new(file);
    let exploration = Vec::new();
    search_maze(exploration, &maze, print_cell);
}

fn print_cell(val : &MazePosition) { 
    println!("{:?}", val )
}