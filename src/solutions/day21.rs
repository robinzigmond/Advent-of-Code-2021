use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Player {
    Player1,
    Player2,
}

#[derive(Debug)]
struct Game {
    last_die: u8,
    turn: Player,
    scores: HashMap<Player, usize>,
    spaces: HashMap<Player, u8>,
    number_of_rolls: usize,
}

impl Game {
    fn new(p1_space: u8, p2_space: u8) -> Game {
        let mut scores = HashMap::new();
        scores.insert(Player::Player1, 0);
        scores.insert(Player::Player2, 0);

        let mut spaces = HashMap::new();
        spaces.insert(Player::Player1, p1_space);
        spaces.insert(Player::Player2, p2_space);

        Game {
            last_die: 0,
            turn: Player::Player1,
            scores,
            spaces,
            number_of_rolls: 0,
        }
    }

    fn move_and_score(&mut self) {
        let Game { turn: player, .. } = self;
        for _ in 0..3 {
            // roll die and move
            self.number_of_rolls += 1;
            self.last_die += 1;
            if self.last_die > 100 {
                self.last_die -= 100;
            }
            self.spaces.entry(*player).and_modify(|e| {
                *e += self.last_die;
                while *e > 10 {
                    *e -= 10;
                }
            });
        }
        let new_space = self.spaces.get(player).unwrap();
        self.scores
            .entry(*player)
            .and_modify(|e| *e += *new_space as usize);
        self.turn = match self.turn {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    fn play_game(&mut self) {
        let mut p1_score = *self.scores.get(&Player::Player1).unwrap();
        let mut p2_score = *self.scores.get(&Player::Player2).unwrap();
        while p1_score < 1000 && p2_score < 1000 {
            self.move_and_score();
            p1_score = *self.scores.get(&Player::Player1).unwrap();
            p2_score = *self.scores.get(&Player::Player2).unwrap();
        }
    }
}

// alter and simplify to cope with part 2. We deal with just one player at a time,
// and will compute in how many "worlds" they may win in each number of turns.
#[derive(Debug)]
struct PlayerState {
    score: usize,
    space: u8,
}

impl PlayerState {
    fn new(starting_space: u8) -> PlayerState {
        PlayerState {
            score: 0,
            space: starting_space,
        }
    }

    // self is not mutable any more as we need to keep previous states intact as
    // we enumerate all the possibilities
    fn move_and_score(&self, die_roll: u8) -> Self {
        let mut space = self.space;
        space += die_roll;
        while space > 10 {
            space -= 10;
        }
        let mut score = self.score;
        score += space as usize;

        Self { score, space }
    }

    fn has_won(&self) -> bool {
        self.score >= 21
    }

    // conversion based on probabilities of rolling a total of 3 to 9 in 3 rolls of a D3
    fn num_worlds(rolls: &Vec<u8>) -> u64 {
        let mut worlds = 1;
        for roll in rolls {
            worlds *= match roll {
                3 => 1,
                4 => 3,
                5 => 6,
                6 => 7,
                7 => 6,
                8 => 3,
                9 => 1,
                _ => panic!("impossible to roll {} in 3 D3's!", roll),
            };
        }
        worlds
    }

    // recursive function which plays as a single player and figures out in how many "worlds"
    // they win in each possible number of turns. This information is stored in hashmap.
    // (not actually returned, but built up into the mutable argument as we go)
    fn build_worlds(&self, rolls: Vec<u8>, so_far: &mut HashMap<usize, u64>) {
        if self.has_won() {
            let worlds = Self::num_worlds(&rolls);
            let turns_played = rolls.len();
            let running_total = so_far.entry(turns_played).or_insert(0);
            *running_total += worlds;
        } else {
            // if no winner yet, simulate all possible turns and call recursively with each
            for next_roll in 3..10 {
                let mut next_rolls = rolls.clone();
                next_rolls.push(next_roll);
                let updated = self.move_and_score(next_roll);
                updated.build_worlds(next_rolls, so_far);
            }
        }
    }
}

fn read_file() -> Game {
    let mut file = File::open("./input/input21.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let scores: Vec<u8> = contents
        .lines()
        .map(|line| line.chars().last().unwrap().to_string().parse().unwrap())
        .collect();
    Game::new(scores[0], scores[1])
}

fn read_file_2() -> [PlayerState; 2] {
    let mut file = File::open("./input/input21.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .map(|line| line.chars().last().unwrap().to_string().parse().unwrap())
        .map(PlayerState::new)
        .collect::<Vec<PlayerState>>()
        .try_into()
        .unwrap()
}

fn solve_part_1(mut game: Game) -> usize {
    game.play_game();
    let p1_score = *game.scores.get(&Player::Player1).unwrap();
    let loser = if p1_score >= 1000 {
        Player::Player2
    } else {
        Player::Player1
    };
    game.scores.get(&loser).unwrap() * game.number_of_rolls
}

fn solve_part_2(players: [PlayerState; 2]) -> u64 {
    let mut p1_counts = HashMap::new();
    let mut p2_counts = HashMap::new();
    players[0].build_worlds(vec![], &mut p1_counts);
    players[1].build_worlds(vec![], &mut p2_counts);

    let mut p1_wins = 0;
    for (turns, worlds) in p1_counts.iter() {
        // p1 wins in exactly t turns if p2 has failed to win in (t - 1) turns OR LESS
        let mut p2_wins = 0;
        for p2_turns in 1..*turns {
            // in particular, since we are looking at the "world count" for t - 1 moves
            // but the hashmap value is that for a smaller number of turns, we need to multiply
            // by the appropriate power of 27 to get the total world count up to what it should be
            let p2_wins_here = p2_counts.get(&p2_turns).unwrap_or(&0)
                * u64::pow(27, (turns - 1 - p2_turns) as u32);
            p2_wins += p2_wins_here;
        }
        let p2_losses = u64::pow(27, (turns - 1) as u32) - p2_wins;
        p1_wins += worlds * p2_losses;
    }

    let mut p2_wins = 0;
    for (turns, worlds) in p2_counts.iter() {
        // p2 wins in t turns if p2 has failed to win in t turns OR LESS
        let mut p1_wins = 0;
        for p1_turns in 1..=*turns {
            let p1_wins_here =
                p1_counts.get(&p1_turns).unwrap_or(&0) * u64::pow(27, (turns - p1_turns) as u32);
            p1_wins += p1_wins_here;
        }
        let p1_losses = u64::pow(27, *turns as u32) - p1_wins;
        p2_wins += worlds * p1_losses;
    }
    max(p1_wins, p2_wins)
}

pub fn part_1() -> usize {
    let game = read_file();
    solve_part_1(game)
}

pub fn part_2() -> u64 {
    let players = read_file_2();
    solve_part_2(players)
}
