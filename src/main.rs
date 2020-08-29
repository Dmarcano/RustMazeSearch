/**
 * The job for this program is to read in a .txt file which represents a maze 
 */
use std::path::Path;

mod maze_solver;
use crate::maze_solver::lib;
use clap::{App, load_yaml};



fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(matches.value_of("maze_file").unwrap());
    let algo = matches.value_of("algorithm").unwrap();
    lib::run(&path);
    println!("Done!");
}
