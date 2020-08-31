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


// helper for IntoIter for Maze Position
pub struct MazePositionIntoIter {
    iter: ::std::vec::IntoIter<MazePosition>
}

impl  IntoIterator for MazePositionMaze  {
    type Item =  MazePosition;
    type IntoIter = MazePositionIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MazePositionIntoIter{
            iter : self.maze.into_iter()
                .map(|v| v.into_iter())
                .flatten()
                .collect::<Vec<MazePosition>>()
                .into_iter()
        }
    }
}

impl Iterator for MazePositionIntoIter {
    type Item = MazePosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// helper struct for Maze Iterator
pub struct MazePositionIter<'a> {
    maze : &'a MazePositionMaze,
    cur_row_idx : u64,
    cur_col_idx : u64, 
    // iter: ::std::vec::IntoIter<&'a &'a MazePosition>
}

impl MazePositionMaze { 
    pub fn iter<'a> (&'a self) -> MazePositionIter<'a>{
        let mut val = self.maze.iter() ;
            MazePositionIter{
                maze : &self,
                cur_col_idx : 0, 
                cur_row_idx : 0
            }
     }
}

impl<'a> Iterator for MazePositionIter<'a> {
    type Item =  &'a MazePosition;

    fn next(&mut self)-> Option<Self::Item> { 
        match self.cur_row_idx {
            y if y >= self.maze.height => {
                // have gone down all rows and have exhausted iter
                return None
            }
            y => {
                // go down cols until reach end then reset
                match self.cur_col_idx { 
                    x if x >= self.maze.width => {
                        // if exhausted a row, reset col, go to next row 
                        self.cur_col_idx = 0; 
                        self.cur_row_idx += 1;
                        return self.next() // recurse to next row
                    }
                    x => {
                        self.cur_col_idx += 1;
                        return Some(self.maze.get(x, y))
                    }
                }
            }
        } 
    }
}


