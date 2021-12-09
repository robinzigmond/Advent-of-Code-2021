use std::fs::File;
use std::io::prelude::*;

struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn get_value(&self, row: usize, col: usize) -> u8 {
        self.heights[row][col]
    }

    fn get_height(&self) -> usize {
        self.heights.len()
    }

    fn get_width(&self) -> usize {
        self.heights[0].len()
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let width = self.get_width();
        let height = self.get_height();

        let mut neighbours = vec![];
        if row > 0 {
            neighbours.push((row - 1, col));
        }
        if row < height - 1 {
            neighbours.push((row + 1, col));
        }
        if col > 0 {
            neighbours.push((row, col - 1));
        }
        if col < width - 1 {
            neighbours.push((row, col + 1));
        }
        neighbours
    }

    fn is_low_point(&self, row: usize, col: usize) -> bool {
        let value = self.get_value(row, col);
        let to_check = self.get_neighbours(row, col);
        for (x, y) in to_check {
            let to_compare = self.get_value(x, y);
            if to_compare <= value {
                return false;
            }
        }

        true
    }

    // gets all low points, and returns for each both the row and column coords,
    // and the actual value
    fn get_lows(&self) -> Vec<((usize, usize), u8)> {
        let height = self.get_height();
        let width = self.get_width();
        let mut lows = vec![];
        for row in 0..height {
            for col in 0..width {
                if self.is_low_point(row, col) {
                    lows.push(((row, col), self.get_value(row, col)));
                }
            }
        }
        lows
    }

    // get the basin (again as a vector of pairs of coordinates and value) containing
    // a particular point.
    // This function is the entry point, it uses get_basin_recursive to do the "hard work!"
    fn get_basin(&self, row: usize, col: usize) -> Vec<((usize, usize), u8)> {
        let value = self.get_value(row, col);
        self.get_basin_recursive(row, col, vec![((row, col), value)])
    }

    fn get_basin_recursive(
        &self,
        row: usize,
        col: usize,
        basin: Vec<((usize, usize), u8)>,
    ) -> Vec<((usize, usize), u8)> {
        let mut new_basin = basin.clone();
        let value = self.get_value(row, col);
        let neighbours = self.get_neighbours(row, col);
        for (neighbour_row, neighbour_col) in neighbours {
            let neighbour_value = self.get_value(neighbour_row, neighbour_col);
            // stop if we find a 9, or an equal or lower value
            if neighbour_value != 9 && neighbour_value > value {
                // don't add the point if we've already been here
                if let None = new_basin
                    .iter()
                    .find(|((row_, col_), _)| row_ == &neighbour_row && col_ == &neighbour_col)
                {
                    new_basin.push(((neighbour_row, neighbour_col), neighbour_value));
                }
                new_basin = self.get_basin_recursive(neighbour_row, neighbour_col, new_basin);
            }
        }
        new_basin
    }
}

fn read_file() -> HeightMap {
    let mut file = File::open("./input/input9.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let heights = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    HeightMap { heights }
}

fn solve_part_1(heights: HeightMap) -> usize {
    heights
        .get_lows()
        .iter()
        .map(|((_, _), val)| (val + 1) as usize)
        .sum()
}

fn solve_part_2(heights: HeightMap) -> usize {
    let low_points = heights.get_lows();
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|((row, col), _)| heights.get_basin(*row, *col).len())
        .collect();
    // sort in descending order
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

pub fn part_1() -> usize {
    let height_map = read_file();
    solve_part_1(height_map)
}

pub fn part_2() -> usize {
    let height_map = read_file();
    solve_part_2(height_map)
}
