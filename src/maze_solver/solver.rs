use super::maze::*;
use std::collections::HashSet;


/**
 * The Exploration trait is responsible for exploring a Maze Interpretation
 */
pub trait Exploration<T> {
    fn give(&mut self, val : T);

    fn take(&mut self) -> Option<T>;
}

impl<T> Exploration<T> for Vec<T> { 

    fn give(&mut self, val : T ){
        self.push(val);
    }

    fn take(&mut self) -> Option<T> {
        self.pop()
    }

}



pub fn search_maze<'a, T>( mut exploration :  T , maze : &'a Maze, func : fn(&MazePosition) ) where T : Exploration<&'a MazePosition> { 
    let mut seen_cells : HashSet<&MazePosition> = HashSet::new();
    let start = maze.get(0, 0);
    seen_cells.insert(start);
    exploration.give(start);

    while let Some(curr_cell) = exploration.take() { 
        func(curr_cell); // perform operation on maze cell
        seen_cells.insert(curr_cell);
        maze.get_neighbors(curr_cell.x, curr_cell.y).iter()
        .filter(|n| !seen_cells.contains(**n) && !n.is_wall)
        .for_each(|neighbor| {
            exploration.give(neighbor);
       });
    }
}