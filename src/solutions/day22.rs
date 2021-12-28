use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum CubeState {
    Off,
    On,
}

#[derive(Clone, Debug)]
struct Step {
    state: CubeState,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Step {
    // processes this step - only caring about cubes between (inclusive) the minbound
    // and maxbound on each axis. Acts on a mutable hashmap argument.
    fn process(
        &self,
        map: &mut HashMap<(isize, isize, isize), ()>,
        minbound: isize,
        maxbound: isize,
    ) {
        let Step {
            state,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        } = self;

        let actual_xmin = max(minbound, *x_min);
        let actual_xmax = min(maxbound, *x_max);
        let actual_ymin = max(minbound, *y_min);
        let actual_ymax = min(maxbound, *y_max);
        let actual_zmin = max(minbound, *z_min);
        let actual_zmax = min(maxbound, *z_max);

        for x in actual_xmin..=actual_xmax {
            for y in actual_ymin..=actual_ymax {
                for z in actual_zmin..=actual_zmax {
                    match map.get(&(x, y, z)) {
                        None => {
                            // cube is off. Turn it on if we're instructed to
                            if let CubeState::On = state {
                                map.insert((x, y, z), ());
                            }
                        }
                        Some(_) => {
                            // cube is on. Turn it off if we're instructed to
                            if let CubeState::Off = state {
                                map.remove(&(x, y, z));
                            }
                        }
                    }
                }
            }
        }
    }
}

struct Steps(Vec<Step>);

impl Steps {
    // processes all the steps - only caring about cubes between (inclusive) the minbound
    // and maxbound on each axis. Returns all on cubes in the form of a hashmap.
    fn process(&self, minbound: isize, maxbound: isize) -> HashMap<(isize, isize, isize), ()> {
        let mut result = HashMap::new();
        for step in &self.0 {
            step.process(&mut result, minbound, maxbound);
        }
        result
    }
}

// types used for part 2
#[derive(Clone, Copy, Debug, PartialEq)]
struct Cuboid {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Cuboid {
    fn from_step(step: &Step) -> Self {
        let Step {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            ..
        } = step;

        let x_min = *x_min;
        let x_max = *x_max;
        let y_min = *y_min;
        let y_max = *y_max;
        let z_min = *z_min;
        let z_max = *z_max;

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn volume(&self) -> u64 {
        ((self.x_max as i64 + 1 - self.x_min as i64)
            * (self.y_max as i64 + 1 - self.y_min as i64)
            * (self.z_max as i64 + 1 - self.z_min as i64)) as u64
    }

    fn overlap(min1: isize, max1: isize, min2: isize, max2: isize) -> Option<(isize, isize)> {
        let mut sorted = vec![min1, max1, min2, max2];
        sorted.sort_unstable();
        if sorted[1] == min1 || sorted[1] == min2 {
            // if the order isn't [min, max, min, max] then we have an overlap!
            Some((sorted[1], sorted[2]))
        } else {
            None
        }
    }

    fn intersection_area(&self, other: &Self) -> Option<Self> {
        let Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        } = self;

        let Self {
            x_min: other_x_min,
            x_max: other_x_max,
            y_min: other_y_min,
            y_max: other_y_max,
            z_min: other_z_min,
            z_max: other_z_max,
        } = other;

        let overlap_x = Self::overlap(*x_min, *x_max, *other_x_min, *other_x_max);
        let overlap_y = Self::overlap(*y_min, *y_max, *other_y_min, *other_y_max);
        let overlap_z = Self::overlap(*z_min, *z_max, *other_z_min, *other_z_max);
        if let Some((min_x, max_x)) = overlap_x {
            if let Some((min_y, max_y)) = overlap_y {
                if let Some((min_z, max_z)) = overlap_z {
                    return Some(Self {
                        x_min: min_x,
                        x_max: max_x,
                        y_min: min_y,
                        y_max: max_y,
                        z_min: min_z,
                        z_max: max_z,
                    });
                }
            }
        }
        None
    }

    fn intersection(&self, other: &Self) -> (Vec<Self>, Vec<Self>) {
        // this method returns 2 vectors of cuboids. The second is those which are part
        // of the just added one, the first those which were already part of self.
        // This is to be able to handle correctly adding a cuboid which intersects more
        // than one existing one.
        //println!("intersection of {:?} with {:?}", self, other);
        match self.intersection_area(other) {
            None => (vec![*self], vec![*other]),
            Some(intersection) => {
                // if new area is totally contained within an existing cuboid, don't need
                // anything new. Need to return it explicitly to avoid infinite loops
                if &intersection == other {
                    (vec![], vec![])
                } else {
                    let mut old = self.remove(&intersection);
                    old.push(intersection);
                    let new = other.remove(&intersection);
                    //println!("intersection in {:?}", intersection);
                    //println!("left over: {:?}", leftover);
                    (old, new)
                }
            }
        }
    }

    fn remove(&self, other: &Self) -> Vec<Self> {
        match self.intersection_area(other) {
            None => vec![*self],
            Some(intersection) => {
                //println!("removing {:?} from {:?}", other, self);
                let mut remaining = vec![];

                let Self {
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    z_min,
                    z_max,
                } = self;

                let Self {
                    x_min: intersection_x_min,
                    x_max: intersection_x_max,
                    y_min: intersection_y_min,
                    y_max: intersection_y_max,
                    z_min: intersection_z_min,
                    z_max: intersection_z_max,
                } = intersection;
                //println!("intersection is {:?}", intersection);
                let x_points = [*x_min, intersection_x_min, intersection_x_max, *x_max];
                let y_points = [*y_min, intersection_y_min, intersection_y_max, *y_max];
                let z_points = [*z_min, intersection_z_min, intersection_z_max, *z_max];
                /*need to somehow rewrite this to "merge" different cuboids together where this is possible!
                (as number is going up exponentially each step, to soon reach the millions and beyond!)
                Can merge when 2 or more cuboids have:
                - the same min and max in 2 of the 3 dimensions
                - consecutive values in the third (ie. the min of one is 1 more than the max of the last
                It should be possible to compute this from the loop below...*/
                for x_index in 1..4 {
                    for y_index in 1..4 {
                        for z_index in 1..4 {
                            if (x_index, y_index, z_index) != (2, 2, 2) {
                                let x_diff = x_points[x_index] - x_points[x_index - 1];
                                let y_diff = y_points[y_index] - y_points[y_index - 1];
                                let z_diff = z_points[z_index] - z_points[z_index - 1];
                                if x_diff > 0 && y_diff > 0 && z_diff > 0 {
                                    let box_part = Self {
                                        x_min: x_points[x_index - 1]
                                            + if x_index == 3 { 1 } else { 0 },
                                        x_max: x_points[x_index] - if x_index == 1 { 1 } else { 0 },
                                        y_min: y_points[y_index - 1]
                                            + if y_index == 3 { 1 } else { 0 },
                                        y_max: y_points[y_index] - if y_index == 1 { 1 } else { 0 },
                                        z_min: z_points[z_index - 1]
                                            + if z_index == 3 { 1 } else { 0 },
                                        z_max: z_points[z_index] - if z_index == 1 { 1 } else { 0 },
                                    };
                                    remaining.push(box_part);
                                }
                            }
                        }
                    }
                }

                remaining
            }
        }
    }
}

// represents the area of cubes that are on, as a disjoint union of cuboid areas
struct OnArea(Vec<Cuboid>);

impl OnArea {
    fn new() -> Self {
        OnArea(vec![])
    }

    fn add_cuboid(&mut self, cuboid: Cuboid) {
        // need to get the process started!
        //println!("adding cuboid {:?}", cuboid);
        //println!("starting to add. Currently {:?}", self.0);
        if self.0.is_empty() {
            self.0.push(cuboid);
        } else {
            let mut new = vec![];
            let mut new_parts = vec![];
            //println!("cuboids at the moment: {}", self.0.len());
            for old in &self.0 {
                let (existing, not_existing) = old.intersection(&cuboid);
                //println!("existing: {:?}", existing);
                //println!("not existing: {:?}", not_existing);
                for &part in &existing {
                    if !new.contains(&part) {
                        new.push(part);
                    }
                }
                //new.extend_from_slice(&existing);
                if existing.len() == 1 && not_existing.len() == 1 {
                    // this only happens when the new cuboid doesn't intersect the current one.
                    // (This should be the most common case!)
                    // In this case we just add the "Not existing" (ie new) one to the "new" vector so
                    // that it gets directly included.
                    if !new.contains(&not_existing[0]) {
                        new.push(not_existing[0]);
                    }
                    //new.extend_from_slice(&not_existing);
                    /*for part in not_existing {
                        if !new.contains(&part) {
                            if !self.0.contains(&part) {
                                new.push(part);
                            }
                        }
                    }*/
                } else {
                    //new_parts.extend_from_slice(&not_existing);
                    for part in not_existing {
                        if !new_parts.contains(&part) {
                            //if !self.0.contains(&part) {
                            new_parts.push(part);
                            //}
                        }
                    }
                    //surely won't work!!
                    //new.extend_from_slice(&not_existing);
                }
            }
            /*27/12 - don't think this is a good way to do it. The combination of width and depth
            - new_parts being quite long and generating similarly long new_parts in each of the many
            recursive calls - slows things down far too much.
            I believe this is logically correct, but might need a new way to solve the problem, which is:
            how to cope with pieces that intersect MORE THAN ONE existing piece?
            I have one idea right now, although can't quite seem to code it:
            - for each "not existing" part, go through each existing one (that is, self.0), and
            successively remove those from "it". "it" is in quotes because remove produces several,
            so we have to generate an array/vector and keep doing this with all. That makes it tricky
            to write - and it could hurt performance. But it should avoid any recursive calls, because
            we just add these to "new" and get them added with no recursion!
            Will it always work correctly? Could we not end up with overlapping pieces? Needs more thought...
            first, assume there isn't a problem and just not bother!
            */
            println!(
                "initially {} new parts from broken-up existing ones",
                new.len()
            );
            println!("{} genuinely new parts", new_parts.len());
            let mut temp = OnArea(new_parts);
            temp.tidy_all();
            println!("{} after tidy", temp.0.len());
            for part in &temp.0 {
                for old in &self.0 {
                    let remaining = part.remove(old);
                    for new_part in remaining {
                        if !new.contains(&new_part) {
                            new.push(new_part);
                        }
                    }
                }
            }
            println!("ended up with {}", new.len());
            //let mut left = new_parts.len();
            /*for part in new_new {
                //println!("adding one");
                //println!("adding new part {:?}", new);
                //println!("{} new parts left to add", left);
                // no need to add anything we already have!
                if !new.contains(&part) {
                    new.push(part);
                }
                //left -= 1;
            }*/
            *self = OnArea(new);
        }
    }

    fn remove_cuboid(&mut self, cuboid: Cuboid) {
        let mut new = vec![];
        for old in &self.0 {
            let remaining = old.remove(&cuboid);
            new.extend_from_slice(&remaining);
        }
        *self = OnArea(new);
    }

    fn tidy(&mut self, co_ord: &str) {
        // cleans up a set of cuboids by putting together those with the same bounds in 2
        // dimensions and where they are next to each other in the 3rd
        //println!("tidying from {:?}", self.0);
        self.0.sort_unstable_by(|a, b| match co_ord {
            "x" => {
                let mut cmp = a.y_min.cmp(&b.y_min);
                if cmp == Ordering::Equal {
                    cmp = a.z_min.cmp(&b.z_min);
                    if cmp == Ordering::Equal {
                        cmp = a.x_min.cmp(&b.x_min);
                    }
                }
                cmp
            }
            "y" => {
                let mut cmp = a.z_min.cmp(&b.z_min);
                if cmp == Ordering::Equal {
                    cmp = a.x_min.cmp(&b.x_min);
                    if cmp == Ordering::Equal {
                        cmp = a.y_min.cmp(&b.y_min);
                    }
                }
                cmp
            }
            "z" => {
                let mut cmp = a.x_min.cmp(&b.x_min);
                if cmp == Ordering::Equal {
                    cmp = a.y_min.cmp(&b.y_min);
                    if cmp == Ordering::Equal {
                        cmp = a.z_min.cmp(&b.z_min);
                    }
                }
                cmp
            }
            _ => panic!("blurgh"),
        });
        let mut tidied: Vec<Cuboid> = vec![];
        for cuboid in &self.0 {
            let Cuboid {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            } = cuboid;

            let clone = tidied.clone();
            let found = clone.iter().enumerate().find(|(_, other)| match co_ord {
                "x" => {
                    other.y_min == *y_min
                        && other.z_max == *z_max
                        && other.y_min == *y_min
                        && other.y_max == *y_max
                        && other.x_max == *x_min - 1
                }
                "y" => {
                    other.x_min == *x_min
                        && other.x_max == *x_max
                        && other.z_min == *z_min
                        && other.z_max == *z_max
                        && other.y_max == *y_min - 1
                }
                "z" => {
                    other.x_min == *x_min
                        && other.x_max == *x_max
                        && other.y_min == *y_min
                        && other.y_max == *y_max
                        && other.z_max == *z_min - 1
                }
                _ => panic!("won't happen"),
            });

            match found {
                None => tidied.push(*cuboid),
                Some((pos, to_join)) => {
                    tidied.remove(pos);
                    let combined = Cuboid {
                        x_min: if co_ord == "x" { to_join.x_min } else { *x_min },
                        x_max: *x_max,
                        y_min: if co_ord == "y" { to_join.y_min } else { *y_min },
                        y_max: *y_max,
                        z_min: if co_ord == "z" { to_join.z_min } else { *z_min },
                        z_max: *z_max,
                    };
                    tidied.push(combined);
                }
            }
        }
        *self = OnArea(tidied);
        //println!("result is {:?}", self.0);
    }

    fn tidy_all(&mut self) {
        self.tidy("z");
        self.tidy("y");
        self.tidy("x");
        //do second time, for reasons
        self.tidy("z");
        self.tidy("y");
        self.tidy("x");
    }

    fn process_step(&mut self, step: Step) {
        println!("processing step {:?}", step);
        //println!("starting from: {:?}", self.0);
        let cuboid = Cuboid::from_step(&step);
        match step.state {
            CubeState::On => {
                self.add_cuboid(cuboid);
            }
            CubeState::Off => {
                self.remove_cuboid(cuboid);
            }
        }
        self.tidy_all();
        println!("now have {} cuboids", self.0.len());
        //println!("{:?}", self.0);
    }

    fn process_all(&mut self, steps: Steps) {
        let mut count = 1;
        for step in steps.0 {
            println!("doing step {}", count);
            self.process_step(step);
            /*if count > 5 {
                break;
            }*/
            count += 1;
        }
    }

    fn count_cubes(&self) -> u64 {
        self.0.iter().map(|cuboid| cuboid.volume()).sum()
    }
}

fn parse_state(word: &str) -> CubeState {
    match word {
        "on" => CubeState::On,
        "off" => CubeState::Off,
        _ => panic!("unexpected cube state instruction: {}", word),
    }
}

fn parse_step(line: &str) -> Step {
    let step_parts: Vec<&str> = line.split(" ").collect();
    let state = parse_state(step_parts[0]);
    let coord_parts: Vec<&str> = step_parts[1].split(",").collect();

    let mut numbers = vec![];
    for part in coord_parts {
        let ends = part.split("=").collect::<Vec<&str>>()[1];
        let parts: Vec<isize> = ends.split("..").map(|num| num.parse().unwrap()).collect();
        numbers.extend_from_slice(&parts);
    }
    let x_min = numbers[0];
    let x_max = numbers[1];
    let y_min = numbers[2];
    let y_max = numbers[3];
    let z_min = numbers[4];
    let z_max = numbers[5];
    Step {
        state,
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
    }
}

fn read_file() -> Steps {
    let mut file = File::open("./input/input22.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let steps = contents.lines().map(parse_step).collect();
    Steps(steps)
}

fn solve_part_1(steps: Steps) -> usize {
    let map = steps.process(-50, 50);
    map.keys().count()
}

pub fn part_1() -> usize {
    let steps = read_file();
    solve_part_1(steps)
}

fn solve_part_2(steps: Steps) -> u64 {
    let mut on = OnArea::new();
    on.process_all(steps);
    on.count_cubes()
}

pub fn part_2() -> u64 {
    let steps = read_file();
    solve_part_2(steps)
}
