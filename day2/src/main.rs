pub mod intcode;

use std::env;
use std::io::{self, BufRead};
use std::mem::replace;

use intcode::{create_memory, intcode};

fn main() {
    let stdin = io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
    let args: Vec<String> = env::args().collect();
    let noun: i64 = args[1].parse().unwrap();
    let verb: i64 = args[2].parse().unwrap();

    let mut memory = create_memory(line);
    replace(&mut memory[1], noun);
    replace(&mut memory[2], verb);
    println!("{}", intcode(&mut memory));
}
