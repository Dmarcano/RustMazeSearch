/**
 * The job for this program is to read in a .txt file which represents a maze 
 */
use std::path::Path;

mod maze_solver;
use crate::maze_solver::lib;


fn main() {
    let path = Path::new("public/easy.txt");
    lib::run(&path);
    println!("Done!");
}
