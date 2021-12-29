use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum CubeState {
    Off,
    On,
}

#[derive(Debug)]
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


/*
General approach to solving part 2:
- go through the list, and add a "cuboid" for each area. Store that with the appropriate off/on state.
- also, at each step, calculate its intersection with EACH previous box and add those to the list.
- Note that this "each previous box" also includes intersections previously calculated
- need to add the appropriate sign/state to each one (see below)
- then just compute volumes of each box and add or subtract each one in list!

Intersections:
- when the new box is "on" and intersects a previous "on", the intersection is "off".
(It's previously been counted twice so needs to be removed.)
- when the new box is "off", DON'T add ANYTHING for the box itself! Just calculate intersections with any
previous "on" boxes and make these new ones "off". Likewise any previous "off"s need to be turned "on" to handle
switched-off intersections correctly
- when the new box is "on" and intersects a previous "off", turn it back "on". 

I don't think this is 100% accurate for all cases, and interestingly enough failed for the test data (although
was very very close - to within just over a million relative to an answer over 10^16). I tried it for the real data
mainly to see if it was still too small or if it was now too big - but to my shock it worked! Don't feel motivated
therefore to spend more time thinking about this :-)
*/

// types used for part 2
#[derive(Clone, Debug)]
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

    fn volume(&self) -> i64 {
        (self.x_max as i64 + 1 - self.x_min as i64)
            * (self.y_max as i64 + 1 - self.y_min as i64)
            * (self.z_max as i64 + 1 - self.z_min as i64)
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
}

// represents all areas we are counting, either on or off
struct Areas(Vec<(Cuboid, CubeState)>);

impl Areas {
    fn count_cubes(&self) -> i64 {
        self.0
            .iter()
            .map(|(cuboid, state)| match state {
                CubeState::On => cuboid.volume(),
                CubeState::Off => -cuboid.volume(),
            })
            .sum()
    }

    fn process_step(&mut self, step: Step) {
        match step.state {
            CubeState::On => {
                for (cuboid, state) in self.0.clone() {
                    if let Some(intersection) = cuboid.intersection_area(&Cuboid::from_step(&step))
                    {
                        let new_state = match state {
                            CubeState::On => CubeState::Off,
                            CubeState::Off => CubeState::On,
                        };
                        self.0.push((intersection, new_state));
                    }
                }
                self.0.push((Cuboid::from_step(&step), CubeState::On));
            }
            CubeState::Off => {
                for (cuboid, state) in self.0.clone() {
                    //essentially the same now - or totally?? simplify after solution, if it works!
                    if let Some(intersection) = cuboid.intersection_area(&Cuboid::from_step(&step))
                    {
                        let new_state = match state {
                            CubeState::On => CubeState::Off,
                            CubeState::Off => CubeState::On,
                        };
                        self.0.push((intersection, new_state));
                    }
                }
            }
        }
    }

    fn process_all(&mut self, steps: Steps) {
        for step in steps.0 {
            self.process_step(step);
        }
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

fn solve_part_2(steps: Steps) -> i64 {
    let mut area = Areas(vec![]);
    area.process_all(steps);
    area.count_cubes()
}

pub fn part_2() -> i64 {
    let steps = read_file();
    solve_part_2(steps)
}
