use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read, stdin};


#[derive(PartialEq, Eq)]
pub struct Node {
    name: String,
    neighbors: HashSet<String>,
}


pub fn construct_graph<R: Read>(reader: R) -> HashSet<Node> {

    let mut graph: HashSet<Node> = HashSet::new();
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        let mut nodes = line.split_whitespace();

        let mut neighbors = HashSet::new();
        let first_node = nodes.nth(0).unwrap().to_owned();

        for n in nodes.skip(1) {
            neighbors.insert(n.to_owned());
        }

        graph.insert(Node { name: first_node, neighbors: neighbors });
    }

    graph
}


pub fn find_path(nodes: Vec<&str>, graph: &HashSet<Node>) -> Option<String> {
    None
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
