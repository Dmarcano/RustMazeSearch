# RustMazeSearch 

A small CLI program that solves Mazes built from given text files. Built for the purpose of learning Rust programming.

* Opens a text file that represents in a maze as open cells or wall cells

* Searches the maze using either depth-first-search or breadth-first-search

* Performs a function on each maze cell as the search goes on (in this case its to display the current position in the search)

## Short Maze Demo
![small demo](https://github.com/Dmarcano/RustMazeSearch/blob/master/public/demos/rust%20maze%20search%20demo%20small.2020-09-06%2014_27_50.gif)


## Long Maze Demo
![large demo](https://github.com/Dmarcano/RustMazeSearch/blob/master/public/demos/rust%20maze%20search%20demo%20large.2020-09-06%2014_27_20.gif)

## Running 

* clone the repo 
* use either ```./run.sh``` or ``` cargo run -- -a DFS -m public/example1.txt``` to run the program with valid arguments
* NOTE Only those with a Unix shell can properly run this program due to adding color text to the terminal. This functionality has yet to be successfully ported over to Windows machines.

## TODO: 
* Write the text solution for a Maze
* Generate Mazes

