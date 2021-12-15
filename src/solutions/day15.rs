use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// attempting to write my own implementation of Djikstra's algorithm. It works!
// But is not very performant - the main stumbling block is how to implement the priority queue.
// I initially had an array (Vector, in Rust), but that took a long time (more than 3 minutes, even
// when compiled in release mode). The current code uses a Hashmap, to at least run in constant space
// and have fast lookups/insertions - but at the cost, of course, of making it less efficient (O(n))
// to find the lowest-priority node. This has improved the runtime to about 1 minute (when compiled for
// release - "cargo run" takes over 4 minutes) - but that's clearly already slower than intended, and
// will not work well at all for part 2. So I'm looking up better ways to do it!

#[derive(Clone, Debug)]
struct Node {
    value: u8,
    visited: bool,
    connected: Vec<(usize, usize)>,
    distance_to: Option<usize>,
    row: usize,
    col: usize,
}

impl Node {
    fn new(nums: Vec<Vec<u8>>, row: usize, col: usize) -> Node {
        let value = nums[row][col];
        let height = nums.len();
        let width = nums[0].len();
        let mut neighbours = vec![];
        if row > 0 {
            neighbours.push((row - 1, col));
        }
        if row < height - 1 {
            neighbours.push((row + 1, col));
        }
        if col > 0 {
            neighbours.push((row, col - 1));
        }
        if col < width - 1 {
            neighbours.push((row, col + 1));
        }

        Node {
            value,
            visited: false,
            connected: neighbours,
            distance_to: None,
            row,
            col,
        }
    }
}

struct Djikstra {
    nodes: HashMap<(usize, usize), Node>,
    priority_queue: HashMap<(usize, usize), Option<usize>>,
    width: usize,
    height: usize,
}

impl Djikstra {
    fn new(nums: Vec<Vec<u8>>) -> Djikstra {
        let height = nums.len();
        let width = nums[0].len();
        let mut nodes = HashMap::new();
        let mut priority_queue = HashMap::new();
        for row in 0..height {
            for col in 0..width {
                let node = Node::new(nums.clone(), row, col);
                nodes.insert((row, col), node);
                priority_queue.insert((row, col), None);
            }
        }
        Djikstra {
            nodes,
            priority_queue,
            height,
            width,
        }
    }

    fn get_node(&self, row: &usize, col: &usize) -> Node {
        self.nodes.get(&(*row, *col)).unwrap().clone()
    }

    fn get_next_node(&self) -> Node {
        let mut min = None;
        let mut node_coords = None;
        for ((row, col), option_distance) in &self.priority_queue {
            if let Some(dist) = option_distance {
                if min.is_none() || dist < min.unwrap() {
                    let node = self.get_node(row, col);
                    if !node.visited {
                        min = Some(dist);
                        node_coords = Some((row, col));
                    }
                }
            }
        }
        let (row, col) = node_coords.unwrap();
        self.get_node(row, col)
    }

    fn update_queue(&mut self, row: usize, col: usize, new_distance: usize) {
        self.priority_queue.insert((row, col), Some(new_distance));
    }

    fn do_step(&mut self, destination: (usize, usize)) -> Option<usize> {
        let mut node = self.get_next_node();
        //println!("next node: {:?}", node);
        for (row, col) in &node.connected {
            let mut connected_node = self.get_node(&row, &col);
            if !connected_node.visited {
                let new_val = node.distance_to.unwrap() + connected_node.value as usize;
                let update_distance = match connected_node.distance_to {
                    Some(old_distance) => new_val < old_distance,
                    None => true,
                };
                if update_distance {
                    connected_node.distance_to = Some(new_val);
                    self.nodes.insert(
                        (connected_node.row, connected_node.col),
                        connected_node.clone(),
                    );
                    self.update_queue(*row, *col, new_val);
                }
            }
        }
        node.visited = true;
        self.nodes.insert((node.row, node.col), node.clone());
        // check if node is destination. If so, return its distance_to
        if (node.row, node.col) == destination {
            node.distance_to
        } else {
            None
        }
    }

    fn solve(&mut self, start: (usize, usize), destination: (usize, usize)) -> usize {
        let (start_row, start_col) = start;
        let mut starting_node = self.get_node(&start_row, &start_col);
        starting_node.distance_to = Some(0);
        self.nodes.insert(start, starting_node);
        self.priority_queue.insert((start_row, start_col), Some(0));
        let mut res = None;
        let mut count = 1;
        while res.is_none() {
            println!("visiting node #{}", count);
            res = self.do_step(destination);
            count += 1;
        }
        res.unwrap()
    }
}

fn read_file() -> Djikstra {
    let mut file = File::open("./input/input15.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let nums = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    Djikstra::new(nums)
}

fn solve_part_1(mut nodes: Djikstra) -> usize {
    let start = (0, 0);
    let destination = (nodes.height - 1, nodes.width - 1);
    nodes.solve(start, destination)
}

pub fn part_1() -> usize {
    let nodes = read_file();
    solve_part_1(nodes)
}
