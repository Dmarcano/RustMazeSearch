use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub mod maze_position;

/**
 * A representation for a cell within a Maze to search
 */
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct MazePosition {
    pub x : u64,
    pub y: u64,
    pub is_wall : bool,
    pub is_goal : bool
}

/**
 * A representation for a Maze to search for
 */
#[derive(Debug, PartialEq, Eq)]
pub struct MazePositionMaze {
    maze : Vec<Vec<MazePosition>>,
    pub width : u64,
    pub height : u64
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


// ===== MAZE POSITION INTO-ITER
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

// ===== MAZE POSITION ROW ITER
pub struct MazeRowIter<'a> { 
    iter : ::std::slice::Iter<'a, Vec<MazePosition>>
}

impl <'a> Iterator for MazeRowIter<'a> { 
    type Item = &'a Vec<MazePosition>;

    fn next(&mut self) -> Option<Self::Item>{ 
        self.iter.next()
    }
}

impl MazePositionMaze { 

    pub fn iter_rows(&self) -> MazeRowIter{ 
        MazeRowIter {
             iter : self.maze.iter()
        }
    }
}


// ===== MAZE POSITION ITER
pub struct MazePositionIter<'a> {
    maze : &'a MazePositionMaze,
    cur_row_idx : u64,
    cur_col_idx : u64, 
    // iter: ::std::vec::IntoIter<&'a &'a MazePosition>
}

impl MazePositionMaze { 
    pub fn iter<'a> (&'a self) -> MazePositionIter<'a>{
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

// ===== MAZE POSITION 


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn into_iter_test() {
        let num_rows = 2;
        let num_cols = 3;
        let rows = 0..num_rows;

        let maze_vec = rows.map(
            |r| {
                create_maze_row(r, num_cols)
            }
        ).collect::<Vec<Vec<MazePosition>>>();

        let maze = MazePositionMaze { maze : maze_vec, width : num_cols, height : num_rows};
        
        let mut iter = maze.into_iter();
        assert_eq!(iter.next(), Some(MazePosition{x : 0, y: 0 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(MazePosition{x : 1, y: 0 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(MazePosition{x : 2, y: 0 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(MazePosition{x : 0, y: 1 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(MazePosition{x : 1, y: 1 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(MazePosition{x : 2, y: 1 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn iter_test() {
        let num_rows = 2;
        let num_cols = 3;
        let rows = 0..num_rows;

        let maze_vec = rows.map(
            |r| {
                create_maze_row(r, num_cols)
            }
        ).collect::<Vec<Vec<MazePosition>>>();

        let maze = MazePositionMaze { maze : maze_vec, width : num_cols, height : num_rows};
        
        let mut iter = maze.iter();
        assert_eq!(iter.next(), Some(&MazePosition{x : 0, y: 0 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(&MazePosition{x : 1, y: 0 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(&MazePosition{x : 2, y: 0 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(&MazePosition{x : 0, y: 1 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(&MazePosition{x : 1, y: 1 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), Some(&MazePosition{x : 2, y: 1 , is_goal : false, is_wall: false}));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_rows_test() {
        let num_rows = 2;
        let num_cols = 3;
        let rows = 0..num_rows;

        let maze_vec = rows.map(
            |r| {
                create_maze_row(r, num_cols)
            }
        ).collect::<Vec<Vec<MazePosition>>>();

        let maze = MazePositionMaze { maze : maze_vec, width : num_cols, height : num_rows};
        let mut row_iter = maze.iter_rows();
        assert_eq!(Some(&vec![
            MazePosition{x : 0, y: 0 , is_goal : false, is_wall: false},
            MazePosition{x : 1, y: 0 , is_goal : false, is_wall: false},
            MazePosition{x : 2, y: 0 , is_goal : false, is_wall: false}]), row_iter.next());

        assert_eq!(Some(&vec![
            MazePosition{x : 0, y: 1 , is_goal : false, is_wall: false},
            MazePosition{x : 1, y: 1 , is_goal : false, is_wall: false},
            MazePosition{x : 2, y: 1 , is_goal : false, is_wall: false}]), row_iter.next());

        assert_eq!(None, row_iter.next())
    
    }

    fn create_maze_row(row_idx : u64, num_cols : u64) -> Vec<MazePosition> {
        let cols = 0..num_cols;
        cols.map( 
            |c| MazePosition{x : c, y: row_idx , is_goal : false, is_wall: false}
        ).collect::<Vec<MazePosition>>()
    }
}
