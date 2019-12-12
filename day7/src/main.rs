#[macro_use]
extern crate log;
extern crate env_logger;
use std::io;
use std::env;
use std::fs;
#[macro_use]
pub mod intcode;
use intcode::IntCode;
use std::io::{BufRead, Write};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();     
    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("coudnt read file");
    
    
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let input: Box<&mut dyn BufRead> = Box::new(&mut stdin);
    
    let mut outbuf:Vec<u8> = Vec::new();
    let output: Box<&mut dyn Write> = Box::new(&mut outbuf);
    
    let mut intcode = IntCode::new(data, input, output);
    intcode.run();
    let mut stdout = io::stdout();
    write!(&mut stdout, "{}", std::str::from_utf8(&outbuf).expect("outbuf is not utf8")).expect("cannot write to stdout?");
    
}
