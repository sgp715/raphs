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

#[derive(PartialEq, Eq)]
struct Node {
    name: String,
    neighbors: HashSet<String>,
}

fn main() {

    let graph_spec = env::args().nth(1).expect("Usage: cargo run <graph_file>");
    let file = File::open(graph_spec).expect("Could not read file");
    let graph = construct_graph(file);
    

    let mut lines = BufReader::new(stdin()).lines();

    while let Some(Ok(line)) = lines.next() {
        let nodes: Vec<&str> = line.split_whitespace().collect();
        
        match find_path(nodes, &graph) {
            Some(path) => println!("{}", path),
            None => println!("No path exists"),
        }
    }
}

fn construct_graph<R: Read>(reader: R) -> HashSet<Node> {
    let mut graph: HashSet<Node> = HashSet::new();
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        let mut nodes = line.split_whitespace();

        let mut neighbors = HashSet::new();
        let first_node = nodes.nth(0).unwrap().to_owned();

        for n in nodes.skip(1) {
            neighbors.insert(n.to_owned());
        }
        
        graph.insert(Node { name: first_node, neighbors: neighbors});
    }

    graph
}


fn find_path(nodes: Vec<&str>, graph: &HashSet<Node>) -> Option<String> {
    None
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
