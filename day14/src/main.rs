use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::collections::HashMap;

fn read_file<'a, 'b>(filename: &str) -> HashMap<&'a str, Reaction<'b>> {
    let file = File::open(filename).expect("read_file: cannot open file");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.expect("read_line: cannot read line"));//.collect()
    let mut reaction_table = HashMap::new();
    let ore = Reaction::new("ORE", 1, None);
    reaction_table.insert("ORE", ore);
    for line in lines {
        let parts = line.split("=>").collect::<Vec<&str>>();
        let inp = parts[0].split(",");
        for part in inp {
            let comp = part.trim().split(" ").collect::<Vec<&str>>();
            let amount = comp[0].parse::<u64>().unwrap();
            let creates = comp[1];
            let reaction = reaction_table.entry(creates).or_insert(Reaction::new(creates, amount, None));
            let reaction = Reaction::new(creates, amount, inputs: Option<Vec<Reaction>>)
        }
        let outp = parts[1].split(" ").collect::<Vec<&str>>();
        let creates = outp[1];
        let amount = outp[0].parse::<u64>().unwrap();
        
    }
    reaction_table
}

pub struct Reaction<'a> {
    creates: &'a str,
    amount: u64,
    inputs: Option<Vec<Reaction<'a>>>
}

impl<'a> Reaction<'a> {
    pub fn new(creates:&'a str, amount: u64, inputs: Option<Vec<Reaction>>) -> Self{
        Reaction {creates, amount, inputs: None}
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_file(&filename);
}
