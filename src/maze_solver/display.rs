use super::maze::*;

/**
 * A struct that carries a reference to a Maze so that current position may be displayed along 
 * with the maze
 */
pub struct MazePositionDisplay<'a> { // we have lifetime of Struct that goes along with the reference
    maze : &'a MazePositionMaze,
    delay : u32
}

impl<'a> MazePositionDisplay<'a> { 
    pub fn new(maze :  &'a MazePositionMaze, delay : u32) -> MazePositionDisplay {
        MazePositionDisplay{maze, delay}
    }
}

impl<'a> MazeDisplay for MazePositionDisplay<'a> {
    fn print_maze_position(&self, cell : &MazePosition) { 
        let row_iter = self.maze.iter_rows() ;
        
        unimplemented!("Have to implement Maze display for Maze position")
    }
}

/**
 * Trait for displaying information based on a given cells
 */
pub trait MazeDisplay {
    fn print_maze_position(&self, cell : &MazePosition);
}