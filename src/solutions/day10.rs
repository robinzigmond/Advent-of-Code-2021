use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum BracketType {
    Bracket,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn error_code(&self) -> usize {
        match self {
            BracketType::Bracket => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }

    fn completion_score(&self) -> usize {
        match self {
            BracketType::Bracket => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum BracketDirection {
    Open,
    Closed,
}

#[derive(PartialEq, Copy, Clone)]
struct Character {
    type_: BracketType,
    direction: BracketDirection,
}

impl Character {
    fn is_opener(&self) -> bool {
        self.direction == BracketDirection::Open
    }

    fn find_match(self) -> Self {
        match self {
            Character {
                type_,
                direction: BracketDirection::Open,
            } => Character {
                type_,
                direction: BracketDirection::Closed,
            },
            Character {
                type_,
                direction: BracketDirection::Closed,
            } => Character {
                type_,
                direction: BracketDirection::Open,
            },
        }
    }
}

// represents a chunk in the process of being parsed. It may or may not have been completely parsed
struct Chunk {
    stack: Vec<Character>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk { stack: vec![] }
    }

    fn push_stack(&mut self, char: Character) {
        if !char.is_opener() {
            panic!("can't push closing bracket to stack!");
        }
        self.stack.push(char);
    }

    fn pop_stack(&mut self) -> Option<Character> {
        self.stack.pop()
    }

    fn parse_character(&mut self, char: Character) -> Result<(), usize> {
        if char.is_opener() {
            self.push_stack(char);
            Ok(())
        } else {
            if let Some(open_bracket) = self.pop_stack() {
                let matching = open_bracket.find_match();
                if char == matching {
                    Ok(())
                } else {
                    Err(char.type_.error_code())
                }
            } else {
                Err(char.type_.error_code())
            }
        }
    }
}

struct CodeLine {
    chars: Vec<Character>,
}

// parses the line of code into a chunk. Returns an error code (as in part 1)
// if a syntax error was encountered, or a completion score (as in part 2) if parsed successfully
// but incomplete
impl CodeLine {
    fn parse(&self) -> Result<usize, usize> {
        let mut chunk = Chunk::new();
        for &char in &self.chars {
            if let Err(error_code) = chunk.parse_character(char) {
                return Err(error_code);
            }
        }
        // if we reach here without returning, the whole line must have been successfully parsed.
        // So we can compute the autocomplete score
        let mut total_score = 0;
        let mut stack = chunk.stack.clone();
        stack.reverse();
        for char in stack {
            let score = char.type_.completion_score();
            total_score *= 5;
            total_score += score;
        }
        Ok(total_score)
    }
}

fn parse_char(c: char) -> Character {
    match c {
        '(' => Character {
            type_: BracketType::Bracket,
            direction: BracketDirection::Open,
        },
        ')' => Character {
            type_: BracketType::Bracket,
            direction: BracketDirection::Closed,
        },
        '[' => Character {
            type_: BracketType::Square,
            direction: BracketDirection::Open,
        },
        ']' => Character {
            type_: BracketType::Square,
            direction: BracketDirection::Closed,
        },
        '{' => Character {
            type_: BracketType::Curly,
            direction: BracketDirection::Open,
        },
        '}' => Character {
            type_: BracketType::Curly,
            direction: BracketDirection::Closed,
        },
        '<' => Character {
            type_: BracketType::Angle,
            direction: BracketDirection::Open,
        },
        '>' => Character {
            type_: BracketType::Angle,
            direction: BracketDirection::Closed,
        },
        _ => panic!("unexpected character {}", c),
    }
}

fn parse_line(line: &str) -> CodeLine {
    let chars = line.chars().map(parse_char).collect();
    CodeLine { chars }
}

fn read_file() -> Vec<CodeLine> {
    let mut file = File::open("./input/input10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(parse_line).collect()
}

fn solve_part_1(code: Vec<CodeLine>) -> usize {
    let mut total = 0;
    for line in code {
        if let Err(code) = line.parse() {
            total += code;
        }
    }
    total
}

fn solve_part_2(code: Vec<CodeLine>) -> usize {
    let mut scores = vec![];
    for line in code {
        if let Ok(score) = line.parse() {
            scores.push(score);
        }
    }
    scores.sort_unstable();
    let num_lines = scores.len();
    // we are told num_lines will be odd - let's confirm
    assert_eq!(num_lines % 2, 1);
    let middle_index = (num_lines - 1) / 2;
    scores[middle_index]
}

pub fn part_1() -> usize {
    let code = read_file();
    solve_part_1(code)
}

pub fn part_2() -> usize {
    let code = read_file();
    solve_part_2(code)
}
