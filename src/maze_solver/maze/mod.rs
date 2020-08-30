use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub mod maze_position;

/**
 * A representation for a cell within a Maze to search
 */
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct MazePosition {
    x : u64,
    y: u64,
    pub is_wall : bool,
    pub is_goal : bool
}

/**
 * A representation for a Maze to search for
 */
#[derive(Debug, PartialEq, Eq)]
pub struct MazePositionMaze {
    maze : Vec<Vec<MazePosition>>,
    width : u64,
    height : u64
}


/**
* A trait representing a Maze for which one can get read only values that represent a Maze
* which has a start and an end
*/
pub trait Maze<T> {
    fn get_start(&self) -> &T;
    fn get_neighbors(&self, val : &T) -> Vec<&T>;
    fn get_end(&self) -> &T;
}

impl Maze<MazePosition> for MazePositionMaze {
    fn get_start(&self) -> &MazePosition {
        self.get(0,0)
    }

    fn get_neighbors(&self, val: &MazePosition) -> Vec<&MazePosition> {
        self.get_neighbors(val.x, val.y)
    }

    fn get_end(&self) -> &MazePosition {
        unimplemented!()
    }
}



// helper for IntoIter
struct MazeIntoIter {

}

