#[macro_use]
extern crate log;
extern crate env_logger;
use std::io::{self, BufRead};
fn check_rules_part1(pw: String) -> bool {
    if pw.len() < 6 {
        return false;
    }
    let mut last = 0;
    let mut double = false;
    for c in pw.chars() {
        last = match c.to_digit(10) {
            Some(v) => {
                if last > v {
                    return false;
                }
                if v == last {
                    double = true;
                }
                v
            }
            None => return false,
        };
    }
    return double;
}

fn main() {
    env_logger::init();
    let stdin = io::stdin();

    let line = stdin
        .lock()
        .lines()
        .next()
        .expect("no new line")
        .expect("cant read line");
    let range: Vec<u32> = line.split("-").map(|x| x.parse().unwrap()).collect();
    let mut n_part1 = 0;
    for pw in range[0]..range[1] {
        n_part1 = match check_rules_part1(pw.to_string()) {
            true => n_part1 + 1,
            false => n_part1,
        }
    }
    println!("{}", n_part1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_rules() {
        env_logger::init();
        assert_eq!(check_rules_part1("111111".to_string()), true);
        assert_eq!(check_rules_part1("223450".to_string()), false);
        assert_eq!(check_rules_part1("123789".to_string()), false);
        assert_eq!(check_rules_part1("138241".to_string()), false);
        assert_eq!(check_rules_part1("674034".to_string()), false);
    }
}
