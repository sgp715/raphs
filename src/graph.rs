use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read, stdin};


#[derive(Clone)]
pub struct Node {

    neighbors: Vec<String>,
    parent: Option<String>,
    red: bool,

}

impl Node {
    fn set_red(&mut self, new_red: bool) {
        self.red = new_red;
    }

    fn set_parent(&mut self, new_parent: String){
        self.parent = Some(new_parent);
    }
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

        // let mut node =  Node { neighbors: neighbors, parent: None, red: false };
        graph.insert(first_node, Node { neighbors: neighbors, parent: None, red: false });

    }

    graph
}

fn traverse_graph(graph: &mut HashMap<String, Node>, s: String, t: String) {

    let mut q: Vec<String> = vec![];
    q.push(s);

    while q.len() != 0 {

        let mut new_q: Vec<String> = vec![];
        for key in &q {

            if key == &t {
                return
            }

            let node = graph.get(key).expect("Could not find key");
            for neigbor in &node.neighbors {

                let mut neighbor_node = graph.get_mut(neigbor).expect("Could not find neighbor");
                neighbor_node.set_parent(key.to_owned());
                // match neighbor_node.parent {
                //     None => neighbor_node.set_parent(key.to_owned()),
                //     _ => println!("Parent alread exists"),
                // }

                new_q.push(neigbor.to_owned());

            }

        }

        q = new_q;

    }

}

pub fn find_path(nodes: Vec<&str>, mut graph: HashMap<String, Node>) -> Option<String> {

    traverse_graph(&mut graph, nodes[0].to_owned(), nodes[1].to_owned());

    // find path by traversing backwards

    None
}

// impl Hash for Node {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }
