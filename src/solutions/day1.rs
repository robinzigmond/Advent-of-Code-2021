use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<i32> {
    let mut file = File::open("./input/input1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|n| n.parse().unwrap()).collect()
}

fn solve_part_1(v: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last = v[0];
    for &n in v.iter().skip(1) {
        if n > last {
            count += 1;
        }
        last = n;
    }
    count
}

fn solve_part_2(v: Vec<i32>) -> i32 {
    let mut count = 0;
    for n in 3..(v.len()) {
        let previous = n - 3;
        if v[n] > v[previous] {
            count += 1;
        }
    }
    count
}

pub fn part_1() -> i32 {
    let nums = read_file();
    solve_part_1(nums)
}

pub fn part_2() -> i32 {
    let nums = read_file();
    solve_part_2(nums)
}
