#[macro_use]
extern crate log;
extern crate env_logger;
pub mod point;
use point::{Line, Point};
use std::io::{self, BufRead};

type Wire = Vec<Line>;

fn create_wire(line: String) -> Wire {
    let mut wire: Vec<Line> = Vec::new();
    let mut start = Point::new(0, 0);
    for (_, token) in line.split(",").enumerate() {
        let section = start.next(token);
        start = section.to.clone();
        wire.push(section);
    }
    wire
}

fn create_crossings(a: &Wire, b: &Wire) -> Vec<Point> {
    let mut crossings: Vec<Point> = Vec::new();
    for line in a {
        match b.iter().find(|x| x.cross(&line).is_some()) {
            Some(v) => {
                let p = v.cross(&line).unwrap();
                crossings.push(p);
            }
            None => {}
        }
    }
    crossings
}

fn walk_to_point(wire: &Wire, p: &Point) -> i64 {
    debug!("walk_start: {:?}", p);
    let mut steps: i64 = 0;
    for line in wire {
        debug!("{:?} {}", line, line.length());
        if line.is_vertical() && line.min_x() == p.x && line.min_y() < p.y && p.y < line.max_y() {
            debug!(
                "^ crossing reached {} {}",
                line.from.distance(&p),
                line.to.distance(&p)
            );
            steps += line.from.distance(&p);
            break;
        } else if line.is_horizontal()
            && line.min_y() == p.y
            && line.min_x() < p.x
            && p.x < line.max_x()
        {
            debug!(
                "> crossing reached {} {}",
                line.from.distance(&p),
                line.to.distance(&p)
            );
            steps += line.from.distance(&p);
            break;
        }
        steps += line.length();
    }
    debug!("walk_end: {}", steps);
    steps
}

fn create_steps(a: &Wire, b: &Wire, crossings: &Vec<Point>) -> i64 {
    let mut steps: i64 = std::i64::MAX;
    for p in crossings {
        let step_a = walk_to_point(&a, &p);
        let step_b = walk_to_point(&b, &p);
        if step_a + step_b < steps {
            steps = step_a + step_b;
        }
    }
    steps
}

fn main() {
    let stdin = io::stdin();

    let line: Vec<String> = stdin
        .lock()
        .lines()
        .map(|x| x.expect("cant read line"))
        .collect();
    if line.len() != 2 {
        panic!("expected 2 lines on stdin");
    }
    let line_a = &line[0];
    let line_b = &line[1];
    let wire_a = create_wire(line_a.to_string());
    let wire_b = create_wire(line_b.to_string());
    let mut crossings = create_crossings(&wire_a, &wire_b);
    let start = Point::new(0, 0);

    crossings.sort_by(|x, y| x.distance(&start).cmp(&y.distance(&start)));
    let p = crossings.first().unwrap();

    println!("{:#?}", p.distance(&start));
    println!("{:#?}", create_steps(&wire_a, &wire_b, &crossings));
}
