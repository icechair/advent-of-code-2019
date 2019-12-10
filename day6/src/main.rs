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

fn orbit_path(map: &OrbitMap, key: &str) -> Vec<String> {
    let mut key = key;
    let mut path: Vec<String> = vec![];
    while let Some(node) = map.get(&key.to_string()) {
        key = match node {
            Some(v) => v,
            None => break,
        };
        path.push(key.to_string());
    }
    path
}

fn main() {
    env_logger::init();
    debug!("start");
    let stdin = io::stdin();
    let mut orbit_map:OrbitMap = HashMap::new();

    for (_, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("cant read line");
        let nodes = line.split(")").collect::<Vec<&str>>();
        let parent = nodes[0];
        let child = nodes[1];
        if orbit_map.len() == 0 {
            orbit_map.insert(parent.to_string(), None);
        }
        orbit_map.insert(child.to_string(), Some(parent.to_string()));
    }
    debug!("{:#?}", orbit_map);
    let mut total = 0;
    for (body, _) in &orbit_map {
        total += orbit_count(&orbit_map, &body);
    };
    println!("total orbits: {}", total);
    
    let mut you_path = orbit_path(&orbit_map, "YOU");
    you_path.reverse();
    let mut san_path = orbit_path(&orbit_map, "SAN");
    san_path.reverse();
    
    let mut common_parent = String::new();
    let mut you = you_path.iter();
    let mut san = san_path.iter();
    loop {
        let y = you.next();
        let s = san.next();
        if y == s {
            common_parent = (*s.unwrap()).to_string();
        } else {
            break;
        }
    }
    you_path.reverse();
    san_path.reverse();
    let you_transfers = you_path.iter().take_while(|x| **x != common_parent).count();
    let san_transfers = san_path.iter().take_while(|x| **x != common_parent).count();
    debug!("common: {}", common_parent);
    println!("common: {}", common_parent);
    println!("YOU transfers: {}", you_transfers);
    println!("SAN transfers: {}", san_transfers);
    println!("YOU -> SAN: {}", you_transfers + san_transfers);
}
