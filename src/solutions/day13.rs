use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy)]
struct Fold {
    axis: Axis,
    position: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CoOrd {
    x: usize,
    y: usize,
}

struct PuzzleData {
    points: Vec<CoOrd>,
    folds: Vec<Fold>,
}

impl PuzzleData {
    fn do_fold(&self, fold: Fold) -> Vec<CoOrd> {
        let mut new_points: Vec<CoOrd> = vec![];
        let Fold { axis, position } = fold;
        for &point in &self.points {
            let to_compare = match axis {
                Axis::X => point.x,
                Axis::Y => point.y,
            };
            if to_compare != position {
                let new_point = if to_compare < position {
                    point
                } else {
                    match axis {
                        Axis::X => CoOrd {
                            x: 2 * position - point.x,
                            ..point
                        },
                        Axis::Y => CoOrd {
                            y: 2 * position - point.y,
                            ..point
                        },
                    }
                };
                // note: we do NOT check here that the point does not exist in the
                // new_points vector. This would be too much of a performance hit to do
                // all the time. We do it at the end, efficiently using a hashmap.
                new_points.push(new_point);
            }
        }
        new_points
    }

    fn do_fold_mut(&mut self, fold: Fold) {
        let new_points = self.do_fold(fold);
        self.points = new_points;
    }

    fn complete_fold(&mut self) {
        for fold in self.folds.clone() {
            self.do_fold_mut(fold);
        }
    }

    fn do_print(&self) {
        let all_points = get_uniques(&self.points);
        let xs = all_points.iter().map(|CoOrd { x, .. }| x);
        let ys = all_points.iter().map(|CoOrd { y, .. }| y);
        let &min_x = xs.clone().min().unwrap();
        let &max_x = xs.max().unwrap();
        let &min_y = ys.clone().min().unwrap();
        let &max_y = ys.max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if all_points.contains(&CoOrd {x, y}) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

// general utility to efficiently get the unique elements in a vector
fn get_uniques(v: &Vec<CoOrd>) -> Vec<CoOrd> {
    let mut all: HashMap<CoOrd, ()> = HashMap::new();
    for &point in v {
        all.insert(point, ());
    }
    all.into_keys().collect()
}

fn parse_point(s: &&str) -> CoOrd {
    let parts: Vec<&str> = s.split(",").collect();
    let x = parts[0].parse().unwrap();
    let y = parts[1].parse().unwrap();
    CoOrd { x, y }
}

fn parse_fold(s: &&str) -> Fold {
    let parts: Vec<&str> = s.split("=").collect();
    // need to remove "fold along " to get the single character afterwards
    let axis = match &parts[0][11..] {
        "x" => Axis::X,
        "y" => Axis::Y,
        c => panic!("unexpected axis character: {}", c),
    };
    let position = parts[1].parse().unwrap();
    Fold { axis, position }
}

fn read_file() -> PuzzleData {
    let mut file = File::open("./input/input13.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parts: Vec<Vec<&str>> = contents
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _)| !k)
        .map(|(_, g)| g.collect())
        .collect();

    let points = parts[0].iter().map(parse_point).collect();
    let folds = parts[1].iter().map(parse_fold).collect();

    PuzzleData { points, folds }
}

fn solve_part_1(data: PuzzleData) -> usize {
    let folded = data.do_fold(data.folds[0]);
    get_uniques(&folded).len()
}

pub fn part_1() -> usize {
    let data = read_file();
    solve_part_1(data)
}

pub fn part_2() {
    let mut data = read_file();
    data.complete_fold();
    data.do_print();
}
