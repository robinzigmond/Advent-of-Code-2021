use std::fs::File;
use std::io::prelude::*;

enum CucumberStatus {
    Right,
    Down,
    None,
}

struct Cucumbers(Vec<Vec<CucumberStatus>>);

impl Cucumbers {
    fn get_width(&self) -> usize {
        self.0[0].len()
    }

    fn get_height(&self) -> usize {
        self.0.len()
    }

    fn read(&self, row: usize, col: usize) -> &CucumberStatus {
        &self.0[row][col]
    }

    fn get_move_position(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let status = self.read(row, col);
        match status {
            &CucumberStatus::Down => {
                let mut new_row = row + 1;
                if new_row == self.get_height() {
                    new_row = 0;
                }
                if let &CucumberStatus::None = self.read(new_row, col) {
                    Some((new_row, col))
                } else {
                    None
                }
            }
            &CucumberStatus::Right => {
                let mut new_col = col + 1;
                if new_col == self.get_width() {
                    new_col = 0;
                }
                if let &CucumberStatus::None = self.read(row, new_col) {
                    Some((row, new_col))
                } else {
                    None
                }
            }
            &CucumberStatus::None => None,
        }
    }

    // return value indicates whether any updates were made or not
    fn step_right(&mut self) -> bool {
        let width = self.get_width();
        let height = self.get_height();
        let mut moved = vec![];
        let mut destinations = vec![];
        for row in 0..height {
            for col in 0..width {
                if let CucumberStatus::Right = self.read(row, col) {
                    if let Some((new_row, new_col)) = self.get_move_position(row, col) {
                        moved.push((row, col));
                        destinations.push((new_row, new_col));
                    }
                }
            }
        }
        for i in 0..(moved.len()) {
            let (old_row, old_col) = moved[i];
            let (new_row, new_col) = destinations[i];
            self.0[old_row][old_col] = CucumberStatus::None;
            self.0[new_row][new_col] = CucumberStatus::Right;
        }
        !moved.is_empty()
    }

    // return value indicates whether any updates were made or not
    fn step_down(&mut self) -> bool {
        let width = self.get_width();
        let height = self.get_height();
        let mut moved = vec![];
        let mut destinations = vec![];
        for row in 0..height {
            for col in 0..width {
                if let CucumberStatus::Down = self.read(row, col) {
                    if let Some((new_row, new_col)) = self.get_move_position(row, col) {
                        moved.push((row, col));
                        destinations.push((new_row, new_col));
                    }
                }
            }
        }
        for i in 0..(moved.len()) {
            let (old_row, old_col) = moved[i];
            let (new_row, new_col) = destinations[i];
            self.0[old_row][old_col] = CucumberStatus::None;
            self.0[new_row][new_col] = CucumberStatus::Down;
        }
        !moved.is_empty()
    }

    fn step_both(&mut self) -> bool {
        let moved_right = self.step_right();
        let moved_down = self.step_down();
        moved_right || moved_down
    }
}

fn parse_cucumber(c: char) -> CucumberStatus {
    match c {
        'v' => CucumberStatus::Down,
        '>' => CucumberStatus::Right,
        '.' => CucumberStatus::None,
        _ => panic!("unexpected character {}", c),
    }
}

fn read_file() -> Cucumbers {
    let mut file = File::open("./input/input25.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let cucumbers = contents
        .lines()
        .map(|line| line.chars().map(parse_cucumber).collect())
        .collect();
    Cucumbers(cucumbers)
}

fn solve_part_1(mut cucumbers: Cucumbers) -> usize {
    let mut steps = 0;
    let mut to_move = true;
    while to_move {
        to_move = cucumbers.step_both();
        steps += 1;
    }
    steps
}

pub fn part_1() -> usize {
    let cucumbers = read_file();
    solve_part_1(cucumbers)
}
