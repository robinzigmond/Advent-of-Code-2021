use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn line_to(&self, other: &Point) -> Vec<Point> {
        let mut points = vec![];
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let x_step = if x_diff > 0 {
            1
        } else if x_diff < 0 {
            -1
        } else {
            0
        };

        let y_step = if y_diff > 0 {
            1
        } else if y_diff < 0 {
            -1
        } else {
            0
        };

        let mut current_x = self.x;
        let mut current_y = self.y;
        loop {
            points.push(Point::new(current_x, current_y));
            if current_x == other.x {
                if current_y == other.y {
                    break;
                }
            }
            current_x += x_step;
            current_y += y_step;
        }

        points
    }
}

// start and end of course determine the line on their own, but we add other info that is
// calculated during construction, to avoid having to recalculate it later.
// (And then removed start and end to eliminate compiler warnings as they were unused in the end!)
struct Line {
    x_diff: isize,
    y_diff: isize,
    points: Vec<Point>,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        let x_diff = end.x - start.x;
        let y_diff = end.y - start.y;

        Line {
            x_diff,
            y_diff,
            points: start.line_to(&end),
        }
    }
}

struct PointsOnLines {
    counts: HashMap<(isize, isize), usize>,
}

impl PointsOnLines {
    fn new() -> PointsOnLines {
        PointsOnLines {
            counts: HashMap::new(),
        }
    }

    fn add(&mut self, points: Vec<Point>) {
        for point in points {
            let key = (point.x, point.y);
            let current = self.counts.get_mut(&key);
            match current {
                Some(val) => *val += 1,
                None => {
                    self.counts.insert(key, 1);
                }
            }
        }
    }

    fn count_greater_than_one(&self) -> usize {
        self.counts.iter().filter(|(_, &val)| val > 1).count()
    }
}

fn parse_point(s: &str) -> Point {
    let coords: Vec<isize> = s.split(",").map(|num| num.parse().unwrap()).collect();
    Point::new(coords[0], coords[1])
}

fn parse_line(s: &str) -> Line {
    let parts: Vec<&str> = s.split_ascii_whitespace().collect();
    let start_point = parse_point(parts[0]);
    let end_point = parse_point(parts[2]);
    Line::new(start_point, end_point)
}

fn read_file() -> Vec<Line> {
    let mut file = File::open("./input/input5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(parse_line).collect()
}

fn solve_part_1(lines: Vec<Line>) -> usize {
    let mut counts = PointsOnLines::new();
    for line in lines.iter().filter(|l| l.x_diff == 0 || l.y_diff == 0) {
        counts.add(line.points.iter().map(|p| p.clone()).collect());
    }
    counts.count_greater_than_one()
}

pub fn part_1() -> usize {
    let lines = read_file();
    solve_part_1(lines)
}

fn solve_part_2(lines: Vec<Line>) -> usize {
    let mut counts = PointsOnLines::new();
    for line in lines.iter() {
        counts.add(line.points.iter().map(|p| p.clone()).collect());
    }
    counts.count_greater_than_one()
}

pub fn part_2() -> usize {
    let lines = read_file();
    solve_part_2(lines)
}
