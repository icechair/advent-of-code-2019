#[macro_use]
extern crate log;
extern crate env_logger;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self(x,y)
    }

    fn distance(&self, other: &Self) -> i64 {
        (other.0 - self.1).abs() + (other.0 - self.1).abs()
    }
    fn bearing(&self, o: &Self) -> f64 {
        let dx = (self.0 - o.0) as f64;
        let dy = (self.1 - o.1) as f64;
        dy.atan2(dx)
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
    let mut values: Vec<(Point, usize)> = visibilities.iter().map(|(k, v)| (k.clone(),*v)).collect();
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
        let a = Point::new(1, 0);
        let b = Point::new(2, 2);
        let c = Point::new(3, 4);
        assert_eq!(a.bearing(&b), a.bearing(&c));
        assert_ne!(b.bearing(&a), b.bearing(&c));
        assert_eq!(c.bearing(&a), c.bearing(&b));
    }
}
