/**
 * The job for this program is to read in a .txt file which represents a maze 
 */
use std::path::Path;
use clap::{App, load_yaml};
use std::process;

mod maze_solver;
use crate::maze_solver::lib::run;
use crate::maze_solver::lib::SearchAlgo;


const SUPPORTED_DFS : &[&str]= &["dfs", "DFS"];
const SUPPORTED_BFS : &[&str]= &["bfs", "BFS"];

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(matches.value_of("maze_file").unwrap());
    let algo = parse_algo(matches.value_of("algorithm").unwrap())
    .unwrap_or_else(|er| {
        println!("Problem parsing arguments: {}", er);
        process::exit(1)});

    run(&path, algo);
    println!("Done!");
}

fn parse_algo(algo_str : &str) -> Result<SearchAlgo, String>{ 

    if SUPPORTED_DFS.contains(&algo_str){
        return Ok(SearchAlgo::DFS)
    };
    if SUPPORTED_BFS.contains(&algo_str) { 
        return Ok(SearchAlgo::BFS)
    };
    let message = format!("Arguments for Algorithm must be one of {:?} or {:?}", SUPPORTED_DFS, SUPPORTED_BFS);
    return Err(message) ;
}