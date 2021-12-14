use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CharPair {
    char1: char,
    char2: char,
}

// for debugging
impl fmt::Debug for CharPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}{}", self.char1, self.char2))
    }
}

impl CharPair {
    fn new(char1: char, char2: char) -> CharPair {
        CharPair { char1, char2 }
    }
}

struct Rule {
    input: CharPair,
    output: char,
}

// for debugging
impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}->{}", self.input, self.output))
    }
}

struct Polymer {
    template: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct PolymerContents {
    contents: HashMap<CharPair, usize>,
    first_pair: CharPair,
    last_pair: CharPair,
}

impl Polymer {
    fn get_contents(&self) -> PolymerContents {
        PolymerContents::new(&self.template)
    }
}

impl PolymerContents {
    fn new(s: &str) -> PolymerContents {
        let mut contents = HashMap::new();
        let mut first_pair = None;
        let mut last_pair = None;
        let total_chars = s.len();
        let mut current_index = 0;
        for (char1, char2) in s.chars().zip(s.chars().skip(1)) {
            let pair = CharPair::new(char1, char2);
            if current_index == 0 {
                first_pair = Some(pair);
            }
            if current_index == total_chars - 2 {
                last_pair = Some(pair);
            }
            let value = contents.entry(pair).or_insert(0);
            *value += 1;
            current_index += 1;
        }
        let first_pair = first_pair.unwrap();
        let last_pair = last_pair.unwrap();
        PolymerContents {
            contents,
            first_pair,
            last_pair,
        }
    }

    fn apply_rules(&mut self, rules: &Vec<Rule>) {
        let mut new_contents = HashMap::new();
        let mut new_first = self.first_pair;
        let mut new_last = self.last_pair;
        for (&pair, old_count) in &self.contents {
            let mut applied_rule = false;
            for rule in rules {
                if rule.input == pair {
                    let new_pair_1 = CharPair::new(pair.char1, rule.output);
                    let count = new_contents.entry(new_pair_1).or_insert(0);
                    *count += old_count;
                    if pair == self.first_pair {
                        new_first = new_pair_1;
                    }

                    let new_pair_2 = CharPair::new(rule.output, pair.char2);
                    let count = new_contents.entry(new_pair_2).or_insert(0);
                    *count += old_count;
                    if pair == self.last_pair {
                        new_last = new_pair_2;
                    }

                    applied_rule = true;
                    break;
                }
            }
            if !applied_rule {
                let count = new_contents.entry(pair).or_insert(0);
                *count += 1;
            }
        }
        self.contents = new_contents;
        self.first_pair = new_first;
        self.last_pair = new_last;
    }

    fn apply_rules_times(&mut self, rules: &Vec<Rule>, times: usize) {
        for _ in 0..times {
            self.apply_rules(rules);
        }
    }

    fn count_chars(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for (pair, count) in &self.contents {
            let CharPair { char1, char2 } = pair;
            let current_count_1 = counts.entry(*char1).or_insert(0);
            *current_count_1 += count;
            // note that we do NOT usually count the second character,
            // or that will be consistently double-counted.
            // To get the counts right, we must just add 1 for the second character of the LAST pair
            if pair == &self.last_pair {
                let current_count_2 = counts.entry(*char2).or_insert(0);
                *current_count_2 += 1;
            }
        }
        counts
    }

    fn get_difference(&self) -> usize {
        let counts = self.count_chars();
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        max - min
    }
}

fn parse_rule(s: &str) -> Rule {
    let parts: Vec<&str> = s.split(" -> ").collect();
    let output = parts[1].chars().nth(0).unwrap();
    let first_part: Vec<char> = parts[0].chars().collect();
    let char1 = first_part[0];
    let char2 = first_part[1];
    let input = CharPair { char1, char2 };
    Rule { input, output }
}

fn read_file() -> Polymer {
    let mut file = File::open("./input/input14.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let template = contents.lines().nth(0).unwrap().to_owned();
    let rules = contents.lines().skip(2).map(parse_rule).collect();
    Polymer { template, rules }
}

fn solve_part_1(polymer: &mut Polymer) -> usize {
    let mut contents = polymer.get_contents();
    contents.apply_rules_times(&polymer.rules, 10);
    contents.get_difference()
}

fn solve_part_2(polymer: &mut Polymer) -> usize {
    let mut contents = polymer.get_contents();
    contents.apply_rules_times(&polymer.rules, 40);
    contents.get_difference()
}

pub fn part_1() -> usize {
    let mut polymer = read_file();
    solve_part_1(&mut polymer)
}

pub fn part_2() -> usize {
    let mut polymer = read_file();
    solve_part_2(&mut polymer)
}
