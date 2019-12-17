use std::cmp::Ordering;
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
}
impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, o: Vector) -> Vector {
        Vector::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Planet {
    position: Vector,
    velocity: Vector,
}
impl Planet {
    pub fn new(position: Vector, velocity: Vector) -> Self {
        Planet { position, velocity }
    }
}

pub fn simulate(planets: &Vec<Planet>) -> Vec<Planet> {
    let mut result = Vec::with_capacity(planets.len());
    for a in planets {
        let mut gravity = Vector::new(0, 0, 0);
        for b in planets {
            if a == b {
                continue;
            }
            gravity = gravity + a.position.gravity(&b.position);
        }
        let velocity = a.velocity + gravity;
        let position = a.position + velocity;
        result.push(Planet::new(position, velocity));
    }
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::{simulate, Planet, Vector};

    #[test]
    fn test_panets() {
        let nix = Vector::new(0, 0, 0);
        let mut planets = vec![
            Planet::new(Vector::new(-1, 0, 2), nix),
            Planet::new(Vector::new(2, -10, -7), nix),
            Planet::new(Vector::new(4, -8, 8), nix),
            Planet::new(Vector::new(3, 5, -1), nix),
        ];
        planets = simulate(&planets);
        assert_eq!(planets[0].position, Vector::new(2, -1, 1));
        assert_eq!(planets[0].velocity, Vector::new(3, -1, -1));

        assert_eq!(planets[1].position, Vector::new(3, -7, -4));
        assert_eq!(planets[1].velocity, Vector::new(1, 3, 3));

        assert_eq!(planets[2].position, Vector::new(1, -7, 5));
        assert_eq!(planets[2].velocity, Vector::new(-3, 1, -3));

        assert_eq!(planets[3].position, Vector::new(2, 2, 0));
        assert_eq!(planets[3].velocity, Vector::new(-1, -3, 1));
    }
}
