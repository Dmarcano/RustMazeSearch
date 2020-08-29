use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MazePosition { 
    pub x : u64,
    pub y: u64,   
    pub is_wall : bool 
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
        self.maze.get(y as usize).unwrap().get(x as usize).unwrap()
    }


    pub fn get_neighbors(&self, x : u64, y : u64) -> Vec<&MazePosition>{ 
        let mut neighbors = Vec::new();

        // TODO handle overflow case 
        for row_idx in y-1..y+2 {
            for col_idx in x-1..x+2 {
                // TODO get only top right bottom left
                if self.valid_index(col_idx, row_idx) && (row_idx != y || col_idx != x) {
                    neighbors.push(self.get(col_idx, row_idx));
                }
            }
        };
        neighbors
    }

    fn valid_index(&self, x : u64, y : u64) -> bool{
        let num_rows = self.maze.len() as u64;
        let num_cols = self.maze.get(0).unwrap().len() as u64;
        match (x,y) { 
            (_, y) if y >= num_rows => {false}
            (x, _) if x >= num_cols => {false}
            (_, _) => {true}
        }
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

    #[test]
    fn get_row_maze() {
        let path = Path::new("public/easy.txt");
        let file = File::open(path).unwrap();
        let actual_maze = Maze::new(file);

        assert_eq!(actual_maze.get(0, 0), &MazePosition{x : 0, y : 0 , is_wall : false});
        assert_eq!(actual_maze.get(1, 0), &MazePosition{x : 1, y : 0 , is_wall : false});
        assert_eq!(actual_maze.get(2, 0), &MazePosition{x : 2, y : 0 , is_wall : false});

    }

    #[test]
    fn get_matrix_maze() { 
        let path = Path::new("public/example1.txt");
        let file = File::open(path).unwrap();
        let actual_maze = Maze::new(file);

        // left column
        assert_eq!(actual_maze.get(0, 0), &MazePosition{x : 0, y : 0 , is_wall : false});
        assert_eq!(actual_maze.get(0, 1), &MazePosition{x : 0, y : 1 , is_wall : false});
        assert_eq!(actual_maze.get(0, 2), &MazePosition{x : 0, y : 2 , is_wall : false});
        // right coloumn 
        assert_eq!(actual_maze.get(4, 0), &MazePosition{x : 4, y : 0 , is_wall : false});
        assert_eq!(actual_maze.get(4, 1), &MazePosition{x : 4, y : 1 , is_wall : false});
        assert_eq!(actual_maze.get(4, 2), &MazePosition{x : 4, y : 2 , is_wall : false});
        // middle column (column 2) of maze 
        assert_eq!(actual_maze.get(1, 0), &MazePosition{x : 1, y : 0 , is_wall : true});
        assert_eq!(actual_maze.get(1, 1), &MazePosition{x : 1, y : 1 , is_wall : true});
        assert_eq!(actual_maze.get(1, 2), &MazePosition{x : 1, y : 2, is_wall : false});
    }

    #[test]
    fn get_neighbors() {
        let path = Path::new("public/example1.txt");
        let file = File::open(path).unwrap();
        let actual_maze = Maze::new(file);

        // get neighbors of middle of maze
        // let middle_neighbors = actual_maze.get_neighbors(1, 1);
        let middle_neighbors = actual_maze.get_neighbors(0, 0);

        // middle_neighbors.iter().for_each(|n|  println!("{:?}", n) );
        for n in middle_neighbors {
            println!("{:?}", n);
        }

        assert_eq!(1+1, 2)
    }
}
