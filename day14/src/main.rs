mod reactions;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = BufReader::new(File::open(filename).expect("main: cannot open file"));

    for line in file.lines() {
        let line = line.expect("line failed");
    }
}

#[cfg(test)]
mod test {}
