#[macro_use]
extern crate log;
extern crate env_logger;
use std::io;

pub mod intcode;

macro_rules! parse {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().expect("parse failed"))
}


fn main() {
    env_logger::init();
    debug!("start");
    
    let mut input = String::new();
    let stdin = io::stdin();
    
    stdin.read_line(&mut input).unwrap();
    
    let input = parse!(input, i64);
    
    println!("{}", input + 1);
}
