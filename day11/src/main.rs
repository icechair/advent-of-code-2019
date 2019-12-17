extern crate intcode;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::env;
use std::fs::{read_to_string};
use std::ops;

use intcode::spawn;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point(i64, i64);
impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, o: &Point) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Point {
    fn cmp(&self, o: &Point) -> Ordering {
        match self.1.cmp(&o.1) {
            Ordering::Equal => self.0.cmp(&o.0),
            rest => rest,
        }
    }
}
const DIRECTIONS: [Point; 4] = [Point(0, -1), Point(1, 0), Point(0, 1), Point(-1, 0)];

fn modulo(x: i64, m: i64) -> i64 {
    (x % m + m) % m
}

fn boundaries(points: &Vec<Point>) -> (Point, Point) {
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    for point in points {
        min_x = min(min_x, point.0);
        min_y = min(min_y, point.1);
        max_x = max(max_x, point.0);
        max_y = max(max_y, point.1);
    }
    (Point(min_x, min_y), Point(max_x, max_y))
}

fn draw_tiles(pdata: String, start: &String) -> HashMap<Point, String> {
    let (tx, rx, _) = spawn(pdata, None);
    let mut cursor = Point(0, 0);
    let mut direction = 0;
    let mut tiles: HashMap<Point, String> = HashMap::new();
    tiles.insert(cursor, start.clone());
    loop {
        let tile = tiles.entry(cursor).or_insert("0".to_string());
        match tx.send(tile.clone()) {
            Err(_) => break,
            Ok(_) => {}
        };
        match rx.recv() {
            Err(_) => break,
            Ok(new_tile) => *tile = new_tile,
        };
        match rx.recv() {
            Err(_) => break,
            Ok(turn) => match turn.as_ref() {
                "0" => direction = modulo(direction - 1, 4),
                "1" => direction = modulo(direction + 1, 4),
                _ => unimplemented!(),
            },
        };
        cursor = cursor + DIRECTIONS[direction as usize];
    }
    tiles
}


fn write_plate(tiles: &HashMap<Point, String>) {
    let points: Vec<Point> = tiles.iter().map(|(k, _)| *k).collect();
    let (from, to) = boundaries(&points);
    println!("{:?}", (from, to));
    let width = from.0.abs() + to.0.abs();
    for r in (from.1)..(to.1)+1 {
        let mut row = String::with_capacity(width as usize);
        for c in (from.0)..(to.0)+1 {
            match tiles.get(&Point(c, r)) {
                Some(t) => match t.as_ref() {
                    "0" => row.push(' '),
                    "1" => row.push('#'),
                    _ => row.push('?')
                },
                None => row.push(' '),
            }
        }
        println!("{}", row);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let start = &args[2];
    let pdata = read_to_string(filename).expect("cannot read file to string");
    //part1
    let tiles = draw_tiles(pdata, &start);
    println!("{}", tiles.len());
    write_plate(&tiles);
}

#[cfg(test)]
mod test {
    use super::{modulo, Point};
    #[test]
    fn test_point() {
        let a = Point(1, 2);
        let b = Point(1, 3);
        assert_eq!(a + b, Point(2, 5));
    }
    #[test]
    fn fn_test_modulo() {
        assert_eq!(modulo(5, 4), 1);
        assert_eq!(modulo(4, 4), 0);
        assert_eq!(modulo(3, 4), 3);
        assert_eq!(modulo(2, 4), 2);
        assert_eq!(modulo(1, 4), 1);
        assert_eq!(modulo(0, 4), 0);
        assert_eq!(modulo(-1, 4), 3);
        assert_eq!(modulo(-2, 4), 2);
        assert_eq!(modulo(-3, 4), 1);
        assert_eq!(modulo(-4, 4), 0);
    }
}
