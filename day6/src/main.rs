#[macro_use]
extern crate log;
extern crate env_logger;
use std::io::{self, BufRead};
use std::collections::HashMap;

type OrbitMap = HashMap<String, Option<String>>;

fn orbit_count(map: &OrbitMap, key: &str) -> usize {
    let mut key = key;
    let mut n = 0;
    while let Some(node) = map.get(&key.to_string()) {
        key = match node {
            Some(v) => {
                v
            },
            None => break,
        };
        n += 1;
    }
    n   
}

fn main() {
    env_logger::init();
    debug!("start");
    let stdin = io::stdin();
    let mut bodies:OrbitMap = HashMap::new();

    for (_, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("cant read line");
        let nodes = line.split(")").collect::<Vec<&str>>();
        let parent = nodes[0];
        let child = nodes[1];
        if bodies.len() == 0 {
            bodies.insert(parent.to_string(), None);
        }
        bodies.insert(child.to_string(), Some(parent.to_string()));
    }
    debug!("{:#?}", bodies);
    let mut total = 0;
    for (body, _) in &bodies {
        total += orbit_count(&bodies, &body);
    };
    
    println!("{}", total);
}
