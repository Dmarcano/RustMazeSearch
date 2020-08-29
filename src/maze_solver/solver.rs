use super::maze::*;
use std::collections::HashSet;


/**
 * The Exploration trait is responsible for exploring a Maze Interpretation
 */
pub trait Exploration {
    fn give(&mut self, val : &MazePosition );

    fn take(&mut self) -> Option<&MazePosition>;
}


pub fn solve_dfs<'a,T>( mut exploration :  Vec< &'a MazePosition> , maze : &'a Maze ) where T : Exploration { 
    let mut seen_cells : HashSet<&MazePosition> = HashSet::new();
    // let mut exploration = Vec::new();
    let start = maze.get(0, 0);
    seen_cells.insert(start);
    exploration.push(start);

    while let Some(curr_cell) = exploration.pop() { 
       seen_cells.insert(curr_cell);
       for neighbor in maze.get_neighbors(curr_cell.x, curr_cell.y){
            // perform function
            println!("{:?}",neighbor);
            exploration.push(neighbor);
       }
    }
}