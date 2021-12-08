use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

/*
For part 2, for each line we proceed as follows. [WARNING SPOILERS!]
[Putting this up here so it's out of the way and before the relevant code - which is mostly in the impl
block for SegmentData]
First note that each contains at least one of each of the 4 unique lengths.
That means that we know what 1, 4, 7 and 8 are.
Then for each "word" (set of letters representing segments), we compue its "signature". This
is a sequence of 4 integers [W, X, Y, Z] where:
W is the number of its segments which are contained in both the 7 and 8, but not the 1 or 4
X is the number contained in the 4 and 8 but not the 1 or 7
Y is the number contained in all 4 of the 1,4,7,8
Z is the number that are in 8 but not 1,4,7.
It turns out that each of the 9 digits has a unique signature in this sense, even though the totals
W + X + Y + Z are not all unique (which is what we used before).
In summary:
- 0 is [1,1,2,2]
- 1 is [0,0,2,0] (not strictly necessary to know)
- 2 is [1,1,1,2]
- 3 is [1,1,2,1]
- 4 is [0,2,2,0] (not strictly necessary to know)
- 5 is [1,2,1,1]
- 6 is [1,2,1,2]
- 7 is [1,0,2,0]
- 8 is [1,2,2,2]
- 9 is [1,2,2,1]
So we have a clear plan (or algorithm):
1) grab the 1, 4, 7 and 8 (as arrays of segments) by comparing lengths
2) compare and count membership to determine, for each of the segments/letters, which of W-Z they belong to
(here mus be respectively 1, 2, 2 and 2 segmens in W, X, Y, Z so we will make assertions as a sanity check)
3) use this to determine the signature of each word/number in the output (no need for the others)
4) use the above table to convert to numbers
5) add up the resuling 200 4-digit numbers
6) PROFIT
*/

// derive debug seems to be needed for compilation - from the unwrap of try_into
#[derive(Debug, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
struct Display {
    segments: Vec<Segment>,
}

impl Display {
    fn len(&self) -> usize {
        self.segments.len()
    }
}

struct SegmentData {
    input: [Display; 10],
    output: [Display; 4],
}

impl SegmentData {
    fn get_all_displays(&self) -> Vec<&Display> {
        let mut all = vec![];
        for display in &self.input {
            all.push(display);
        }
        for display in &self.output {
            all.push(display);
        }
        all
    }

    fn get_one(&self) -> &Display {
        self.get_all_displays()
            .iter()
            .find(|disp| disp.len() == 2)
            .unwrap()
    }

    fn get_four(&self) -> &Display {
        self.get_all_displays()
            .iter()
            .find(|disp| disp.len() == 4)
            .unwrap()
    }

    fn get_seven(&self) -> &Display {
        self.get_all_displays()
            .iter()
            .find(|disp| disp.len() == 3)
            .unwrap()
    }

    fn get_eight(&self) -> &Display {
        self.get_all_displays()
            .iter()
            .find(|disp| disp.len() == 7)
            .unwrap()
    }

    fn get_sets(&self) -> [Vec<Segment>; 4] {
        let one = self.get_one();
        let four = self.get_four();
        let seven = self.get_seven();
        let eight = self.get_eight();
        let mut set_w = vec![];
        let mut set_x = vec![];
        let mut set_y = vec![];
        let mut set_z = vec![];
        for segment in [
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ] {
            let in_one = one.segments.contains(&segment);
            let in_four = four.segments.contains(&segment);
            let in_seven = seven.segments.contains(&segment);
            let in_eight = eight.segments.contains(&segment);
            if in_seven && in_eight && !in_one && !in_four {
                set_w.push(segment);
            } else if in_four && in_eight && !in_one && !in_seven {
                set_x.push(segment);
            } else if in_one && in_four && in_seven && in_eight {
                set_y.push(segment);
            } else if in_eight && !in_one && !in_four && !in_seven {
                set_z.push(segment);
            } else {
                panic!("Segment didn't fit in any of the 4 categories!");
            }
        }
        assert_eq!(set_w.len(), 1);
        assert_eq!(set_x.len(), 2);
        assert_eq!(set_y.len(), 2);
        assert_eq!(set_z.len(), 2);
        [set_w, set_x, set_y, set_z]
    }

    fn get_signature(&self, word: &Display) -> [usize; 4] {
        let [set_w, set_x, set_y, set_z] = self.get_sets();
        let mut w = 0;
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for segment in &word.segments {
            if set_w.contains(&segment) {
                w += 1;
            } else if set_x.contains(&segment) {
                x += 1;
            } else if set_y.contains(&segment) {
                y += 1;
            } else if set_z.contains(&segment) {
                z += 1;
            }
        }
        [w, x, y, z]
    }

    fn decode_number(&self, word: &Display) -> usize {
        let signature = self.get_signature(word);
        match signature {
            [1, 1, 2, 2] => 0,
            [0, 0, 2, 0] => 1,
            [1, 1, 1, 2] => 2,
            [1, 1, 2, 1] => 3,
            [0, 2, 2, 0] => 4,
            [1, 2, 1, 1] => 5,
            [1, 2, 1, 2] => 6,
            [1, 0, 2, 0] => 7,
            [1, 2, 2, 2] => 8,
            [1, 2, 2, 1] => 9,
            _ => panic!("unrecognised signature"),
        }
    }

    fn get_output_number(&self) -> usize {
        let digits: Vec<usize> = self
            .output
            .iter()
            .map(|word| self.decode_number(word))
            .collect();
        1000 * digits[0] + 100 * digits[1] + 10 * digits[2] + digits[3]
    }
}

struct DisplayData {
    data: Vec<SegmentData>,
}

fn parse_segment(c: &char) -> Segment {
    match c {
        'a' => Segment::A,
        'b' => Segment::B,
        'c' => Segment::C,
        'd' => Segment::D,
        'e' => Segment::E,
        'f' => Segment::F,
        'g' => Segment::G,
        _ => panic!("unexpected character: {}", c),
    }
}

fn parse_display(s: &str) -> Display {
    let segments = s.chars().map(|c| parse_segment(&c)).collect();
    Display { segments }
}

fn parse_displays(s: &str) -> Vec<Display> {
    s.split(" ").map(parse_display).collect()
}

fn parse_line(s: &str) -> SegmentData {
    let parts: Vec<&str> = s.split(" | ").collect();
    let input_str = parts[0];
    let output_str = parts[1];
    let input = parse_displays(input_str).try_into().unwrap();
    let output = parse_displays(output_str).try_into().unwrap();
    SegmentData { input, output }
}

fn read_file() -> DisplayData {
    let mut file = File::open("./input/input8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data = contents.lines().map(parse_line).collect();
    DisplayData { data }
}

fn solve_part_1(data: DisplayData) -> usize {
    // just hard-code the counts we're looking for. There'll be more hardcoding in part 2 anyway!
    let unique_counts = [2, 3, 4, 7];
    data.data
        .iter()
        .map(|SegmentData { output, .. }| {
            output
                .iter()
                .filter(|display| unique_counts.contains(&display.segments.len()))
                .count()
        })
        .sum()
}

fn solve_part_2(data: DisplayData) -> usize {
    data.data
        .iter()
        .map(|display| display.get_output_number())
        .sum()
}

pub fn part_1() -> usize {
    let display_data = read_file();
    solve_part_1(display_data)
}

pub fn part_2() -> usize {
    let display_data = read_file();
    solve_part_2(display_data)
}
