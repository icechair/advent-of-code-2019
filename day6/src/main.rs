#[macro_use]
extern crate log;
extern crate env_logger;
use std::io::{self, BufRead};

fn main() {
    env_logger::init();
    debug!("start");
    let stdin = io::stdin();
    for (_, line) in stdin.lock().lines().enumerate() {
        
    }
    
    let mut total = 0;
    
    println!("{}", total);
}
