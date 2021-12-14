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
}

// for my own debugging - but useful enough to keep in!
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Path: {}",
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

    // note: end is a small cave, so it's not allowed to visit it more than once.
    // So we assume here that we're finished with a path when we reach the "target"
    fn all_paths_recursive(
        &self,
        connections: &HashMap<Room, Vec<&Room>>,
        start: Room,
        target: Room,
        so_far: Path,
        nexts: &Vec<Vec<Room>>,
        already_known: &mut Vec<Path>,
    ) -> Vec<Path> {
        //println!("recursive call!");
        //println!("current path has length {:?}", so_far.path.len());
        //println!("found {:?} paths so far", already_known.len());
        let current_room = so_far.path.iter().last().unwrap();
        let mut current_path = so_far.clone();
        let mut new_nexts = nexts.clone();
        if current_room == &target {
            // we've reached the target so can just add the current path to the collection
            already_known.push(current_path.clone());
            // need to "reset" so_far by figuring out the next path to take
            new_nexts.reverse();
            let length = new_nexts.len();
            let mut next_room = Room {
                name: String::from("SHOULD NOT EXIST"),
            }; // need to initialise with something!
            let mut next_index = 0;
            for i in 0..length {
                if !new_nexts[i].is_empty() {
                    next_index = i;
                    next_room = new_nexts[i][0].clone();
                    break;
                }
            }
            // if next_index is 0, we have finished
            if next_index > 0 {
                new_nexts.reverse();
                new_nexts = new_nexts[0..(length - next_index - 1)].to_vec();
                //need to push something onto new_nexts here, the "next next".
                //How to get it?? Do we have to carry a bunch more state around in function arguments?
                //probably yes - rather than an options, have an array (vec) of all those still to go
                //(now done it, just here left to change!!!)
                //NOTE: ALSO need to ensure we remove the actual "next" used from the sub-vector it's
                //part of!
                let mut next_path = current_path.path[0..(length - next_index - 1)].to_vec();
                next_path.push(next_room);
                current_path = Path { path: next_path };
            }

            println!("exit found on path {:?}", so_far);
            println!("now {:?} paths have been found", already_known.len());
            println!("nexts to take forward: {:?}", new_nexts);
            println!("next path is {:?}", current_path);
        }
        let can_go_next: Vec<Room> = connections
            .get(current_room)
            .unwrap()
            .iter()
            .map(|&room| room.clone())
            .filter(|room| so_far.is_valid(room))
            .collect();
        //head of loop needs to change, use index version so can slice
        for i in 0..can_go_next.len() {
            let next = can_go_next[i].clone();
            let after = can_go_next[i + 1..].to_vec();
            new_nexts.push(after);
            let extended = current_path.add_room(next.clone());
            // now the recursive call to find the way forward from here

            self.all_paths_recursive(
                connections,
                start.clone(),
                target.clone(),
                extended,
                &new_nexts,
                already_known,
            );
        }
        already_known.to_owned()
    }

    fn all_paths(&self, start: Room, end: Room) -> Vec<Path> {
        let connections = self.connections_cached();
        self.all_paths_recursive(
            &connections,
            start.clone(),
            end,
            Path { path: vec![start] },
            &vec![vec![]],
            &mut vec![],
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
    map.all_paths(
        Room {
            name: String::from("start"),
        },
        Room {
            name: String::from("end"),
        },
    )
    .len()
}

pub fn part_1() -> usize {
    let map = read_file();
    solve_part_1(map)
}
