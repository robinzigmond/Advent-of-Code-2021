use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// attempting to write my own implementation of Djikstra's algorithm. This has been through many improvements
// but my solution still takes around 45 seconds for part 2, even when compiled in release mode.
// It must be possible to do better, but I'm no longer too inclined to squeeze more performance out of this!

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
    fn new(nums: &Vec<Vec<u8>>, row: usize, col: usize) -> Node {
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
    priority_list: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Djikstra {
    fn new(nums: Vec<Vec<u8>>) -> Djikstra {
        let height = nums.len();
        let width = nums[0].len();
        let mut nodes = HashMap::new();
        let priority_list = vec![];
        for row in 0..height {
            for col in 0..width {
                let node = Node::new(&nums, row, col);
                nodes.insert((row, col), node);
            }
        }
        // starting with empty priority list so we don't have an enormous array to start with.
        // Nodes will be inserted as they are needed (see update_queue method)
        Djikstra {
            nodes,
            priority_list,
            height,
            width,
        }
    }

    fn get_node(&self, row: &usize, col: &usize) -> Node {
        self.nodes.get(&(*row, *col)).unwrap().clone()
    }

    // gets the next node AND removes it from the priority list.
    // (This is more efficient, to keep the list small - the node won't be needed again afterward)
    fn get_next_node(&mut self) -> Node {
        let mut found_node = None;
        for (row, col) in &self.priority_list {
            let node = self.get_node(row, col);
            if !node.visited {
                found_node = Some(node);
                break;
            }
        }
        let actual_node = found_node.unwrap();
        self.priority_list = self
            .priority_list
            .iter()
            .filter(|(row, col)| row != &actual_node.row || col != &actual_node.col)
            .map(|&pair| pair)
            .collect();
        actual_node
    }

    // updates queue by simply re-adding the (row, col) pair at the now-correct point.
    // It DOES NOT remove anything - firstly it does not need to (the newer one has lower priority so
    // is inserted earlier, so will always be find first), secondly the performance-killing memory issue
    // of carrying around an ever-growing array of over 10,000 elements is removed (in the get_next_node
    // method) by removing an item from the queue as soon as it has been used.
    fn update_queue(&mut self, row: &usize, col: &usize, new_distance: usize) {
        let mut index = 0;
        let insert_index = loop {
            if index >= self.priority_list.len() {
                // means we haven't found the node yet. So let's insert it!
                self.priority_list.push((*row, *col));
                return;
            }
            let (old_row, old_col) = self.priority_list[index];
            let distance = self.get_node(&old_row, &old_col).distance_to;
            if distance.is_none() || distance.unwrap() > new_distance {
                // node found here
                break index;
            }
            index += 1;
        };
        // insert the found note into the list at the appropriate index
        self.priority_list.insert(insert_index, (*row, *col));
    }

    fn do_step(&mut self, destination: (usize, usize)) -> Option<usize> {
        let mut node = self.get_next_node();
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
                    self.update_queue(row, col, new_val);
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
        self.priority_list.insert(0, (start_row, start_col));
        let mut res = None;
        while res.is_none() {
            res = self.do_step(destination);
        }
        res.unwrap()
    }
}

fn read_file() -> Vec<Vec<u8>> {
    let mut file = File::open("./input/input15.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_part_1(mut nodes: Djikstra) -> usize {
    let start = (0, 0);
    let destination = (nodes.height - 1, nodes.width - 1);
    nodes.solve(start, destination)
}

pub fn part_1() -> usize {
    let nums = read_file();
    let nodes = Djikstra::new(nums);
    solve_part_1(nodes)
}

fn increment(num: u8) -> u8 {
    if num == 9 {
        1
    } else {
        num + 1
    }
}

fn increment_times(num: u8, times: usize) -> u8 {
    let mut res = num;
    for _ in 0..times {
        res = increment(res);
    }
    res
}

pub fn part_2() -> usize {
    let nums = read_file();
    let height = nums.len();
    let width = nums[0].len();
    let mut more_nums = vec![];
    //println!("building grid");
    for tile_row in 0..5 {
        for row in 0..height {
            let mut new_row = vec![];
            for tile_col in 0..5 {
                for col in 0..width {
                    let reference = nums[row][col];
                    let new_val = increment_times(reference, tile_row + tile_col);
                    new_row.push(new_val);
                }
            }
            more_nums.push(new_row);
            //println!("row #{} done", height * tile_row + row);
        }
    }
    //println!("new grid is {:?}", more_nums);
    //println!("starting to build starting Djikstra structure");
    let nodes = Djikstra::new(more_nums);
    //println!("and finished!");
    solve_part_1(nodes)
}
