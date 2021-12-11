use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
struct Octopus {
    energy: u8,
}

impl Octopus {
    // gives the octopus an additional energy, and returns true if it causes a flash
    fn gain_energy(&mut self) -> bool {
        let will_flash = self.energy == 9;
        self.energy += 1;
        will_flash
    }

    fn reset(&mut self) {
        if self.energy > 9 {
            self.energy = 0;
        }
    }
}

struct OctopusGrid {
    grid: Vec<Vec<Octopus>>,
}

impl OctopusGrid {
    fn get_height(&self) -> usize {
        self.grid.len()
    }

    fn get_width(&self) -> usize {
        self.grid[0].len()
    }

    fn grid_size(&self) -> usize {
        self.get_width() * self.get_height()
    }

    fn get_octopus(&self, row: usize, col: usize) -> Octopus {
        self.grid[row][col]
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbouring_positions: Vec<(usize, usize)> = vec![];
        let width = self.get_width();
        let height = self.get_height();
        let has_left_neighbours = col > 0;
        let has_right_neighbours = col < width - 1;
        let has_top_neighbours = row > 0;
        let has_bottom_neighbours = row < height - 1;
        if has_top_neighbours && has_left_neighbours {
            neighbouring_positions.push((row - 1, col - 1));
        }
        if has_top_neighbours {
            neighbouring_positions.push((row - 1, col));
        }
        if has_top_neighbours && has_right_neighbours {
            neighbouring_positions.push((row - 1, col + 1));
        }
        if has_right_neighbours {
            neighbouring_positions.push((row, col + 1));
        }
        if has_right_neighbours && has_bottom_neighbours {
            neighbouring_positions.push((row + 1, col + 1));
        }
        if has_bottom_neighbours {
            neighbouring_positions.push((row + 1, col));
        }
        if has_bottom_neighbours && has_left_neighbours {
            neighbouring_positions.push((row + 1, col - 1));
        }
        if has_left_neighbours {
            neighbouring_positions.push((row, col - 1));
        }
        neighbouring_positions
    }

    // as above, but takes a list of positions and returns positions of all neighbours of any
    // points in the original list.
    // NOTE: repeats ARE allowed, and deliberately - because any neighbour of two positions that
    // flashes will need to be incremented twice (or more, if more than 2 neighbours flashes)
    fn get_all_neighbours(&self, positions: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut all_neighbours = vec![];
        for (row, col) in positions {
            let neighbours = self.get_neighbours(row, col);
            for neighbour in neighbours {
                all_neighbours.push(neighbour);
            }
        }
        all_neighbours
    }

    fn gain_energy(&mut self, row: usize, col: usize) -> bool {
        let mut octopus = self.get_octopus(row, col);
        let has_flashed = octopus.gain_energy();
        self.grid[row][col] = octopus;
        has_flashed
    }

    fn reset(&mut self, row: usize, col: usize) {
        let mut octopus = self.get_octopus(row, col);
        octopus.reset();
        self.grid[row][col] = octopus;
    }

    fn reset_all(&mut self) {
        let width = self.get_width();
        let height = self.get_height();
        for row in 0..height {
            for col in 0..width {
                self.reset(row, col);
            }
        }
    }

    // does a single "gain energy" step for all specified octopuses, and returns
    // a vector of all (row, col) coords where flashes were triggered
    fn gain_energy_all(&mut self, to_gain: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut flashes = vec![];
        for (row, col) in to_gain {
            let flashed = self.gain_energy(row, col);
            if flashed {
                flashes.push((row, col));
            }
        }
        flashes
    }

    // convenience function to get a list of all coords in the grid
    fn get_all_coords(&self) -> Vec<(usize, usize)> {
        let width = self.get_width();
        let height = self.get_height();
        let mut coords = vec![];
        for row in 0..height {
            for col in 0..width {
                coords.push((row, col));
            }
        }
        coords
    }

    // does the full recursive process
    fn do_step(&mut self) -> usize {
        let mut total_flashes = 0;
        let mut to_increase = self.get_all_coords();
        loop {
            let flashes = self.gain_energy_all(to_increase);
            let num_flashes = flashes.len();
            if num_flashes == 0 {
                // no more flashes, so we can read the answer
                break;
            } else {
                total_flashes += num_flashes;
                to_increase = self.get_all_neighbours(flashes);
            }
        }
        self.reset_all();
        total_flashes
    }

    fn do_steps(&mut self, num_steps: usize) -> usize {
        let mut total = 0;
        for _ in 0..num_steps {
            total += self.do_step();
        }
        total
    }

    fn steps_till_all_flash(&mut self) -> usize {
        let size = self.grid_size();
        let mut step_number = 1;
        loop {
            let flashes = self.do_step();
            if flashes == size {
                break step_number;
            }
            step_number += 1;
        }
    }
}

fn parse_digit(c: char) -> Octopus {
    let energy = c.to_string().parse().unwrap();
    Octopus { energy }
}

fn parse_line(line: &str) -> Vec<Octopus> {
    line.chars().map(parse_digit).collect()
}

fn read_file() -> OctopusGrid {
    let mut file = File::open("./input/input11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let grid = contents.lines().map(parse_line).collect();
    OctopusGrid { grid }
}

fn solve_part_1(mut octopuses: OctopusGrid) -> usize {
    octopuses.do_steps(100)
}

pub fn part_1() -> usize {
    let octopuses = read_file();
    solve_part_1(octopuses)
}

fn solve_part_2(mut octopuses: OctopusGrid) -> usize {
    octopuses.steps_till_all_flash()
}

pub fn part_2() -> usize {
    let octopuses = read_file();
    solve_part_2(octopuses)
}
