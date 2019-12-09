use std::io;
use std::io::prelude::*;

fn fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn fuel_for_fuel(mass: i32) -> i32 {
    let mut fuel_total = 0;
    let mut mass = mass;
    loop {
        mass = fuel(mass);
        if mass <= 0 {
            return fuel_total;
        }
        fuel_total += mass;
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let mut total: i32 = 0;
    let mut fuel_total: i32 = 0;
    for mass in lines {
        total += fuel(mass);
        fuel_total += fuel_for_fuel(mass);
    }
    println!("part1: {}", total);
    println!("part2: {}", fuel_total);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }
    #[test]
    fn test_fuel_for_fuel() {
        assert_eq!(fuel_for_fuel(12), 2);
        assert_eq!(fuel_for_fuel(14), 2);
        assert_eq!(fuel_for_fuel(1969), 966);
        assert_eq!(fuel_for_fuel(100756), 50346);
    }
}
