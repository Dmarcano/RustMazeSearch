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

    fn prin_maze_row(&self, row : &Vec<MazePosition>, cell : &MazePosition ) { 
        row.iter().for_each(|maze_cell| {
            match maze_cell {
                c if c == cell => { 
                    print!("@")
                }
                c if c.is_wall => {
                    print!("#")
                }
                _ => {
                    print!(".")
                }
            };
        });
        print!("\n")
    }  
}

impl<'a> MazeDisplay for MazePositionDisplay<'a> {
    fn print_maze_position(&self, cell : &MazePosition) { 

        println!("=============");
        let row_iter = self.maze.iter_rows() ;  
        row_iter.for_each(|maze_row| 
            self.prin_maze_row(maze_row, cell)
        );
    }
}

/**
 * Trait for displaying information based on a given cells
 */
pub trait MazeDisplay {
    fn print_maze_position(&self, cell : &MazePosition);
}