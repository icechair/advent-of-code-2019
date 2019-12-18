extern crate num;
extern crate regex;
use num::Integer;
use regex::Regex;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

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
    fn invert(&self) -> Self {
        Vector::new(self.x * -1, self.y * -1, self.z * -1)
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
    pub fn new(position: Vector) -> Self {
        Moon {
            position,
            velocity: Vector::new(0, 0, 0),
        }
    }
    fn apply_gravity(&mut self, gravity: Vector) {
        self.velocity = self.velocity + gravity;
    }
    fn apply_velocity(&mut self) {
        self.position = self.position + self.velocity
    }
    fn total_energy(&self) -> i64 {
        self.position.energy() * self.velocity.energy()
    }
    fn x_equal(&self, o: &Self) -> bool {
        self.position.x == o.position.x && self.velocity.x == o.velocity.x
    }
    fn y_equal(&self, o: &Self) -> bool {
        self.position.y == o.position.y && self.velocity.y == o.velocity.y
    }
    fn z_equal(&self, o: &Self) -> bool {
        self.position.z == o.position.z && self.velocity.z == o.velocity.z
    }
}

pub fn simulate(moons: &mut Vec<Moon>) {
    let len = moons.len();
    for a in 0..len {
        for b in a + 1..len {
            let gravity = moons[a].position.gravity(&moons[b].position);
            moons[a].apply_gravity(gravity);
            moons[b].apply_gravity(gravity.invert());
        }
    }
    for moon in moons {
        moon.apply_velocity();
    }
}

fn total_energy(sum: i64, x: &Moon) -> i64 {
    sum + x.total_energy()
}

fn read_moons(filename: String) -> Vec<Moon> {
    let mut moons = Vec::new();
    let file = File::open(filename).expect("read_moons: cannot read file");
    let reader = BufReader::new(file);
    let re =
        Regex::new(r"x=([\d-]*).*y=([\d-]*).*z=([\d-]*)").expect("read_moons: cannot create regex");
    for line in reader.lines() {
        let line = line.expect("read_moons: cannot read line");
        for cap in re.captures_iter(&line) {
            let x: i64 = cap[1].parse().expect("read_moons: cannot read x");
            let y: i64 = cap[2].parse().expect("read_moons: cannot read y");
            let z: i64 = cap[3].parse().expect("read_moons: cannot read z");
            let moon = Moon::new(Vector::new(x, y, z));
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
    let mut px: i64 = 0;
    let mut py: i64 = 0;
    let mut pz: i64 = 0;
    let len = moons.len();
    loop {
        simulate(&mut moons);
        step += 1;
        if step == 1000 {
            println!(
                "total after 1000: {}",
                moons.iter().fold(0i64, total_energy),
            );
        }
        let mut xsame = px == 0;
        let mut ysame = py == 0;
        let mut zsame = pz == 0;
        for i in 0..len {
            if xsame && !original[i].x_equal(&moons[i]) {
                xsame = false;
            }
            if ysame && !original[i].y_equal(&moons[i]) {
                ysame = false;
            }
            if zsame && !original[i].z_equal(&moons[i]) {
                zsame = false;
            }
        }

        if xsame {
            px = step;
        }
        if ysame {
            py = step;
        }
        if zsame {
            pz = step;
        }
        if px > 0 && py > 0 && pz > 0 {
            break;
        }
    }

    println!("LCM: {}", pz.lcm(&px.lcm(&py)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_panets() {
        let original = vec![
            Moon::new(Vector::new(-1, 0, 2)),
            Moon::new(Vector::new(2, -10, -7)),
            Moon::new(Vector::new(4, -8, 8)),
            Moon::new(Vector::new(3, 5, -1)),
        ];
        let mut moons = original.clone();
        assert_eq!(original, moons);
        simulate(&mut moons);
        assert_ne!(original, moons);
        assert_eq!(moons[0].position, Vector::new(2, -1, 1));
        assert_eq!(moons[0].velocity, Vector::new(3, -1, -1));

        assert_eq!(moons[1].position, Vector::new(3, -7, -4));
        assert_eq!(moons[1].velocity, Vector::new(1, 3, 3));

        assert_eq!(moons[2].position, Vector::new(1, -7, 5));
        assert_eq!(moons[2].velocity, Vector::new(-3, 1, -3));

        assert_eq!(moons[3].position, Vector::new(2, 2, 0));
        assert_eq!(moons[3].velocity, Vector::new(-1, -3, 1));
        for _ in 1..10 {
            simulate(&mut moons);
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
