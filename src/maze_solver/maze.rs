
pub struct MazePosition { 
    x : u64,
    y: u64, 
    is_wall : bool 
}

pub struct Maze { 
    maze : Vec<Vec<MazePosition>>,
    width : u64, 
    height : u64
}

impl MazePosition { 
    fn new(x : u64, y : u64, is_wall :bool) -> MazePosition { 
        MazePosition { x, y, is_wall}
    }
}

impl Maze {
    fn new(height : u64, width : u64) -> Maze{ 
        unimplemented!("Not imlemented making a maze!")
    }

    fn get(&self, x : u64, y : u64) -> &MazePosition{ 
        unimplemented!("Not implemented getting a maze cell from maze!")
    }
}