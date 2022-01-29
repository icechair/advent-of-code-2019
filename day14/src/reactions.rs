use std::collections::HashMap;
#[derive(Hash, Eq)]
pub struct Chemical {
    name: String,
    amount: usize,
}

impl PartialEq for Chemical {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn parse_chemical(line: &str) -> Chemical {
    let mut tokens = line.trim().split(" ");
    match tokens.next() {
        None => unreachable!("chemical empty: {:?}", line),
        Some(amount) => match amount.parse::<usize>() {
            Ok(amount) => match tokens.next() {
                None => unreachable!("missing chemical name: {:?}", line),
                Some(name) => Chemical {
                    name: name.to_string(),
                    amount,
                },
            },
            Err(_) => unreachable!("invalid chemical amount: {:?}", line),
        },
    }
}

type Reaction = Vec<Chemical>;

fn parse_reaction(line: &str) -> (Chemical, Reaction) {
    let mut parts = line.split("=>");
    let left = parts.next().expect("empty line").trim().split(",");
    let output = parts.next().expect("no output").trim();
    let output = parse_chemical(output);
    let mut reaction = Vec::new();
    for input in left {
        let input = parse_chemical(input);
        reaction.push(input);
    }
    (output, reaction)
}

struct ReactionTable {
    reactions: HashMap<Chemical, Reaction>,
}

impl ReactionTable {
    pub fn new() -> Self {
        let mut reactions = HashMap::new();
        reactions.insert(parse_chemical("1 ORE"), Vec::new());
        Self { reactions }
    }

    pub fn add_reaction(&mut self, line: &str) {
        let (chem, reaction) = parse_reaction(line);
        self.reactions.insert(chem, reaction);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reaction_table() {
        let mut table = ReactionTable::new();
        table.add_reaction("10 ORE => 10 A");
        table.add_reaction("1 ORE => 1 B");
        table.add_reaction("7 A, 1 B => 1 C");
        table.add_reaction("7 A, 1 C => 1 D");
        table.add_reaction("7 A, 1 D => 1 E");
        table.add_reaction("7 A, 1 E => 1 FUEL");
    }
}
