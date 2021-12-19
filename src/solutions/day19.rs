use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Copy, Debug)]
enum AxesFlipped {
    None,
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
    XYZ,
}

impl AxesFlipped {
    fn all_even() -> Vec<Self> {
        vec![
            AxesFlipped::None,
            AxesFlipped::XY,
            AxesFlipped::XZ,
            AxesFlipped::YZ,
        ]
    }

    fn all_odd() -> Vec<Self> {
        vec![
            AxesFlipped::X,
            AxesFlipped::Y,
            AxesFlipped::Z,
            AxesFlipped::XYZ,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
enum Permutation {
    None,
    YZX,
    ZXY,
    XZY,
    ZYX,
    YXZ,
}

impl Permutation {
    fn all_even() -> Vec<Self> {
        vec![Permutation::None, Permutation::YZX, Permutation::ZXY]
    }

    fn all_odd() -> Vec<Self> {
        vec![Permutation::XZY, Permutation::ZYX, Permutation::YXZ]
    }
}

#[derive(Debug)]
struct Rotation {
    flipped: AxesFlipped,
    permutation: Permutation,
}

impl Rotation {
    fn all() -> Vec<Self> {
        let mut all = vec![];

        let all_flips = AxesFlipped::all_even();
        let all_perms = Permutation::all_even();
        for &flipped in &all_flips {
            for &permutation in &all_perms {
                all.push(Rotation {
                    flipped,
                    permutation,
                });
            }
        }

        let all_flips = AxesFlipped::all_odd();
        let all_perms = Permutation::all_odd();
        for &flipped in &all_flips {
            for &permutation in &all_perms {
                all.push(Rotation {
                    flipped,
                    permutation,
                });
            }
        }

        all
    }
}

impl Point {
    fn rotate(&self, r: &Rotation) -> Self {
        let mut rotated = Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        match r.permutation {
            Permutation::None => (),
            Permutation::YZX => {
                rotated.x = self.y;
                rotated.y = self.z;
                rotated.z = self.x;
            }

            Permutation::ZXY => {
                rotated.x = self.z;
                rotated.y = self.x;
                rotated.z = self.y;
            }
            Permutation::XZY => {
                rotated.y = self.z;
                rotated.z = self.y;
            }
            Permutation::ZYX => {
                rotated.x = self.z;
                rotated.z = self.x;
            }
            Permutation::YXZ => {
                rotated.x = self.y;
                rotated.y = self.x;
            }
        }
        match r.flipped {
            AxesFlipped::None => (),
            AxesFlipped::X => {
                rotated.x *= -1;
            }
            AxesFlipped::Y => {
                rotated.y *= -1;
            }
            AxesFlipped::Z => {
                rotated.z *= -1;
            }
            AxesFlipped::XY => {
                rotated.x *= -1;
                rotated.y *= -1;
            }
            AxesFlipped::XZ => {
                rotated.x *= -1;
                rotated.z *= -1;
            }
            AxesFlipped::YZ => {
                rotated.y *= -1;
                rotated.z *= -1;
            }
            AxesFlipped::XYZ => {
                rotated.x *= -1;
                rotated.y *= -1;
                rotated.z *= -1;
            }
        }
        rotated
    }

    fn rotate_and_translate(
        &self,
        r: &Rotation,
        translate_x: isize,
        translate_y: isize,
        translate_z: isize,
    ) -> Self {
        let mut rotated = self.rotate(r);
        rotated.x += translate_x;
        rotated.y += translate_y;
        rotated.z += translate_z;
        rotated
    }

    fn manhattan(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

// the ID is used to keep track of which scans match with others, to dramatically improve performance
#[derive(Clone)]
struct Scan {
    id: usize,
    points: Vec<Point>,
}

impl Scan {
    fn rotate_and_translate_all(
        &mut self,
        r: &Rotation,
        translate_x: isize,
        translate_y: isize,
        translate_z: isize,
    ) {
        for point in &mut self.points {
            *point = point.rotate_and_translate(r, translate_x, translate_y, translate_z);
        }
    }

    fn count_overlaps(&self, other: &Scan) -> usize {
        self.points
            .iter()
            .filter(|point| other.points.contains(point))
            .count()
    }

    fn get_transform_for_overlapping_points(
        &self,
        other: &Scan,
        target: usize,
    ) -> Option<(Rotation, isize, isize, isize)> {
        // This method does the dumb brute force approach to try to find any combination of rotation
        // and translation that puts enough points on the "other" scan on top of this one.
        // Namely, it goes through every point in other, and then tries to put that on each point in
        // self, in turn - going through each of the 24 rotation types and figuring out the translation needed.
        // For each of these it tests how many points overlap between the original and the transformed other,
        // and if it's at least the target number, that transformation gets returned.
        for my_point in &self.points {
            for other_point in &other.points {
                for rotation in Rotation::all() {
                    let rotated = my_point.rotate(&rotation);
                    let translate_x = other_point.x - rotated.x;
                    let translate_y = other_point.y - rotated.y;
                    let translate_z = other_point.z - rotated.z;
                    let mut candidate = self.clone();
                    candidate.rotate_and_translate_all(
                        &rotation,
                        translate_x,
                        translate_y,
                        translate_z,
                    );
                    if other.count_overlaps(&candidate) >= target {
                        return Some((rotation, translate_x, translate_y, translate_z));
                    }
                }
            }
        }
        None
    }
}

fn parse_points(line: &str) -> Point {
    let nums: Vec<isize> = line.split(",").map(|num| num.parse().unwrap()).collect();
    Point {
        x: nums[0],
        y: nums[1],
        z: nums[2],
    }
}

fn parse_scan(lines: Vec<&str>, id: usize) -> Scan {
    let point_lines = &lines[1..]; // need to chop off --- scanner XXX --- from the start
    let points = point_lines.iter().map(|&str| parse_points(str)).collect();
    Scan { points, id }
}

fn read_file() -> Vec<Scan> {
    let mut file = File::open("./input/input19.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _)| !k)
        .enumerate()
        .map(|(idx, (_, g))| parse_scan(g.collect(), idx))
        .collect()
}

fn all_beacons_and_scanners(scans: Vec<Scan>) -> (Vec<Point>, Vec<Point>) {
    // cache for matching
    let mut no_matches = HashMap::new();
    let number = scans.len();
    let reference = scans.clone().into_iter().nth(0).unwrap();
    let mut actual_points = reference.points.clone();
    let mut scanners = vec![];
    let mut done = vec![reference];
    let mut todo = scans[1..].to_vec().clone();
    'outer: loop {
        for (todo_idx, scan) in todo.iter().enumerate() {
            let empty = vec![];
            for ref_scan in &done {
                let no_need = no_matches.get(&scan.id).unwrap_or(&empty);
                if no_need.contains(&ref_scan.id) {
                    continue;
                }
                let mut cloned = scan.clone();
                if let Some((rotation, translate_x, translate_y, translate_z)) =
                    cloned.get_transform_for_overlapping_points(&ref_scan, 12)
                {
                    scanners.push(Point {
                        x: translate_x,
                        y: translate_y,
                        z: translate_z,
                    });
                    cloned.rotate_and_translate_all(
                        &rotation,
                        translate_x,
                        translate_y,
                        translate_z,
                    );
                    actual_points.extend_from_slice(&cloned.points);
                    actual_points.sort_unstable();
                    actual_points.dedup();
                    done.push(cloned);
                    todo.remove(todo_idx);
                    if done.len() == number {
                        break 'outer;
                    }
                    continue 'outer;
                } else {
                    let not_matched_yet = no_matches.entry(scan.id).or_insert(vec![]);
                    not_matched_yet.push(ref_scan.id);
                }
            }
        }
    }
    (actual_points, scanners)
}

fn max_manhattan(beacons: Vec<Point>) -> usize {
    let mut max = 0;
    for (idx, beacon) in beacons.iter().enumerate() {
        for other_beacon in &beacons[(idx + 1)..] {
            let manhattan = beacon.manhattan(other_beacon);
            if manhattan > max {
                max = manhattan;
            }
        }
    }
    max
}

pub fn solve() -> (usize, usize) {
    let scans = read_file();
    let (beacons, scanners) = all_beacons_and_scanners(scans);
    let ans_1 = beacons.len();
    let ans_2 = max_manhattan(scanners);
    (ans_1, ans_2)
}
