use super::maze::*;
use termion::color;

/**
 * A struct that carries a reference to a Maze so that current position may be displayed along 
 * with the maze
 */
pub struct MazePositionDisplay<'a> { // we have lifetime of Struct that goes along with the reference
    maze : &'a MazePositionMaze,
    delay : u64
}

impl<'a> MazePositionDisplay<'a> { 
    pub fn new(maze :  &'a MazePositionMaze, delay : u64) -> MazePositionDisplay {
        MazePositionDisplay{maze, delay}
    }

    fn prin_maze_row(&self, row : &Vec<MazePosition>, cell : &MazePosition ) { 
        row.iter().for_each(|maze_cell| {
            match maze_cell {
                c if c == cell => { 
                    print!("{}@", color::Fg(color::Red))
                }
                c if c.is_wall => {
                    print!("{}#", color::Fg(color::Blue))
                }
                _ => {
                    print!("{}.", color::Fg(color::White))
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

        let duration = std::time::Duration::from_millis(self.delay);

        let row_iter = self.maze.iter_rows().enumerate();
        row_iter.filter(|(idx, _)| {
            match *idx { 
                _ if (*idx as u64) < start => false,
                _ if (*idx as u64) > end => false, 
                _ => true
            }
        }).for_each(|(_, row)| self.prin_maze_row(row, cell));

        std::thread::sleep(duration);
        print!("{}[2J", 27 as char);
    }
}

/**
 * Trait for displaying information based on a given cells
 */
pub trait MazeDisplay {
    fn print_maze_position(&self, cell : &MazePosition);
}