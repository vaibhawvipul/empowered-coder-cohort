use std::collections::HashMap;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    state: Arc<Mutex<HashMap<String, String>>>,
    peers: Vec<usize>,
}

impl Node {
    fn new(id: usize, peers: Vec<usize>) -> Self {
        Node {
            id,
            state: Arc::new(Mutex::new(HashMap::new())),
            peers,
        }
    }

    fn update_state(&self, key: String, value: String) {
        let mut state = self.state.lock().unwrap();
        state.insert(key, value);
    }

    fn get_state(&self) -> HashMap<String, String> {
        self.state.lock().unwrap().clone()
    }

    fn gossip(&self) {
        let mut rng = rand::thread_rng();
        // talk to peers
        let peer = self.peers.choose(&mut rng).unwrap();
        println!("Node {} is gossiping with Node {}", self.id, peer);
    }
}

fn start_gossiping(nodes: Arc<Mutex<Vec<Node>>>, iterations: usize, interval: Duration) {
    for _ in 0..iterations {
        let nodes = nodes.clone();
        thread::spawn(move || {
            let nodes = nodes.lock().unwrap();
            for node in nodes.iter() {
                node.gossip();
            }
        });
        thread::sleep(interval);
    }
}



fn main() {
    println!("Starting server!");

    let nodes = Arc::new(Mutex::new(vec![
        Node::new(1, vec![2, 3]),
        Node::new(2, vec![1, 3]),
        Node::new(3, vec![1, 2]),
    ]));

    {
        let mut nodes = nodes.lock().unwrap();
        nodes[0].update_state("key1".to_string(), "value1".to_string());
    }

    start_gossiping(Arc::clone(&nodes), 10, Duration::from_secs(1));

    thread::sleep(Duration::from_secs(12)); // Wait for gossiping to complete

    let nodes = nodes.lock().unwrap();
    for node in nodes.iter() {
        println!("Node {}: {:?}", node.id, node.get_state());
    }
}
