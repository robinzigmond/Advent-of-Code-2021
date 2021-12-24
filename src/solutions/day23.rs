use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug)]
enum BurrowSpace {
    Corridor(u8), // will take numbers 1-11
    HomeA(u8),    // 1-2
    HomeB(u8),    // 1-2
    HomeC(u8),    // 1-2
    HomeD(u8),    // 1-2
}

// each vec will contain exactly 2 values, but easier to work with when it's a "plain" vector
// rather than an array or tuple
struct Burrow(HashMap<Amphipod, Vec<BurrowSpace>>);

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

*/

fn read_position(col: u8, pos: u8) -> BurrowSpace {
    match col {
        3 => BurrowSpace::HomeA(pos),
        5 => BurrowSpace::HomeB(pos),
        7 => BurrowSpace::HomeC(pos),
        9 => BurrowSpace::HomeD(pos),
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
                positions.entry(amphipod).or_insert(vec![]).push(position);
            }
        }
    }

    Burrow(positions)
}

fn solve_part_1(burrow: Burrow) -> usize {
    //TODO
    println!("{:?}", burrow.0);
    0
}

pub fn part_1() -> usize {
    let burrow = read_file();
    solve_part_1(burrow)
}
