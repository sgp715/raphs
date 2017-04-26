use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read, stdin,BufWriter};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
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
        for n in nodes {
            neighbors.push(n.to_owned());
        }

        // let mut node =  Node { neighbors: neighbors, parent: None, red: false };
        graph.insert(first_node, Node { neighbors: neighbors, parent: None, red: false });

    }

    graph
}
#[cfg(test)]
mod construct_graph_tests {

    use super::*;
    /*
    #[test]
    fn construct_graph_test() {
        let mut buffer = BufWriter::new(File::create("foo.txt"));
        buffer.write("a b d\nb a d\nc\nd c".as_bytes());
        buffer.flush();
        let actual = construct_graph("foo.txt");
        let mut expected: HashMap<String, Node> = HashMap::new();
        expected.insert("a".to_string(), Node {neighbors: vec!["b".to_string(),"d".to_string()], parent: None, red: false});
        expected.insert("b".to_string(), Node {neighbors: vec!["a".to_string(),"d".to_string()], parent: None, red: false});
        expected.insert("c".to_string(), Node {neighbors: vec![], parent: None, red: false});
        expected.insert("d".to_string(), Node {neighbors: vec!["c".to_string()], parent: None, red: false});
        assert_eq!(actual, expected);

    }*/
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

            let mut neighbors: Vec<String> = vec![];
            {
                let node = graph.get(key).expect("Could not find key");
                for neighbor in &node.neighbors {
                    neighbors.push(neighbor.to_string());
                }
            }

            for neighbor in &neighbors {

                let neighbor_node = graph.get_mut(neighbor).expect("Could not find neighbor");
                if neighbor_node.red == false {

                    neighbor_node.set_parent(key.to_owned());
                    match neighbor_node.parent {
                        Some(_) => (),
                        None => neighbor_node.set_parent(key.to_owned()),
                    }

                }

                new_q.push(neighbor.to_owned());

            }

            let node = graph.get_mut(key).expect("Could not find key").red = true;

        }

        q = new_q;

    }

}

pub fn find_path(nodes: Vec<&str>, mut graph: HashMap<String, Node>) -> Option<String> {

    traverse_graph(&mut graph, nodes[0].to_owned(), nodes[1].to_owned());

    // look at the final nodes parents to find the path
    let mut current = nodes[1];
    let mut path: Vec<String> = vec![];
    loop {

        path.push(current.to_string());
        match graph.get(current).expect("Node does not exist").parent {
            Some(ref p) => current = &p,
            None => break,
        }

    }

    for (key, val) in &graph {
        println!("key: {}", key);
        println!("val: {:?}", val);
    }

    if path.last().expect("Could not get last element") == nodes[0] {

        let mut path_string = "".to_string();

        for n in path {
            path_string = n + " " + &path_string;
        }

        return Some(path_string)
    }

    None

}

#[cfg(test)]
mod find_path_tests {

    use super::*;

    #[test]
    fn path_itself_test() {

        let mut graph: HashMap<String, Node> = HashMap::new();
        graph.insert("a".to_string(), Node {neighbors: vec!["b".to_string(),"d".to_string()], parent: None, red: false});
        graph.insert("b".to_string(), Node {neighbors: vec!["a".to_string(),"d".to_string()], parent: None, red: false});
        graph.insert("c".to_string(), Node {neighbors: vec![], parent: None, red: false});
        graph.insert("d".to_string(), Node {neighbors: vec!["c".to_string()], parent: None, red: false});
        let actual = find_path(vec!["a","a"], graph);
        let expected = Some("a".to_string());

        assert_eq!(actual, expected);

    }

    #[test]
    fn path_single_test() {

        let mut graph: HashMap<String, Node> = HashMap::new();
        graph.insert("a".to_string(), Node {neighbors: vec!["b".to_string(),"d".to_string()], parent: None, red: false});
        graph.insert("b".to_string(), Node {neighbors: vec!["a".to_string(),"d".to_string()], parent: None, red: false});
        graph.insert("c".to_string(), Node {neighbors: vec![], parent: None, red: false});
        graph.insert("d".to_string(), Node {neighbors: vec!["c".to_string()], parent: None, red: false});
        let actual = find_path(vec!["a","d"], graph);
        let expected = Some("a d".to_string());

        assert_eq!(actual, expected);

    }

    #[test]
    fn path_multiple_test() {

        let mut graph: HashMap<String, Node> = HashMap::new();
        graph.insert("a".to_string(), Node {neighbors: vec!["b".to_string(),"d".to_string()], parent: None, red: false});
        graph.insert("b".to_string(), Node {neighbors: vec!["a".to_string(),"d".to_string()], parent: None, red: false});
        graph.insert("c".to_string(), Node {neighbors: vec![], parent: None, red: false});
        graph.insert("d".to_string(), Node {neighbors: vec!["c".to_string()], parent: None, red: false});
        let actual = find_path(vec!["a","c"], graph);
        let expected = Some("a d c".to_string());

        assert_eq!(actual, expected);

    }
}



// impl Hash for Node {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }
