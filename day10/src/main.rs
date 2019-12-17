#[macro_use]
extern crate log;
extern crate env_logger;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    fn distance(&self, o: Self) -> i64 {
        (o.0 - o.1).abs() - (self.0 - self.1).abs()
    }
    fn bearing(&self, o: Self) -> f64 {
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
    fn new(a: Point, b: Point) -> Self {
        let a = a;
        let b = b;
        let z = Point::new(0, 0);
        if z.distance(a) < z.distance(b) {
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

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader.lines().map(|x| x.unwrap()).collect()
}

fn asteroid_set(lines: Vec<String>) -> HashSet<Point> {
    let mut asteroids: HashSet<Point> = HashSet::new();
    for (r, row) in lines.iter().enumerate() {
        for (c, col) in row.chars().enumerate() {
            match col {
                '#' => {
                    asteroids.insert(Point::new(c as i64, r as i64));
                }
                _ => {}
            }
        }
    }
    asteroids
}

fn visibility_map(asteroids: &HashSet<Point>) -> HashMap<Point, usize> {
    let mut visibilities: HashMap<Point, usize> = HashMap::new();
    for a in asteroids {
        let mut bearings: Vec<f64> = Vec::with_capacity(asteroids.len() - 1);
        for b in asteroids {
            if a == b {
                continue;
            }
            bearings.push(a.bearing(*b));
        }
        let list = unique(&bearings);
        let v = visibilities.entry(*a).or_insert(0);
        *v = list.len();
    }
    visibilities
}

fn simulate_laser(origin: Point, asteroids: &HashSet<Point>) -> Vec<Point> {
    let mut killed = Vec::new();
    let mut bearings: Vec<(Point, f64, i64)> = asteroids
        .into_iter()
        .map(|p| (*p, origin.bearing(*p), origin.distance(*p)))
        .collect();
    bearings.sort_by(|a, b| match a.1.partial_cmp(&b.1).unwrap() {
        Ordering::Equal => a.2.partial_cmp(&b.2).unwrap(),
        some => some,
    });
    let mut bearings: VecDeque<(Point, f64, i64)> = VecDeque::from(bearings);

    let mut last_bearing: f64 = 1.99 * PI;
    loop {
        let k_len = killed.len();
        let a_len = asteroids.len();
        let b_len = bearings.len();
        if k_len > 0 {
            println!(
                "{:?}:{:?}",
                k_len,
                killed[k_len - 1],
            );
        }
        match bearings.pop_front() {
            Some(target) => {
                if target.0 == origin {
                    continue;
                }
                if bearings.len() == 0 {
                    killed.push(target.0);
                    break;
                }
                if target.1 == last_bearing {
                    bearings.push_back(target);
                    continue;
                }
                killed.push(target.0);
                last_bearing = target.1;
            }
            None => break,
        }
    }
    killed
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let lines = read_lines(args[1].clone());
    let asteroids = asteroid_set(lines);

    let visibilities = visibility_map(&asteroids);
    let mut values: Vec<(Point, usize)> = visibilities.iter().map(|(k, v)| (*k, *v)).collect();
    values.sort_by(|a, b| a.1.cmp(&b.1));
    let choice = values.pop().unwrap();
    println!("{:?}", choice);

    let killed = simulate_laser(choice.0, &asteroids);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let a = Point::new(8, 0);
        let b = Point::new(8, 1);
        let c = Point::new(8, 3);
        let ab = Edge::new(b, a);
        assert_eq!(ab.0, a);
        assert_eq!(ab.1, b);
    }

    #[test]
    fn test_bearing() {
        let a = Point::new(1, 1);
        let b = Point::new(2, 1);
        let c = Point::new(1, 2);
        let d = Point::new(2, 2);

        assert_eq!(a.bearing(b), 0.5 * PI);
        assert_eq!(a.bearing(c), PI);
        assert_eq!(a.bearing(d), 0.75 * PI);

        assert_eq!(b.bearing(a), 1.5 * PI);
        assert_eq!(b.bearing(c), 1.25 * PI);
        assert_eq!(b.bearing(d), PI);

        assert_eq!(c.bearing(a), 0.0);
        assert_eq!(c.bearing(b), 0.25 * PI);
        assert_eq!(c.bearing(d), 0.5 * PI);

        assert_eq!(d.bearing(a), 1.75 * PI);
        assert_eq!(d.bearing(b), 0.0);
        assert_eq!(d.bearing(c), 1.5 * PI);
    }
}
