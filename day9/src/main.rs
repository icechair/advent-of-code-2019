extern crate intcode;
use intcode::spawn;
use std::env;
use std::fs::read_to_string;
fn main() {
    let args: Vec<String> = env::args().collect();
    let intcode_file = &args[1];
    let mode = &args[2];
    let data = read_to_string(intcode_file).unwrap();
    let (_tx, rx, _) = spawn(data, Some(mode.clone()));

    let output = rx.recv().unwrap();
    println!("{}", output);
}
