#[macro_use]
extern crate log;
extern crate env_logger;
use std::collections::{HashMap, HashSet};
use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    fn distance(&self, other: &Self) -> i64 {
        (other.0 - self.1).abs() + (other.0 - self.1).abs()
    }
    fn bearing(&self, o: &Self) -> f64 {
        let dx = (self.0 - o.0) as f64;
        let dy = (self.1 - o.1) as f64;
        let mut angle = dy.atan2(dx) + 1.5 * PI;
        if angle >= 2.0 * PI {
            angle -= 2.0 * PI;
        }
        angle
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Edge(Point, Point);
impl Edge {
    fn new(a: &Point, b: &Point) -> Self {
        let a = a.clone();
        let b = b.clone();
        let z = Point::new(0, 0);
        if z.distance(&a) < z.distance(&b) {
            return Self(a, b);
        }
        Self(b, a)
    }
}

fn unique(list: &Vec<f64>) -> Vec<f64> {
    let mut unique: Vec<f64> = Vec::with_capacity(list.len());
    for item in list {
        match unique.iter().position(|&x| x == *item) {
            Some(v) => {}
            None => unique.push(*item),
        }
    }
    return unique;
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    let mut asteroids: HashSet<Point> = HashSet::new();
    for (r, row) in reader.lines().enumerate() {
        let row = row.expect("cannot read line");
        println!("{}", row);
        for (c, col) in row.chars().enumerate() {
            match col {
                '#' => {
                    asteroids.insert(Point::new(c as i64, r as i64));
                }
                _ => {}
            }
        }
    }

    let mut visibilities: HashMap<Point, usize> = HashMap::new();
    for a in &asteroids {
        let mut bearings: Vec<f64> = Vec::with_capacity(asteroids.len() - 1);
        for b in &asteroids {
            if a == b {
                continue;
            }
            bearings.push(a.bearing(&b));
        }
        let list = unique(&bearings);
        let v = visibilities.entry(a.clone()).or_insert(0);
        *v = list.len();
    }
    let mut values: Vec<(Point, usize)> =
        visibilities.iter().map(|(k, v)| (k.clone(), *v)).collect();
    values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("{:?}", values.pop().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let a = Point::new(1, 1);
        let b = Point::new(2, 2);
        let ab = Edge::new(&b, &a);
        assert_eq!(ab.0, a);
        assert_eq!(ab.1, b);
    }

    #[test]
    fn test_bearing() {
        let a = Point::new(1, 1);
        let b = Point::new(2, 1);
        let c = Point::new(1, 2);
        let d = Point::new(2, 2);

        assert_eq!(a.bearing(&b), 0.5 * PI);
        assert_eq!(a.bearing(&c), PI);
        assert_eq!(a.bearing(&d), 0.75 * PI);

        assert_eq!(b.bearing(&a), 1.5 * PI);
        assert_eq!(b.bearing(&c), 1.25 * PI);
        assert_eq!(b.bearing(&d), PI);

        assert_eq!(c.bearing(&a), 0.0);
        assert_eq!(c.bearing(&b), 0.25 * PI);
        assert_eq!(c.bearing(&d), 0.5 * PI);

        assert_eq!(d.bearing(&a), 1.75 * PI);
        assert_eq!(d.bearing(&b), 0.0);
        assert_eq!(d.bearing(&c), 1.5 * PI);
    }
}
