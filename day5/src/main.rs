#[macro_use]
extern crate text_io;

#[macro_use]
extern crate log;
extern crate env_logger;

pub mod intcode;
use intcode::{create_memory, intcode};
use std::env;
use std::fs;
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let prg = fs::read_to_string(filename).expect("cannot read file");
    let mut memory = create_memory(prg);
    intcode(&mut memory);
    debug!("end: {:?}", memory);
}
