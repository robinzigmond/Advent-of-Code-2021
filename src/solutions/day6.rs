use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Fish {
    number_of_each: HashMap<usize, usize>,
}

impl Fish {
    fn new(nums: Vec<usize>) -> Fish {
        let mut number_of_each = HashMap::new();
        for num in nums {
            let val = number_of_each.entry(num).or_insert(0);
            *val += 1;
        }
        Fish { number_of_each }
    }

    fn evolve(&mut self) {
        // reproduction rules are equivalent to the following in this setting:
        // the new value of n is equal to the old value of n + 1 (for n from 0 to 7)
        // there is one exception to the above - the new value of 6 is the sum of the
        // just-calculated value and the previous value of 0
        // the new value of 8 is equal to the old value of 0
        let mut temp_new: HashMap<usize, usize> = HashMap::new();
        for n in 0..8 {
            temp_new.insert(n, *self.number_of_each.get(&(n + 1)).unwrap_or(&0));
        }
        let zeros = *self.number_of_each.get(&0).unwrap_or(&0);
        let six_value = temp_new.entry(6).or_insert(0);
        *six_value += zeros;
        temp_new.insert(8, zeros);
        for n in 0..9 {
            self.number_of_each.insert(n, *temp_new.get(&n).unwrap());
        }
    }

    fn evolve_days(&mut self, days: usize) {
        for _ in 1..(days + 1) {
            self.evolve();
        }
    }

    fn total(&self) -> usize {
        self.number_of_each.values().sum()
    }
}

fn read_file() -> Fish {
    let mut file = File::open("./input/input6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let days = contents
        .split(",")
        .map(|digit| digit.parse().unwrap())
        .collect();
    Fish::new(days)
}

fn solve_part_1(fish: &mut Fish) -> usize {
    fish.evolve_days(80);
    fish.total()
}

pub fn part_1() -> usize {
    let mut fish = read_file();
    solve_part_1(&mut fish)
}

fn solve_part_2(fish: &mut Fish) -> usize {
    fish.evolve_days(256);
    fish.total()
}

pub fn part_2() -> usize {
    let mut fish = read_file();
    solve_part_2(&mut fish)
}
