use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Room {
    name: String,
}

impl Room {
    fn is_large(&self) -> bool {
        // names are either all upper or all lower so just test first character
        // for uppercase
        self.name.chars().nth(0).unwrap().is_uppercase()
    }
}

struct Connection {
    rooms: [Room; 2],
}

#[derive(PartialEq, Clone)]
struct Path {
    path: Vec<Room>,
    // property needed for part 2
    visited_small_twice: bool,
}

// for my own debugging - but useful enough to keep in!
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.path
                .iter()
                .map(|room| room.name.to_owned())
                .collect::<Vec<String>>()
                .join("-")
        )
    }
}

impl Path {
    // tests a room against a path to see if it's valid
    fn is_valid(&self, room: &Room) -> bool {
        let mut is_valid = true;
        let is_small = !room.is_large();
        if is_small {
            // a small room isn't valid if it's been visited already
            if let Some(_) = self.path.iter().find(|&visited| visited == room) {
                is_valid = false;
            }
        }
        is_valid
    }

    // adjusted version for part 2.
    // Returns a second bool to indicate if a small room was visited for the first time or
    // not, so that we can update the visited_small_twice property if this happens.
    // (Note, we don't do this in here making &self mutable, as then it would get tricky to
    // filter by this function - the property would be updated at the wrong time!)
    fn is_valid_2(&self, room: &Room) -> (bool, bool) {
        let mut is_valid = true;
        let mut visited_small_again = false;
        let is_small = !room.is_large();
        if is_small {
            // check how many times it's been visited already
            let previous_visits = self.path.iter().filter(|&visited| visited == room).count();
            // check if any small rooms have been visited twice already
            if previous_visits > 1 {
                is_valid = false;
            } else if previous_visits == 1 {
                if self.visited_small_twice || room.name == "start" || room.name == "end" {
                    is_valid = false;
                } else {
                    visited_small_again = true;
                }
            }
        }
        (is_valid, visited_small_again)
    }

    fn add_room(&self, room: Room) -> Self {
        let mut new_path = self.clone();
        new_path.path.push(room);
        new_path
    }
}

struct Map {
    connections: Vec<Connection>,
}

impl Map {
    fn find_connections(&self, room: &Room) -> Vec<&Room> {
        let mut result = vec![];
        for connection in &self.connections {
            if &connection.rooms[0] == room {
                result.push(&connection.rooms[1]);
            }
            if &connection.rooms[1] == room {
                result.push(&connection.rooms[0]);
            }
        }
        result
    }

    // used to cache the results of calling find_connection, so that it doesn't have to be computed
    // at every step of the recursive path-finding algorithm
    fn connections_cached(&self) -> HashMap<Room, Vec<&Room>> {
        let mut connections = HashMap::new();
        for Connection {
            rooms: [start, end],
        } in &self.connections
        {
            for room in [start.clone(), end.clone()] {
                if let None = connections.get(&room) {
                    let connected = self.find_connections(&room);
                    connections.insert(room, connected);
                }
            }
        }
        connections
    }

    fn count_all_paths_recursive(
        &self,
        connections: &HashMap<Room, Vec<&Room>>,
        start: Room,
        target: Room,
        so_far: Path,
        mut count: &mut usize,
    ) -> usize {
        let current_room = so_far.path.iter().last().unwrap();
        let current_path = so_far.clone();
        if current_room == &target {
            // we've reached the target so can just add 1 to the number of paths
            *count += 1;
            // we shouldn't continue any more (as no path, even for part 2, can reach the end twice)
            // The return value is actually irrelevant but we need something of the correct type.
            return 0;
        }
        let can_go_next = connections.get(current_room).unwrap();
        for &next in can_go_next
            .iter()
            .filter(|room| current_path.is_valid(room))
        {
            let extended = current_path.add_room(next.clone());
            // now the recursive call to find the way forward from here
            self.count_all_paths_recursive(
                connections,
                start.clone(),
                target.clone(),
                extended,
                &mut count,
            );
        }
        *count
    }

    // mostly a copy of the previous one, but uses is_valid_2 rather than is_valid,
    // with changes a bit trickier than can be easily done by just passing a function parameter
    fn count_all_paths_recursive_2(
        &self,
        connections: &HashMap<Room, Vec<&Room>>,
        start: Room,
        target: Room,
        so_far: Path,
        mut count: &mut usize,
    ) -> usize {
        let current_room = so_far.path.iter().last().unwrap();
        if current_room == &target {
            // we've reached the target so can just add 1 to the number of paths
            *count += 1;
            // we shouldn't continue any more (as no path, even for part 2, can reach the end twice)
            // The return value is actually irrelevant but we need something of the correct type.
            return 0;
        }
        let can_go_next = connections.get(current_room).unwrap();
        for &next in can_go_next {
            let (is_valid, update_flag) = so_far.is_valid_2(next);
            if is_valid {
                let mut current_path = so_far.clone();
                if update_flag {
                    current_path.visited_small_twice = true;
                }
                let extended = current_path.add_room(next.clone());
                // now the recursive call to find the way forward from here
                self.count_all_paths_recursive_2(
                    connections,
                    start.clone(),
                    target.clone(),
                    extended,
                    &mut count,
                );
            }
        }
        *count
    }

    fn count_paths(&self, start: Room, end: Room) -> usize {
        let connections = self.connections_cached();
        self.count_all_paths_recursive(
            &connections,
            start.clone(),
            end,
            Path {
                path: vec![start],
                visited_small_twice: false,
            },
            &mut 0,
        )
    }

    fn count_paths_2(&self, start: Room, end: Room) -> usize {
        let connections = self.connections_cached();
        self.count_all_paths_recursive_2(
            &connections,
            start.clone(),
            end,
            Path {
                path: vec![start],
                visited_small_twice: false,
            },
            &mut 0,
        )
    }
}

fn parse_room(room: &str) -> Room {
    let name = room.to_owned();
    Room { name }
}

fn parse_line(line: &str) -> Connection {
    let rooms = line
        .split("-")
        .map(parse_room)
        .collect::<Vec<Room>>()
        .try_into()
        .unwrap();
    Connection { rooms }
}

fn read_file() -> Map {
    let mut file = File::open("./input/input12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let connections = contents.lines().map(parse_line).collect();
    Map { connections }
}

fn solve_part_1(map: Map) -> usize {
    map.count_paths(
        Room {
            name: String::from("start"),
        },
        Room {
            name: String::from("end"),
        },
    )
}

fn solve_part_2(map: Map) -> usize {
    map.count_paths_2(
        Room {
            name: String::from("start"),
        },
        Room {
            name: String::from("end"),
        },
    )
}

pub fn part_1() -> usize {
    let map = read_file();
    solve_part_1(map)
}

pub fn part_2() -> usize {
    let map = read_file();
    solve_part_2(map)
}
