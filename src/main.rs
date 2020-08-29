/**
 * The job for this program is to read in a .txt file which represents a maze 
 */
use std::path::Path;
use clap::{App, load_yaml};

mod maze_solver;
use crate::maze_solver::lib::run;
use crate::maze_solver::lib::SearchAlgo;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(matches.value_of("maze_file").unwrap());
    let algo = parse_algo(matches.value_of("algorithm").unwrap());
    run(&path, algo);
    println!("Done!");
}

fn parse_algo(algo_str : &str) -> SearchAlgo{ 
    SearchAlgo::DFS
}