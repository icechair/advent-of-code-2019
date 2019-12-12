#[macro_use]
extern crate log;
extern crate env_logger;
use std::io;
use std::env;
use std::fs;
#[macro_use]
pub mod intcode;
use intcode::IntCode;
use std::io::{BufRead};
use std::sync::mpsc::{channel};
use std::thread;
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();     
    let filename = &args[1];

    let data = fs::read_to_string(filename).expect("coudnt read file");

    let (tx, prx) = channel();
    let (ptx, rx) = channel();
    let pdata = data.to_string();
    thread::spawn(move || {
        let mut p = IntCode::new(pdata, prx, ptx);
        p.run();
    });
    let stdin = io::stdin();
    for (_, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("cannot read line");
        tx.send(line).expect("main: cannot send");
    } 
    let out = rx.recv().expect("main: cannot recv");
    println!("{}", out);
}
