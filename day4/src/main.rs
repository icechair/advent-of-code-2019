#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::{self, BufRead};

type Maybe = Option<Vec<u32>>;

fn check_common(pw: String) -> Maybe {
    if pw.len() < 6 {
        return None;
    }
    let mut list: Vec<u32> = vec![0; 10];
    let mut last = 0;
    for c in pw.chars() {
        last = match c.to_digit(10) {
            Some(v) => {
                if last > v {
                    return None;
                }
                list[v as usize] = list[v as usize] + 1;
                v
            }
            None => return None,
        }
    }
    debug!("part2: list: {:?}", list);
    Some(list)
}

fn check_part1(maybe: &Maybe) -> bool {
    match maybe {
        Some(list) => {
            for item in list {
                if *item >= 2 {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

fn check_part2(maybe: &Maybe) -> bool {
    match maybe {
        Some(list) => {
            for item in list {
                if *item == 2 {
                    return true;
                }
            }
            false
        }
        None => false,
    }
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
    debug!("range: {:?}", range);
    let mut n_part1 = 0;
    let mut n_part2 = 0;
    for pw in range[0]..range[1] {
        let maybe = check_common(pw.to_string());
        n_part1 = match check_part1(&maybe) {
            true => n_part1 + 1,
            false => n_part1,
        };
        n_part2 = match check_part2(&maybe) {
            true => n_part2 + 1,
            false => n_part2,
        }
    }
    println!("{}", n_part1);
    println!("{}", n_part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_rules() {
        env_logger::init();
        assert_eq!(check_part1(&check_common("111111".to_string())), true);
        assert_eq!(check_part1(&check_common("223450".to_string())), false);
        assert_eq!(check_part1(&check_common("123789".to_string())), false);
        assert_eq!(check_part1(&check_common("138241".to_string())), false);
        assert_eq!(check_part1(&check_common("674034".to_string())), false);

        assert_eq!(check_part2(&check_common("112233".to_string())), true);
        assert_eq!(check_part2(&check_common("123444".to_string())), false);
        assert_eq!(check_part2(&check_common("111122".to_string())), true);
    }
}
