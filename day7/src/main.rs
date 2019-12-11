#[macro_use]
extern crate log;
extern crate env_logger;
use std::io;

#[macro_use]
pub mod intcode;
use intcode::IntCode;



fn main() {
    env_logger::init();
    debug!("start");
    
    let mut input = String::new();
    let stdin = io::stdin();
    
    stdin.read_line(&mut input).unwrap();
    
    let input = parse!(input, i64);
    
    println!("{}", input + 1);
}
