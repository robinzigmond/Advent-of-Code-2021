use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

struct Position {
    horizontal: isize,
    depth: isize,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    fn update(&mut self, command: Command) {
        match command {
            Command::Forward(n) => self.horizontal += n as isize,
            Command::Up(n) => self.depth -= n as isize,
            Command::Down(n) => self.depth += n as isize,
        }
    }
}

fn parse_line(line: &str) -> Command {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    let dir = parts[0];
    let amount: usize = parts[1].parse().unwrap();
    match dir {
        "forward" => Command::Forward(amount),
        "up" => Command::Up(amount),
        "down" => Command::Down(amount),
        _ => panic!("Unexpected command found in input"),
    }
}

fn read_file() -> Vec<Command> {
    let mut file = File::open("./input/input2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|n| parse_line(n)).collect()
}

fn solve_part_1(commands: Vec<Command>) -> isize {
    let mut position = Position::new();
    for &command in commands.iter() {
        position.update(command);
    }
    position.horizontal * position.depth
}

struct Position2 {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Position2 {
    fn new() -> Position2 {
        Position2 {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn update(&mut self, command: Command) {
        match command {
            Command::Forward(n) => {
                self.horizontal += n as isize;
                self.depth += self.aim * n as isize;
            }
            Command::Up(n) => self.aim -= n as isize,
            Command::Down(n) => self.aim += n as isize,
        }
    }
}

fn solve_part_2(commands: Vec<Command>) -> isize {
    let mut position = Position2::new();
    for &command in commands.iter() {
        position.update(command);
    }
    position.horizontal * position.depth
}

pub fn part_1() -> isize {
    let commands = read_file();
    solve_part_1(commands)
}

pub fn part_2() -> isize {
    let commands = read_file();
    solve_part_2(commands)
}
