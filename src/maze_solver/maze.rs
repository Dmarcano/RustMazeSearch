use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MazePosition { 
    x : u64,
    y: u64,   
    is_wall : bool 
}

#[derive(Debug, PartialEq, Eq)]
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
    pub fn new(file : File) -> Maze{ 
        let reader = BufReader::new(file); 
        let mut width : u64 = 0;  
        let mut height : u64 = 0;  
        let mut maze : Vec<Vec<MazePosition>> = Vec::new();

        for (row, line) in reader.lines().map(|l| l.unwrap()).enumerate() { 
            match row { 
                0 => {
                    let dimensions_strs = line.split_whitespace().collect::<Vec<&str>>();
                    width = dimensions_strs.get(0).unwrap().parse().unwrap();
                    height = dimensions_strs.get(1).unwrap().parse().unwrap();
                }
                _ => {
                    // iter the line for symbols
                    maze.push(parse_maze_row(line, (row - 1) as u64 , width));
                }
            }
        }
        Maze {maze, width, height}
    }

    pub fn get(&self, x : u64, y : u64) -> &MazePosition{ 
        unimplemented!("Not implemented getting a maze cell from maze!")
    }

    pub fn get_neighbors(&self, x : u64, y : u64) -> Vec<MazePosition>{ 
        unimplemented!();
    }
}


fn parse_maze_row(line : String, row_idx : u64, width : u64) -> Vec<MazePosition>{ 
    let mut row : Vec<MazePosition>=  Vec::with_capacity(width as usize);
    for (col_idx, chr) in line.chars().enumerate() { 
        let mut is_wall = false;
        match chr { 
            '.' => {}
            '#' => {
                is_wall = true;
            }
            _ => {
                println!("unknown character in maze file! row: {}, col {} Assuming it is not a wall...", row_idx, col_idx as u64)
            }
        }
        row.push( MazePosition::new(col_idx as u64, row_idx, is_wall )  )
    }
    row
}


#[cfg(test)] 
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn  parse_maze_row_usual() {
        let row_str = String::from("..#.#");
        let row_idx = 0;
        let expected = vec![
            MazePosition::new(0, 0, false),
            MazePosition::new(1, 0, false),
            MazePosition::new(2, 0, true),
            MazePosition::new(3, 0, false),
            MazePosition::new(4, 0, true),
        ];
        let actual = parse_maze_row(row_str, row_idx, 5);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_maze_test() {
        let maze_row = vec![MazePosition::new(0, 0, false),MazePosition::new(1, 0, false),MazePosition::new(2, 0, false)];
        let expected = Maze {height : 1, width : 3, maze : vec![maze_row]};

        let path = Path::new("public/easy.txt");
        let file = File::open(path).unwrap();
        let actual = Maze::new(file);
        assert_eq!(expected, actual)
    }
}
