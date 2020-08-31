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
        let start = match cell.y {
            y if y as i128 - 2 <= 0 =>  0, 
            y => y - 2
        };

        let end = match cell.y {
            y if y + 2 >= self.maze.height =>  self.maze.height, 
            y => y + 2
        };

        let mut idx = 0;
        println!("=============");
        let row_iter = self.maze.iter_rows() ;  
        row_iter.filter(|_| {
            match idx {
                _ if idx < start => {
                    idx += 1;
                    return false }
                _ if idx > end => {
                    idx += 1;
                    return false
                }
                _ => return true 
            }
        }).for_each(|maze_row| self.prin_maze_row(maze_row, cell))
    }
}

/**
 * Trait for displaying information based on a given cells
 */
pub trait MazeDisplay {
    fn print_maze_position(&self, cell : &MazePosition);
}