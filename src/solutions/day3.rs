use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct BinaryNumber {
    bits: [bool; 12],
}

impl BinaryNumber {
    fn to_num(&self) -> usize {
        let mut binary_string = String::from("");
        for bit in self.bits {
            binary_string.push_str(if bit { "1" } else { "0" });
        }
        usize::from_str_radix(&binary_string, 2).unwrap()
    }
}

struct Diagnostic {
    nums: Vec<BinaryNumber>,
}

impl Diagnostic {
    fn most_common(&self, idx: usize) -> bool {
        let target = (&self.nums.len() / 2) + 1;
        let mut one_count = 0;
        let mut zero_count = 0;
        for num in &self.nums {
            let bit = num.bits[idx];
            if bit {
                one_count += 1;
            } else {
                zero_count += 1;
            }
            if one_count >= target {
                return true;
            }
            if zero_count >= target {
                return false;
            }
        }
        // if we get here, both bits are equally common. By convention we return 1 (true),
        // to match with the puzzle instructions
        true
    }

    fn gamma_and_epsilon(&self) -> (usize, usize) {
        let mut gamma_str = String::from("");
        let mut epsilon_str = String::from("");
        for idx in 0..12 {
            let most_common = self.most_common(idx);
            gamma_str.push_str(if most_common { "1" } else { "0" });
            epsilon_str.push_str(if most_common { "0" } else { "1" });
        }
        let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
        let epsilon = usize::from_str_radix(&epsilon_str, 2).unwrap();
        (gamma, epsilon)
    }

    fn filter(&self, idx: usize, most_or_least: bool) -> Self {
        let commonest = self.most_common(idx);
        let wanted = if most_or_least { commonest } else { !commonest };
        let to_keep = self
            .nums
            .iter()
            .filter(|bin| bin.bits[idx] == wanted)
            .map(|num| num.clone())
            .collect();
        Diagnostic { nums: to_keep }
    }

    fn full_filter(&self, most_or_least: bool) -> usize {
        let mut i = 0;
        let mut current = self;
        let mut new: Self;
        loop {
            if current.nums.len() == 1 {
                let remaining = &current.nums[0];
                return remaining.to_num();
            }
            new = current.filter(i, most_or_least);
            current = &new;
            i += 1;
        }
    }
}

fn parse_line(s: &str) -> BinaryNumber {
    let mut bits = [false; 12];
    for (idx, bit) in s.chars().enumerate() {
        match bit {
            '0' => (),
            '1' => bits[idx] = true,
            _ => panic!("Found character that wasn't 0 or 1 in input!"),
        }
    }
    BinaryNumber { bits }
}

fn read_file() -> Diagnostic {
    let mut file = File::open("./input/input3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let nums = contents.lines().map(parse_line).collect();
    Diagnostic { nums }
}

fn solve_part_1(nums: Diagnostic) -> usize {
    let (gamma, epsilon) = nums.gamma_and_epsilon();
    gamma * epsilon
}

fn solve_part_2(nums: Diagnostic) -> usize {
    let oxygen = nums.full_filter(true);
    let co2 = nums.full_filter(false);
    oxygen * co2
}

pub fn part_1() -> usize {
    let nums = read_file();
    solve_part_1(nums)
}

pub fn part_2() -> usize {
    let nums = read_file();
    solve_part_2(nums)
}
