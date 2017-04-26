/**
 * rust_graphs
 *
 * This program finds paths in graphs. It reads a pair of nodes and attempts to
 * find a path between the two nodes based on the graph file. If a path does
 * not exists, the program prints "No path exists".
 *
 * Usage: cargo run <graph_file>
 *
 * Assumptions:
 *  - If the graph file only shows one direction of the edge, we assume that
 *    there exists an edge in both directions
 *
 */


use std::env;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::File;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use graph::construct_graph;
use graph::find_path;
use graph::Node;

mod graph;

fn main() {

    let graph_spec = env::args().nth(1).expect("Usage: cargo run <graph_file>");
    let file = File::open(graph_spec).expect("Could not read file");
    let graph = construct_graph(file);

    let mut lines = BufReader::new(stdin()).lines();

    while let Some(Ok(line)) = lines.next() {
        let nodes: Vec<&str> = line.split_whitespace().collect();

        match find_path(nodes, graph.clone()) {
            Some(path) => println!("{}", path),
            None => println!("No path exists"),
        }
    }
}
