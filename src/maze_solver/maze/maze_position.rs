use super::*;

impl MazePosition {
    fn new(x : u64, y : u64, is_wall :bool, is_goal : bool) -> MazePosition {
        MazePosition { x, y, is_wall, is_goal}
    }
}

impl MazePositionMaze {
    /**
     * Creates a new maze from a file that corresponds to a specific layout
     *
     * <li>
     *  #
     *
     * </li>
     */
    pub fn new(file : File) -> MazePositionMaze {
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
        MazePositionMaze {maze, width, height}
    }

    /**
     *
     */
    pub fn get(&self, x : u64, y : u64) -> &MazePosition{
        self.maze.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    pub fn get_neighbors(&self, x : u64, y : u64) -> Vec<&MazePosition>{
        let mut neighbors = Vec::new();

        // top
        if y > 0 && self.valid_index(x, y -1) {
            neighbors.push(self.get(x, y-1));
        }
        //right
        if self.valid_index(x + 1, y) {
            neighbors.push(self.get(x+1, y));
        }
        // bottom
        if self.valid_index(x , y + 1) {
            neighbors.push(self.get(x, y + 1));
        }
        //left
        if x > 0 && self.valid_index(x - 1, y) {
            neighbors.push(self.get(x - 1, y));
        }
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
        row.push( MazePosition::new(col_idx as u64, row_idx, is_wall, false )  )
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
            MazePosition::new(0, 0, false, false),
            MazePosition::new(1, 0, false, false),
            MazePosition::new(2, 0, true, false),
            MazePosition::new(3, 0, false, false),
            MazePosition::new(4, 0, true, false),
        ];
        let actual = parse_maze_row(row_str, row_idx, 5);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_maze_test() {
        let maze_row = vec![MazePosition::new(0, 0, false, false),MazePosition::new(1, 0, false, false),MazePosition::new(2, 0, false, false)];
        let expected = MazePositionMaze {height : 1, width : 3, maze : vec![maze_row]};

        let path = Path::new("public/easy.txt");
        let file = File::open(path).unwrap();
        let actual = MazePositionMaze::new(file);
        assert_eq!(expected, actual)
    }

    #[test]
    fn get_row_maze() {
        let path = Path::new("public/easy.txt");
        let file = File::open(path).unwrap();
        let actual_maze = MazePositionMaze::new(file);

        assert_eq!(actual_maze.get(0, 0), &MazePosition{x : 0, y : 0 , is_wall : false, is_goal : false});
        assert_eq!(actual_maze.get(1, 0), &MazePosition{x : 1, y : 0 , is_wall : false, is_goal : false});
        assert_eq!(actual_maze.get(2, 0), &MazePosition{x : 2, y : 0 , is_wall : false, is_goal : false});

    }

    #[test]
    fn get_matrix_maze() {
        let path = Path::new("public/example1.txt");
        let file = File::open(path).unwrap();
        let actual_maze = MazePositionMaze::new(file);

        // left column
        assert_eq!(actual_maze.get(0, 0), &MazePosition{x : 0, y : 0 , is_wall : false, is_goal : false});
        assert_eq!(actual_maze.get(0, 1), &MazePosition{x : 0, y : 1 , is_wall : false, is_goal : false});
        assert_eq!(actual_maze.get(0, 2), &MazePosition{x : 0, y : 2 , is_wall : false, is_goal : false});
        // right coloumn
        assert_eq!(actual_maze.get(4, 0), &MazePosition{x : 4, y : 0 , is_wall : false, is_goal : false});
        assert_eq!(actual_maze.get(4, 1), &MazePosition{x : 4, y : 1 , is_wall : false, is_goal : false});
        assert_eq!(actual_maze.get(4, 2), &MazePosition{x : 4, y : 2 , is_wall : false, is_goal : false});
        // middle column (column 2) of maze
        assert_eq!(actual_maze.get(1, 0), &MazePosition{x : 1, y : 0 , is_wall : true, is_goal : false});
        assert_eq!(actual_maze.get(1, 1), &MazePosition{x : 1, y : 1 , is_wall : true, is_goal : false});
        assert_eq!(actual_maze.get(1, 2), &MazePosition{x : 1, y : 2, is_wall : false, is_goal : false});
    }

    #[test]
    fn get_neighbors() {
        let path = Path::new("public/example1.txt");
        let file = File::open(path).unwrap();
        let actual_maze = MazePositionMaze::new(file);

        // get neighbors of middle of maze
        let middle_neighbors = actual_maze.get_neighbors(1, 1);
        // let middle_neighbors = actual_maze.get_neighbors(0, 0);
        let expected_neighbors = vec! {
            &MazePosition{x : 1, y :0, is_wall: true, is_goal : false}, // top
            &MazePosition{x : 2, y :1, is_wall: false, is_goal : false}, // right
            &MazePosition{x : 1, y :2, is_wall: false, is_goal : false}, // bottom
            &MazePosition{x : 0, y :1, is_wall: false, is_goal : false}, // left
        };
        assert_eq!(expected_neighbors, middle_neighbors);
    }

    #[test]
    fn get_neighbors_edges() {
        let path = Path::new("public/example1.txt");
        let file = File::open(path).unwrap();
        let actual_maze = MazePositionMaze::new(file);

        let top_left = actual_maze.get_neighbors(0, 0);
        let bottom_left = actual_maze.get_neighbors(0, 2);
        let top_right = actual_maze.get_neighbors(4, 0);
        let bottom_right = actual_maze.get_neighbors(4, 2);

        let expected_top_left = vec! {
            &MazePosition{x : 1, y :0, is_wall: true, is_goal : false}, // right
            &MazePosition{x : 0, y :1, is_wall: false, is_goal : false}, // botom
        };

        let expected_bottom_left = vec! {
            &MazePosition{x : 0, y :1, is_wall: false, is_goal : false}, // top
            &MazePosition{x : 1, y :2, is_wall: false, is_goal : false}, // right
        };

        let expected_top_right = vec! {
            &MazePosition{x : 4, y :1, is_wall: false, is_goal : false}, // bottom
            &MazePosition{x : 3, y :0, is_wall: true, is_goal : false}, // left
        };

        let expected_bottom_right = vec! {
            &MazePosition{x : 4, y :1, is_wall: false, is_goal : false}, // top
            &MazePosition{x : 3, y :2, is_wall: true, is_goal : false}, // left
        };

        assert_eq!(expected_top_left, top_left);
        assert_eq!(expected_bottom_left, bottom_left);
        assert_eq!(expected_top_right, top_right);
        assert_eq!(expected_bottom_right, bottom_right);
    }
}
