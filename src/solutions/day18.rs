use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

// types used for leaving "breadcrumbs" as to our place in the nested Snailfish structure

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct BreadCrumbs(Vec<Direction>);

impl BreadCrumbs {
    fn push(&mut self, dir: Direction) {
        let dirs = &mut self.0;
        dirs.push(dir);
    }
}

#[derive(Debug, Clone)]
enum SnailfishPart {
    Regular(usize),
    Pair(Box<Snailfish>),
}

#[derive(Debug, Clone)]
struct Snailfish {
    first: SnailfishPart,
    second: SnailfishPart,
}

impl SnailfishPart {
    // helper for finding explosions. Mutually recursive with the one defined for Snailfish
    fn find_explosion_with_nesting(&self, nest_level: usize) -> Option<BreadCrumbs> {
        match self {
            SnailfishPart::Regular(_) => None,
            SnailfishPart::Pair(snailfish) => {
                if nest_level == 4 {
                    Some(BreadCrumbs(vec![]))
                } else {
                    snailfish.find_explosion_with_nesting(nest_level)
                }
            }
        }
    }

    // sees if a split is possible, recursively, on a part. Does the split if possible
    // (on the first such number), and returns a bool indicating if a split was done or not
    fn do_split(&mut self) -> bool {
        match self {
            SnailfishPart::Regular(n) => {
                if *n >= 10 {
                    let first_half = *n / 2;
                    let second_half = *n - first_half;
                    *self = SnailfishPart::Pair(Box::new(Snailfish {
                        first: SnailfishPart::Regular(first_half),
                        second: SnailfishPart::Regular(second_half),
                    }));
                    return true;
                }
            }
            SnailfishPart::Pair(sf) => {
                let first = &mut sf.first;
                let second = &mut sf.second;
                let split_first = first.do_split();
                if split_first {
                    return true;
                }
                return second.do_split();
            }
        };
        false
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailfishPart::Regular(n) => *n,
            SnailfishPart::Pair(boxed_sf) => (*boxed_sf).magnitude(),
        }
    }
}

impl Snailfish {
    // breadcrumb helpers
    fn follow_breadcrumbs(&mut self, breadcrumbs: BreadCrumbs) -> Option<&mut SnailfishPart> {
        let mut current_crumb = breadcrumbs.0;
        let mut current_snailfish = self;
        loop {
            let part = match current_crumb[0] {
                Direction::Left => &mut current_snailfish.first,
                Direction::Right => &mut current_snailfish.second,
            };
            let rest = &current_crumb[1..];
            if rest.is_empty() {
                return Some(part);
            }
            match part {
                SnailfishPart::Regular(_) => {
                    return None;
                }
                SnailfishPart::Pair(sf) => {
                    current_crumb = rest.to_vec();
                    current_snailfish = sf;
                }
            }
        }
    }

    // follows breadcrumbs, expecting to find a regular number. If so, returns a mutable reference
    // to the part containing the number, so this can be updated in place
    fn follow_for_number(&mut self, breadcrumbs: BreadCrumbs) -> Option<&mut SnailfishPart> {
        let mut found = self;
        for (idx, breadcrumb) in breadcrumbs.0.iter().enumerate() {
            let part = match breadcrumb {
                Direction::Left => &mut found.first,
                Direction::Right => &mut found.second,
            };
            match part {
                SnailfishPart::Regular(_) => {
                    let is_end = idx == breadcrumbs.0.len() - 1;
                    if is_end {
                        return Some(part);
                    } else {
                        return None;
                    }
                }
                SnailfishPart::Pair(boxed_sf) => {
                    found = &mut **boxed_sf;
                }
            }
        }
        None
    }

    fn find_number_breadcrumbs_recursive(
        &mut self,
        current_crumb: BreadCrumbs,
        so_far: &mut Vec<BreadCrumbs>,
    ) {
        let found_number = self.follow_for_number(current_crumb.clone());
        match found_number {
            Some(_) => {
                so_far.push(current_crumb);
            }
            None => {
                let mut extend_left = current_crumb.clone();
                extend_left.push(Direction::Left);
                self.find_number_breadcrumbs_recursive(extend_left, so_far);

                let mut extend_right = current_crumb.clone();
                extend_right.push(Direction::Right);
                self.find_number_breadcrumbs_recursive(extend_right, so_far);
            }
        }
    }

    // very important utility method to find ALL sets of breadcrumbs which result in a number
    fn find_number_breadcrumbs(&mut self) -> Vec<BreadCrumbs> {
        let mut found_crumbs = vec![];
        self.find_number_breadcrumbs_recursive(BreadCrumbs(vec![]), &mut found_crumbs);
        found_crumbs
    }

    fn find_right_number_part(&mut self, breadcrumbs: BreadCrumbs) -> Option<&mut SnailfishPart> {
        let all_nums_bc = self.find_number_breadcrumbs();
        for bc in all_nums_bc {
            if bc > breadcrumbs {
                let number = self.follow_for_number(bc);
                // must be a "Some" value by design
                return number;
            }
        }
        None
    }

    fn find_left_number_part(&mut self, breadcrumbs: BreadCrumbs) -> Option<&mut SnailfishPart> {
        let all_nums_bc = self.find_number_breadcrumbs();
        let mut previous = None;
        for bc in all_nums_bc {
            if bc > breadcrumbs {
                match previous {
                    Some(previous) => {
                        let number = self.follow_for_number(previous);
                        // must be a "Some" value by design
                        return number;
                    }
                    // only happens when there are no breadcrumbs from before the one we're testing
                    // that lead to numbers
                    None => return None,
                }
            }
            previous = Some(bc);
        }
        // we only get here if all breadcrumbs that lead to numbers are "before" the given one.
        // In that case we want to follow the last one, and previous must hold it inside a Some
        self.follow_for_number(previous.unwrap())
    }

    // recursive helper for below function
    fn find_explosion_with_nesting(&self, nest_level: usize) -> Option<BreadCrumbs> {
        let Snailfish { first, second } = self;
        match first.find_explosion_with_nesting(nest_level + 1) {
            Some(BreadCrumbs(bcs)) => {
                let mut new_bcs = vec![Direction::Left];
                new_bcs.extend_from_slice(&bcs);
                Some(BreadCrumbs(new_bcs))
            }
            None => match second.find_explosion_with_nesting(nest_level + 1) {
                Some(BreadCrumbs(bcs)) => {
                    let mut new_bcs = vec![Direction::Right];
                    new_bcs.extend_from_slice(&bcs);
                    Some(BreadCrumbs(new_bcs))
                }
                None => None,
            },
        }
    }

    // explodes the first pair that can, if one is found.
    // If one happened, returns the breadcrumbs leading to it - otherwise returns None.
    // Just a simple entry point to the above helper
    fn find_explosion(&self) -> Option<BreadCrumbs> {
        self.find_explosion_with_nesting(0)
    }

    fn do_explosion(&mut self) -> bool {
        let exploded = self.find_explosion();
        match exploded {
            None => false,
            Some(crumbs) => {
                let exploded_part = self.follow_breadcrumbs(crumbs.clone()).unwrap();

                let first_number = match exploded_part {
                    SnailfishPart::Regular(_) => {
                        panic!("exploded on a non-pair? Something's gone very wrong!")
                    }
                    SnailfishPart::Pair(child_sf) => match child_sf.first {
                        SnailfishPart::Regular(n) => n,
                        SnailfishPart::Pair(_) => {
                            panic!("first part of exploded pair is not a number?")
                        }
                    },
                };

                let second_number = match exploded_part {
                    SnailfishPart::Regular(_) => {
                        panic!("exploded on a non-pair? Something's gone very wrong!")
                    }
                    SnailfishPart::Pair(child_sf) => match child_sf.second {
                        SnailfishPart::Regular(n) => n,
                        SnailfishPart::Pair(_) => {
                            panic!("first part of exploded pair is not a number?")
                        }
                    },
                };

                // replace pair with number 0
                *exploded_part = SnailfishPart::Regular(0);

                // replace number on left, if it exists
                let left_number = self.find_left_number_part(crumbs.clone());
                if let Some(part) = left_number {
                    if let SnailfishPart::Regular(n) = part {
                        let new_num = *n + first_number;
                        *part = SnailfishPart::Regular(new_num);
                    } else {
                        panic!("found a number but it's not a number??");
                    }
                }

                // replace number on right, if it exists
                let right_number = self.find_right_number_part(crumbs);
                if let Some(part) = right_number {
                    if let SnailfishPart::Regular(n) = part {
                        let new_num = *n + second_number;
                        *part = SnailfishPart::Regular(new_num);
                    } else {
                        panic!("found a number but it's not a number??");
                    }
                }

                true
            }
        }
    }

    // sees if a split is possible, recursively, on a part. Does the split if possible
    // (on the first such number), and returns a bool indicating if a split was done or not
    fn do_split(&mut self) -> bool {
        let split_first = self.first.do_split();
        if split_first {
            return true;
        }
        self.second.do_split()
    }

    // keeps exploding until no more can be done, and returns a bool indicating if any
    // were done or not
    fn keep_exploding(&mut self) -> bool {
        let mut is_exploding = true;
        let mut any = false;
        while is_exploding {
            is_exploding = self.do_explosion();
            if is_exploding {
                any = true;
            }
        }
        any
    }

    fn reduce(&mut self) {
        let mut is_splitting = true;
        let mut is_exploding = true;
        while is_exploding || is_splitting {
            is_exploding = self.keep_exploding();
            is_splitting = self.do_split();
        }
    }

    fn magnitude(&self) -> usize {
        let left_mag = self.first.magnitude();
        let right_mag = self.second.magnitude();
        3 * left_mag + 2 * right_mag
    }
}

fn add_snailfish(left: Snailfish, right: Snailfish) -> Snailfish {
    let mut added = Snailfish {
        first: SnailfishPart::Pair(Box::new(left)),
        second: SnailfishPart::Pair(Box::new(right)),
    };
    added.reduce();
    added
}

fn add_list(nums: Vec<Snailfish>) -> Snailfish {
    let mut result = nums.clone().into_iter().nth(0).unwrap();
    let rest = nums[1..].to_vec();
    for sf in rest {
        result = add_snailfish(result, sf);
    }
    result
}

fn parse_part(val: &Value) -> SnailfishPart {
    match val {
        Value::Number(n) => SnailfishPart::Regular(n.as_u64().unwrap() as usize),
        Value::Array(_) => SnailfishPart::Pair(Box::new(parse_array(val))),
        _ => panic!("json value is neither an array nor number"),
    }
}

fn parse_array(array: &Value) -> Snailfish {
    if let Value::Array(vec) = array {
        assert_eq!(vec.len(), 2);
        let first = parse_part(&vec[0]);
        let second = parse_part(&vec[1]);
        return Snailfish { first, second };
    }
    panic!("line isn't decodable to an array");
}

fn read_line(line: &str) -> Snailfish {
    let json = serde_json::from_str(line).unwrap();
    parse_array(&json)
}

fn read_file() -> Vec<Snailfish> {
    let mut file = File::open("./input/input18.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(read_line).collect()
}

fn solve_part_1(nums: Vec<Snailfish>) -> usize {
    let sum_sf = add_list(nums);
    sum_sf.magnitude()
}

pub fn part_1() -> usize {
    let pairs = read_file();
    solve_part_1(pairs)
}
