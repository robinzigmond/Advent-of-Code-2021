use std::fs::File;
use std::io::prelude::*;

struct Crabs {
    crabs: Vec<usize>,
}

impl Crabs {
    fn sort(&mut self) {
        self.crabs.sort_unstable();
    }

    fn get_minimum_point(&mut self) -> usize {
        // after thinking about it for a while, it becomes clear that to mimise the
        // sum fo distances, we need any number that has an equal number of those in the
        // original set on either side of it.
        // This is essentially finding the median - and is exactly the median if there are an
        // odd number of numbers. If even, we need a number (any number) between the two middle
        // ones if it exists. If it doesn't exist, we simply take one of them - although it
        // won't satisfy equal number on either side, it's as close as we can get.
        // We deal with both of those cases by taking the first of the two middle elements,
        // and adding 1. (If they are not the same.)
        self.sort();
        let length = self.crabs.len();
        if length % 2 == 0 {
            let lower_mid = (length / 2) - 1;
            if self.crabs[lower_mid] == self.crabs[lower_mid + 1] {
                self.crabs[lower_mid]
            } else {
                self.crabs[lower_mid] + 1
            }
        } else {
            let midpoint = (length - 1) / 2;
            self.crabs[midpoint]
        }
    }

    fn calculate_fuel(&self, point: usize) -> usize {
        let mut total = 0;
        for crab in &self.crabs {
            total += (*crab as isize - point as isize).abs();
        }
        total as usize
    }

    // gets the minimum value as well as the point to use to calculate it - see last part of function
    // for why!
    fn get_minimum_point_2(&self) -> (usize, usize) {
        // for the second part, rather than finding the median (which, in essence,
        // is what the first part was about), we need (an integer close to) the mean.
        // This gives the minimum because the mean (by definition) has the property
        // that the sum of the differences of all points from it, taken with sign, must be
        // 0. This means that the sum of absolute differences between those points above it
        // and those below must be the same.
        // And if we are at the integer nearest the mean (call this the "mean"),
        // those sums will differ by the minimum possible (in absolute value).
        // But this means that if we are at that point already and move up 1 from it, the
        // total fuel needed according to the new calculation can only increase:
        // - points higher than the new position have their "score" reduced by their difference
        // from the previous position (which was the "mean")
        // - points lower than or equal to the old position have their score increased
        // by their difference to the new position, ie one more than the difference from the "mean".
        // So the increase minus the decrease is definitely positive, because the 1 for each point
        // less than or equal to the "mean" will be greater than the difference in the sums from
        // either side (as the latter is as close to zero as possible)
        // [I'm aware the above isn't 100% rigorous, but it's the best I can do right now!]
        // Of course, the argument is identical for moving down.
        // So in short, if we are at the "mean" - the integer nearest the actual mean - moving up
        // or down one can only increase the total fuel, which means the "mean" must give the minimum.
        // Note, if the actual mean is of the form n.5 - as turns out to be the case - we don't know
        // which side to go from the above, so no matter what the result we test the integers either side.
        let crabs = &self.crabs;
        let total: usize = crabs.iter().sum();
        let number = crabs.len();
        let average = (total as f64) / (number as f64);
        // this will in general not be an integer, and it's not clear which side we need
        // to go to to get the correct answer. (See above. As it happens, it is an odd half-integer, so
        // even if closest is always right - which I suspect it is - that doesn't help us here!)
        // So we "cheat" by taking both sides and seeing which is smaller.
        let rounded_down = average.floor() as usize;
        let rounded_down_answer = self.calculate_fuel_2(rounded_down) as isize;
        let rounded_up = rounded_down + 1;
        let rounded_up_answer = self.calculate_fuel_2(rounded_up) as isize;
        let diff = rounded_up_answer - rounded_down_answer;
        if diff < 0 {
            (rounded_up, rounded_up_answer as usize)
        } else {
            (rounded_down, rounded_down_answer as usize)
        }
    }

    fn calculate_fuel_2(&self, point: usize) -> usize {
        let mut total = 0;
        for crab in &self.crabs {
            let diff = (*crab as isize - point as isize).abs();
            for i in 1..(diff + 1) {
                total += i;
            }
        }
        total as usize
    }
}

fn read_file() -> Crabs {
    let mut file = File::open("./input/input7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let crabs = contents
        .split(",")
        .map(|digit| digit.parse().unwrap())
        .collect();
    Crabs { crabs }
}

fn solve_part_1(crabs: &mut Crabs) -> usize {
    let minimum_point = crabs.get_minimum_point();
    crabs.calculate_fuel(minimum_point)
}

pub fn part_1() -> usize {
    let mut crabs = read_file();
    solve_part_1(&mut crabs)
}

fn solve_part_2(crabs: &mut Crabs) -> usize {
    let (_, answer) = crabs.get_minimum_point_2();
    answer
}

pub fn part_2() -> usize {
    let mut crabs = read_file();
    solve_part_2(&mut crabs)
}
