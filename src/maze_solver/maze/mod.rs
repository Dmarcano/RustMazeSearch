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
    iter : ::std::slice::Iter<'a, &'a MazePosition>
    // iter: ::std::vec::IntoIter<&'a &'a MazePosition>


}

impl MazePositionMaze { 
    pub fn iter<'a> (&'a self) -> MazePositionIter<'a>{

        // let val = self.maze.iter() 
        //     .flat_map(|v| v.iter())
        //     .collect::<Vec<&'a MazePosition>>()
        //     .iter();
        let mut val : Vec<&MazePosition> = Vec::new();
        for vec in &self.maze { 
            for cell in vec {
                val.push(cell);
            }
        }

            MazePositionIter{
                iter : val.iter()
            }

        // unimplemented!()
        // MazePositionIter { 
        //     iter: self.maze.iter()
        //     .flat_map(|v| v.iter())
        //     .collect::<Vec<&'a  MazePosition>>()
        //     .iter()
        //     // .flatten()
        //     // .collect::<Vec<MazePosition>>()
            
        // }
     }
}

// impl<'a> IntoIterator for &'a MazePositionMaze {

//     type Item = &'a &'a MazePosition;
//     type IntoIter =  MazePositionIter<'a>;

//     fn into_iter(self) -> Self::IntoIter { 
//         unimplemented!()
//     }

// }

impl<'a> Iterator for MazePositionIter<'a> {
    type Item = &'a &'a MazePosition;

    fn next(&mut self)-> Option<Self::Item> { 
        self.iter.next()
    }
}


