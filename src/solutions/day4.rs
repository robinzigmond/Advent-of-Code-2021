use itertools::Itertools;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

struct Bingo {
    board: [[usize; 5]; 5],
    covered: Vec<(usize, usize)>,
    has_won: bool,
}

impl Bingo {
    // takes a number that has just been called, checks it against the given grid
    // and updates the "covered" list if appropriate. If so, it returns the row and column
    // covered, wrapped in Some - otherwise returns None
    fn check_number(&mut self, called: usize) -> Option<(usize, usize)> {
        for row in 0..5 {
            for col in 0..5 {
                let number = self.board[row][col];
                if number == called {
                    self.covered.push((row, col));
                    return Some((row, col));
                }
            }
        }
        None
    }

    // does as check_number, but now the return value indicates if the board has a line
    // (ie "has won") or not
    fn check_win(&mut self, called: usize) -> bool {
        let covered = self.check_number(called);
        if let Some((row, col)) = covered {
            let mut has_won = true;
            // first check row
            for col_to_check in 0..5 {
                if !self.covered.contains(&(row, col_to_check)) {
                    has_won = false;
                    break;
                }
            }
            if has_won {
                self.has_won = true;
                return true;
            }
            // then column
            has_won = true;
            for row_to_check in 0..5 {
                if !self.covered.contains(&(row_to_check, col)) {
                    has_won = false;
                    break;
                }
            }
            if has_won {
                self.has_won = true;
                return true;
            }
        }
        false
    }

    fn sum_uncovered(&self) -> usize {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.covered.contains(&(row, col)) {
                    sum += self.board[row][col];
                }
            }
        }
        sum
    }
}

struct Game {
    numbers: Vec<usize>,
    boards: Vec<Bingo>,
}

impl Game {
    // play the game, finds the winning board and returns the sum of the
    // uncovered numbers in it, multiplied by the last number
    fn play_to_win_first(&mut self) -> usize {
        for &num in &self.numbers {
            for board in &mut self.boards {
                let has_won = board.check_win(num);
                if has_won {
                    return board.sum_uncovered() * num;
                }
            }
        }
        panic!("no winner after all numbers called");
    }

    fn play_to_win_last(&mut self) -> usize {
        let mut remaining = self.boards.len();
        for &num in &self.numbers {
            let one_left = remaining == 1;
            for board in &mut self.boards {
                if !board.has_won {
                    let has_won = board.check_win(num);
                    if has_won {
                        if one_left {
                            return board.sum_uncovered() * num;
                        }
                        remaining -= 1;
                    }
                }
            }
        }
        panic!("couldn't find a last winner!");
    }
}

fn parse_bingo_board(v: &Vec<&str>) -> Bingo {
    let board = v
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[usize; 5]>>()
        .try_into()
        .unwrap();
    Bingo {
        board,
        covered: vec![],
        has_won: false,
    }
}

fn read_file() -> Game {
    let mut file = File::open("./input/input4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parts: Vec<Vec<&str>> = contents
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _)| !k)
        .map(|(_, g)| g.collect())
        .collect();
    let numbers: Vec<usize> = parts[0][0]
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let boards = parts[1..].iter().map(parse_bingo_board).collect();
    Game { numbers, boards }
}

fn solve_part_1(mut game: Game) -> usize {
    game.play_to_win_first()
}

fn solve_part_2(mut game: Game) -> usize {
    game.play_to_win_last()
}

pub fn part_1() -> usize {
    let game = read_file();
    solve_part_1(game)
}

pub fn part_2() -> usize {
    let game = read_file();
    solve_part_2(game)
}
