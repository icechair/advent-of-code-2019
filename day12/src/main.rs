extern crate regex;
use std::cmp::Ordering;
use std::ops;
use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn new(x: i64, y: i64, z: i64) -> Vector {
        Vector { x, y, z }
    }
    fn gravity(&self, o: &Vector) -> Vector {
        let x = match self.x.cmp(&o.x) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let y = match self.y.cmp(&o.y) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let z = match self.z.cmp(&o.z) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        Vector { x, y, z }
    }
    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, o: Vector) -> Vector {
        Vector::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Moon {
    position: Vector,
    velocity: Vector,
}
impl Moon {
    pub fn new(position: Vector, velocity: Vector) -> Self {
        Moon { position, velocity }
    }
}

pub fn simulate(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut result = Vec::with_capacity(moons.len());
    for a in moons {
        let mut gravity = Vector::new(0, 0, 0);
        for b in moons {
            if a == b {
                continue;
            }
            gravity = gravity + a.position.gravity(&b.position);
        }
        let velocity = a.velocity + gravity;
        let position = a.position + velocity;
        result.push(Moon::new(position, velocity));
    }
    result
}

fn total_energy(sum:i64, x: &Moon) -> i64 {
    sum + x.position.energy() * x.velocity.energy()
}

fn read_moons(filename: String) -> Vec<Moon> {
    let mut moons = Vec::new();
    let file = File::open(filename).expect("read_moons: cannot read file");
    let reader = BufReader::new(file);
    let re = Regex::new(r"x=([\d-]*).*y=([\d-]*).*z=([\d-]*)").expect("read_moons: cannot create regex");
    let nix = Vector::new(0, 0, 0);
    for line in reader.lines() {
        let line = line.expect("read_moons: cannot read line");
        for cap in re.captures_iter(&line) {
            let x:i64 = cap[1].parse().expect("read_moons: cannot read x");
            let y:i64 = cap[2].parse().expect("read_moons: cannot read y");
            let z:i64 = cap[3].parse().expect("read_moons: cannot read z");
            let moon = Moon::new(Vector::new(x,y,z), nix);
            moons.push(moon);
        }
    }

    moons
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let original = read_moons(args[1].clone());
    let mut moons = original.clone();
    let mut step = 0;
    loop {
        step += 1;
        moons = simulate(&moons);
        if step == 1000 {
            println!("total after 1000: {}", moons.iter().fold(0i64, total_energy));
        }
        if original == moons {
            println!("returned to initial after: {}", step);
           // println!("{:?}", original);
           // println!("{:?}", moons);
            break;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_panets() {
        let nix = Vector::new(0, 0, 0);
        let mut moons = vec![
            Moon::new(Vector::new(-1, 0, 2), nix),
            Moon::new(Vector::new(2, -10, -7), nix),
            Moon::new(Vector::new(4, -8, 8), nix),
            Moon::new(Vector::new(3, 5, -1), nix),
        ];
        moons = simulate(&moons);
        assert_eq!(moons[0].position, Vector::new(2, -1, 1));
        assert_eq!(moons[0].velocity, Vector::new(3, -1, -1));

        assert_eq!(moons[1].position, Vector::new(3, -7, -4));
        assert_eq!(moons[1].velocity, Vector::new(1, 3, 3));

        assert_eq!(moons[2].position, Vector::new(1, -7, 5));
        assert_eq!(moons[2].velocity, Vector::new(-3, 1, -3));

        assert_eq!(moons[3].position, Vector::new(2, 2, 0));
        assert_eq!(moons[3].velocity, Vector::new(-1, -3, 1));
        for _ in 1..10 {
            moons = simulate(&moons);
        }

        assert_eq!(moons[0].position, Vector::new(2, 1, -3));
        assert_eq!(moons[0].velocity, Vector::new(-3, -2, 1));

        assert_eq!(moons[1].position, Vector::new(1, -8, 0));
        assert_eq!(moons[1].velocity, Vector::new(-1, 1, 3));

        assert_eq!(moons[2].position, Vector::new(3, -6, 1));
        assert_eq!(moons[2].velocity, Vector::new(3, 2, -3));

        assert_eq!(moons[3].position, Vector::new(2, 0, 4));
        assert_eq!(moons[3].velocity, Vector::new(1, -1, -1));
        let total = moons.iter().fold(0i64, total_energy);
        assert_eq!(total, 179);
    }
}
