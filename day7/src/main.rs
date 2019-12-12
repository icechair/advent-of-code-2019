#[macro_use]
extern crate log;
extern crate env_logger;

extern crate itertools;
use itertools::Itertools;

use std::env;
use std::fs;

#[macro_use]
extern crate intcode;
use intcode::spawn;

fn main() {
    env_logger::init();
    debug!("start");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let from = parse!(&args[2], usize);
    let to = parse!(&args[3], usize);
    let data = fs::read_to_string(filename).unwrap();

    for settings in (from..(to + 1)).permutations(5) {
        let phase_a = settings[0].to_string();
        let (tx_to_a, rx_from_a, _) = spawn(data.clone(), phase_a);
        tx_to_a.send(String::from("0")).unwrap();

        let phase_b = settings[1].to_string();
        let (tx_to_b, rx_from_b, _) = spawn(data.clone(), phase_b);
        tx_to_b.send(rx_from_a.recv().unwrap()).unwrap();

        let phase_c = settings[2].to_string();
        let (tx_to_c, rx_from_c, _) = spawn(data.clone(), phase_c);
        tx_to_c.send(rx_from_b.recv().unwrap()).unwrap();

        let phase_d = settings[3].to_string();
        let (tx_to_d, rx_from_d, _) = spawn(data.clone(), phase_d);
        tx_to_d.send(rx_from_c.recv().unwrap()).unwrap();

        let phase_e = settings[4].to_string();
        let (tx_to_e, rx_from_e, _) = spawn(data.clone(), phase_e);
        tx_to_e.send(rx_from_d.recv().unwrap()).unwrap();
        let mut out = String::new();
        for recv in rx_from_e {
            out = recv.clone();
            //debug!("main: loop: {}", out);
            match tx_to_a.send(recv) {
                Ok(_) => {
                    tx_to_b.send(rx_from_a.recv().unwrap()).unwrap();
                    tx_to_c.send(rx_from_b.recv().unwrap()).unwrap();
                    tx_to_d.send(rx_from_c.recv().unwrap()).unwrap();
                    tx_to_e.send(rx_from_d.recv().unwrap()).unwrap();
                }
                Err(_) => {}
            };
        }
        println!("{}", out);
    }
}
