use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum BurrowSpace {
    Corridor(u8),       // will take numbers 1-11
    Home(Amphipod, u8), // 1-2
}

impl fmt::Debug for BurrowSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BurrowSpace::Corridor(n) => format!("corridor {}", n),
                BurrowSpace::Home(amphipod, n) => format!("home {:?} {}", amphipod, n),
            }
        )
    }
}

#[derive(Clone, PartialEq)]
struct Move {
    start: BurrowSpace,
    end: BurrowSpace,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} to {:?}", self.start, self.end)
    }
}

struct Burrow(HashMap<BurrowSpace, Amphipod>);

impl Burrow {
    fn is_finished(&self) -> bool {
        for amphipod_type in [
            Amphipod::Amber,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
        ] {
            for pos in 1..=2 {
                let home_pos = BurrowSpace::Home(amphipod_type, pos);
                if self.0.get(&home_pos) != Some(&amphipod_type) {
                    return false;
                }
            }
        }
        true
    }

    // determines which amphipod, if any, occupies a space
    // (simply a wraper for the hashmap's get method)
    fn get_occupier(&self, space: &BurrowSpace) -> Option<&Amphipod> {
        self.0.get(space)
    }

    /*
    From the description, there are only 2 "types" of possible move:
    - an amphipod moving out of a room into a corridor position.
    CONSTRAINTS:
    a) can only do this if it hasn't moved yet, and if it's not starting in its destination room
    b) must be able to reach its destination (can't be blocked by other amphipods)
    [in particular, the "second" in a room can't move out until the first one has]
    c) must not be moving to a room "threshold" (corridor positions 3,5,7,9)
    - moving from the corridor into a room
    a) path must be clear (not blocked)
    b) room must be its home room
    c) home room is not currently occupied by any amphipods other than its "mate"
    These rules are encoded in the following method.
    */
    fn get_valid_moves(&self) -> Vec<(Move, usize)> {
        let mut valid = vec![];
        for (position, &amphipod_type) in &self.0 {
            match position {
                BurrowSpace::Corridor(pos) => {
                    // can only move into destination room - determine which space
                    // and check path is free
                    let back_pos = BurrowSpace::Home(amphipod_type, 2);
                    let front_pos = BurrowSpace::Home(amphipod_type, 1);
                    let destination = match self.get_occupier(&back_pos) {
                        None => back_pos,
                        Some(occupier) => {
                            if occupier == &amphipod_type {
                                front_pos
                            } else {
                                continue;
                            }
                        }
                    };
                    let threshold_position = match amphipod_type {
                        Amphipod::Amber => 3,
                        Amphipod::Bronze => 5,
                        Amphipod::Copper => 7,
                        Amphipod::Desert => 9,
                    };
                    let positions = if *pos < threshold_position {
                        (*pos + 1)..=threshold_position
                    } else {
                        threshold_position..=(*pos - 1)
                    };
                    let mut spaces: Vec<BurrowSpace> =
                        positions.map(BurrowSpace::Corridor).collect();
                    if destination == back_pos {
                        spaces.push(front_pos);
                    }
                    spaces.push(destination);
                    if spaces
                        .iter()
                        .all(|space| self.get_occupier(space).is_none())
                    {
                        let valid_move = Move {
                            start: *position,
                            end: destination,
                        };
                        let path_length = spaces.len();
                        let cost = match amphipod_type {
                            Amphipod::Amber => 1,
                            Amphipod::Bronze => 10,
                            Amphipod::Copper => 100,
                            Amphipod::Desert => 1000,
                        };
                        let total_cost = cost * path_length;
                        valid.push((valid_move, total_cost));
                    }
                }
                BurrowSpace::Home(home_type, home_pos) => {
                    // we can't move the amphipod if it's in its correct home and
                    // the other home space has an amphipod of the same type or
                    // is empty
                    if home_type == &amphipod_type {
                        let other_pos = 3 - home_pos;
                        match self.get_occupier(&BurrowSpace::Home(*home_type, other_pos)) {
                            Some(occupying_type) => {
                                if occupying_type == home_type {
                                    continue;
                                }
                            }
                            None => continue,
                        }
                    }
                    // check available spaces in corridor, that aren't thresholds
                    let threshold_position = match home_type {
                        Amphipod::Amber => 3,
                        Amphipod::Bronze => 5,
                        Amphipod::Copper => 7,
                        Amphipod::Desert => 9,
                    };
                    let illegal_destinations = vec![3, 5, 7, 9];
                    let positions = 1..=11;
                    let spaces: Vec<BurrowSpace> = positions
                        .filter(|pos| !illegal_destinations.contains(pos))
                        .map(BurrowSpace::Corridor)
                        .collect();

                    for space in spaces {
                        // can't fail, and no need for sanity check
                        if let BurrowSpace::Corridor(destination) = space {
                            let positions = if destination < threshold_position {
                                destination..=threshold_position
                            } else {
                                threshold_position..=destination
                            };
                            let mut path: Vec<BurrowSpace> =
                                positions.map(|pos| BurrowSpace::Corridor(pos)).collect();

                            if *home_pos == 2 {
                                path.push(BurrowSpace::Home(*home_type, 1));
                            }

                            if path.iter().all(|space| self.get_occupier(space).is_none()) {
                                let valid_move = Move {
                                    start: *position,
                                    end: BurrowSpace::Corridor(destination),
                                };
                                let path_length = path.len();
                                let cost = match amphipod_type {
                                    Amphipod::Amber => 1,
                                    Amphipod::Bronze => 10,
                                    Amphipod::Copper => 100,
                                    Amphipod::Desert => 1000,
                                };
                                let total_cost = cost * path_length;
                                valid.push((valid_move, total_cost));
                            }
                        }
                    }
                }
            }
        }
        valid
    }

    // doesn't mutate so we can do a DFS of all moves without messing things up
    fn do_move(&self, move_: &Move) -> Self {
        let mut new_positions = self.0.clone();
        let amphipod = self.0.get(&move_.start).unwrap();
        new_positions.remove(&move_.start);
        new_positions.insert(move_.end, *amphipod);
        Self(new_positions)
    }

    // now about 40-45 seconds in release mode, for test data.
    // around 2 hours for real data (between 1 and 2 anyway - I left it running...)
    // Not good, but at least it's an answer!
    fn find_solutions_and_cost_recursive(
        &self,
        current_path: Vec<Move>,
        current_cost: usize,
        lowest_cost: Option<usize>,
    ) -> Option<usize> {
        // if current cost is already too high, abandon the search
        if let Some(cost) = lowest_cost {
            if current_cost >= cost {
                return None;
            }
        }

        // first "update" the burrow to the situation after following the "path"
        let mut updated = self;
        let mut next;
        for move_ in &current_path {
            next = updated.do_move(move_);
            updated = &next;
        }

        // if we've finished the current path, check its cost and mark it as new lowest if apprporiate,
        // then (whether lowest or not) exit by returning
        if updated.is_finished() {
            return match lowest_cost {
                None => Some(current_cost),
                Some(old) => {
                    if current_cost < old {
                        println!("found shortest path so far of cost {}", current_cost);
                        Some(current_cost)
                    } else {
                        None
                    }
                }
            };
        }

        // find possible moves from where we are.
        let next_moves = updated.get_valid_moves(/*test*/);

        // If none, exit this path without checking the cost
        if next_moves.is_empty() {
            return None;
        }

        // For each move, add to path and recursively call.
        let mut new_lowest = lowest_cost;
        for (move_, cost) in next_moves {
            let mut new_path = current_path.clone();
            new_path.push(move_);
            let new_cost = current_cost + cost;
            if let Some(lower) =
                self.find_solutions_and_cost_recursive(new_path, new_cost, new_lowest)
            {
                new_lowest = Some(lower);
            }

        }
        // for info while running!
        if current_path.len() <= 3 {
            println!("completely searched path beginning {:?}", current_path);
        }
        new_lowest
    }

    fn find_lowest_cost(&self) -> usize {
        self.find_solutions_and_cost_recursive(vec![], 0, None)
            .unwrap()
    }
}

fn read_position(col: u8, pos: u8) -> BurrowSpace {
    match col {
        3 => BurrowSpace::Home(Amphipod::Amber, pos),
        5 => BurrowSpace::Home(Amphipod::Bronze, pos),
        7 => BurrowSpace::Home(Amphipod::Copper, pos),
        9 => BurrowSpace::Home(Amphipod::Desert, pos),
        _ => panic!("didn't expect to find amphipod at column {}", col),
    }
}

fn parse_amphipod(c: char) -> Option<Amphipod> {
    match c {
        'A' => Some(Amphipod::Amber),
        'B' => Some(Amphipod::Bronze),
        'C' => Some(Amphipod::Copper),
        'D' => Some(Amphipod::Desert),
        '#' => None,
        ' ' => None,
        _ => panic!("unexpected char in file: {}", c),
    }
}

fn read_file() -> Burrow {
    let mut file = File::open("./input/input23.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let third_line = lines[2];
    let fourth_line = lines[3];
    let mut positions = HashMap::new();

    for (idx, line) in vec![third_line, fourth_line].iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(amphipod) = parse_amphipod(c) {
                let is_third = idx == 0;
                let home_pos = if is_third { 1 } else { 2 };
                let position = read_position(col as u8, home_pos);
                positions.insert(position, amphipod);
            }
        }
    }

    Burrow(positions)
}

fn solve_part_1(burrow: Burrow) -> usize {
    burrow.find_lowest_cost()
}

pub fn part_1() -> usize {
    let burrow = read_file();
    solve_part_1(burrow)
}
