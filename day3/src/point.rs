use std::cmp;
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    pub fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    pub fn distance(&self, b: &Point) -> i64 {
        (b.x - self.x).abs() + (b.y - self.y).abs()
    }
    pub fn next(&self, step: &str) -> Line {
        let dir = &step[0..1];
        let len: i64 = step[1..].parse().expect("invalid step");
        match dir {
            "U" => Line {
                from: Point::new(self.x, self.y),
                to: Point::new(self.x, self.y + len),
            },
            "R" => Line {
                from: Point::new(self.x, self.y),
                to: Point::new(self.x + len, self.y),
            },
            "D" => Line {
                from: Point::new(self.x, self.y),
                to: Point::new(self.x, self.y - len),
            },
            "L" => Line {
                from: Point::new(self.x, self.y),
                to: Point::new(self.x - len, self.y),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Self {
        Line { from, to }
    }
    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
    pub fn min_x(&self) -> i64 {
        cmp::min(self.from.x, self.to.x)
    }
    pub fn max_x(&self) -> i64 {
        cmp::max(self.from.x, self.to.x)
    }
    pub fn min_y(&self) -> i64 {
        cmp::min(self.from.y, self.to.y)
    }
    pub fn max_y(&self) -> i64 {
        cmp::max(self.from.y, self.to.y)
    }

    pub fn cross(&self, line: &Line) -> Option<Point> {
        if self.min_x() < line.min_x()
            && line.min_x() < self.max_x()
            && line.min_y() < self.min_y()
            && self.min_y() < line.max_y()
        {
            return Some(Point {
                x: line.min_x(),
                y: self.min_y(),
            });
        }

        if line.min_x() < self.min_x()
            && self.min_x() < line.max_x()
            && self.min_y() < line.min_y()
            && line.min_y() < self.max_y()
        {
            return Some(Point {
                x: self.min_x(),
                y: line.min_y(),
            });
        }
        return None;
    }

    pub fn length(&self) -> i64 {
        self.from.distance(&self.to)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_points() {
        let a = Point::new(0, 0);
        let b = Point::new(13, 0);
        assert_eq!(a.distance(&b), 13);
        assert!(a < b);

        let a = Point::new(-2, 3);
        assert_eq!(a.distance(&b), 18);
        assert!(a < b);

        let b = Point::new(2, -2);
        assert_eq!(a.distance(&b), 9);
        assert!(a < b);
    }
    #[test]
    fn test_lines() {
        let start = Point::new(0, 0);
        let a = start.next("R13");
        assert!(a.to == Point::new(13, 0));

        assert_eq!(a.length(), 13);
        println!("a is 13");
        assert!(a.is_horizontal());

        let b = a.to.next("D14");
        assert!(b.to == Point::new(13, -14));
        assert_eq!(b.length(), 14);
        println!("b is 14");
        assert!(b.is_vertical());
        println!("b is vertical");
        let a = Point::new(6, 3).next("L4");
        let b = Point::new(3, 5).next("D3");

        assert_eq!(a.cross(&b), Some(Point::new(3, 3)));
        println!("cross is 3,3");
        let a = Point::new(6, 7).next("D4");
        let b = Point::new(8, 5).next("L5");

        assert_eq!(a.cross(&b), Some(Point::new(6, 5)));
        println!("cross is 6,5");
        let a = Point::new(8, 0).next("U5");
        let b = Point::new(0, 7).next("R6");

        assert_eq!(a.cross(&b), None);
        println!("cross is None");
        assert_eq!(a.length(), 5);
        println!("a is 5");
        assert_eq!(b.length(), 6);
        println!("a is 6");
    }
}
