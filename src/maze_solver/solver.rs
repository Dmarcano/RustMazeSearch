use super::maze::*;
use std::collections::HashSet;
use std::collections::VecDeque;

/**
 * Trait that corresponds to a structure which can search a Maze over a generic Type T
 */
pub trait Exploration<T> {
    fn give(&mut self, val : T);

    fn take(&mut self) -> Option<T>;
}

// uses a Vec as a Stack for exploring a Maze of any type T
impl<T> Exploration<T> for Vec<T> { 

    fn give(&mut self, val : T ){
        self.push(val);
    }

    fn take(&mut self) -> Option<T> {
        self.pop()
    }

}

// uses a VecDeque as a Queue for exploring a Maze of type T
impl<T> Exploration<T> for VecDeque<T> { 

    fn give(&mut self, val : T ){
        self.push_back(val);
    }

    fn take(&mut self) -> Option<T> {
        self.pop_front()
    }
}

// Search a Maze using a structure which implements Maze exploration, performing a function on every Maze cell found
pub fn search_maze<'a, T, U>(mut exploration :  T, maze : &'a U, func : fn(&MazePosition) )
    where T : Exploration<&'a MazePosition>,
    U : Maze<MazePosition> {
    let mut seen_cells : HashSet<&MazePosition> = HashSet::new();
    let start = maze.get_start();
    seen_cells.insert(start);
    exploration.give(start);

    while let Some(curr_cell) = exploration.take() { 
        func(curr_cell); // perform operation on maze cell
        seen_cells.insert(curr_cell);
        maze.get_neighbors(curr_cell).iter()
        .filter(|n| !seen_cells.contains(**n) && !n.is_wall)
        .for_each(|neighbor| {
            exploration.give(neighbor);
       });
    }
}