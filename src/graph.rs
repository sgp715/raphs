use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read, stdin};


#[derive(Clone)]
pub struct Node {

    neighbors: Vec<String>,
    parent: Option<String>,
    red: bool,

}


pub fn construct_graph<R: Read>(reader: R) -> HashMap<String, Node> {

    let mut graph: HashMap<String, Node> = HashMap::new();
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {

        let mut nodes = line.split_whitespace();

        let first_node = nodes.nth(0).unwrap().to_owned();

        let mut neighbors: Vec<String> = vec![];
        for n in nodes.skip(1) {
            neighbors.push(n.to_owned());
        }

        graph.insert(first_node, Node { neighbors: neighbors, parent: None, red: false });

    }

    graph
}


pub fn find_path(nodes: Vec<&str>, graph: HashMap<String, Node>) -> Option<String> {
    None
}

// impl Hash for Node {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }
