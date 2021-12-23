use std::cmp::{max, min};
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
                    let leftover = self.remove(&intersection);
                    //println!("intersection in {:?}", intersection);
                    //println!("left over: {:?}", leftover);
                    (leftover, vec![intersection])
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
        println!("adding cuboid {:?}", cuboid);
        //println!("starting to add. Currently {:?}", self.0);
        if self.0.is_empty() {
            self.0.push(cuboid);
        } else {
            let mut new = vec![];
            let mut new_parts = vec![];
            for old in &self.0 {
                let (existing, not_existing) = old.intersection(&cuboid);
                new.extend_from_slice(&existing);
                if existing.len() == 1 {
                    // this only happens when the new cuboid is distinct from any existing ones.
                    // In this case, rather than building a list of "new" pieces (which leads to infinite recursion)
                    // we just add the existing one directly.
                    new.extend_from_slice(&not_existing);
                } else {
                    //println!("new parts to add: {:?}", not_existing);
                    //new_parts.extend_from_slice(&not_existing);
                    /*for part in not_existing {
                        if !new_parts.contains(&part) {
                            new_parts.push(part);
                        }
                    }*/
                    //surely won't work!!
                    new.extend_from_slice(&not_existing);
                }
            }
            *self = OnArea(new);
            //println!("new parts: {:?}", new_parts);
            println!("{} new parts to add", new_parts.len());
            for new in new_parts {
                println!("adding new part {:?}", new);
                self.add_cuboid(new);
            }
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

    fn process_step(&mut self, step: Step) {
        //println!("current area: {:?}", self.0);
        //println!("processing step {:?}", step);
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
        //println!("result: {:?}", self.0);
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
