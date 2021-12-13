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
        already_known: &mut Vec<Path>,
    ) -> Vec<Path> {
        //println!("recursive call!");
        println!("currently traversed: {:?}", so_far);
        //println!("current path has length {:?}", so_far.path.len());
        println!("paths already found: {:?}", already_known);
        //println!("found {:?} paths so far", already_known.len());
        let current_room = so_far.path.iter().last().unwrap();
        let current_path = so_far.clone();
        if current_room == &target {
            // we've reached the target so can just add the current path to the collection
            already_known.push(current_path.clone());
            //need to "reset" so_far - but how?
            //Can do either via making mutable like already_known, or by a recursive call -
            //but both ways lead to problems. (Much the same problems conceptually, in fact!)
            //And how do we know what to reset *to*? Tried that before, logic was mostly right
            //but it was the "reset" that caused problems in some cases.
        }
        let can_go_next = connections.get(current_room).unwrap();
        for &next in can_go_next.iter().filter(|room| so_far.is_valid(room)) {
            let extended = current_path.add_room(next.clone());
            // now the recursive call to find the way forward from here
            //need to somehow "look ahead" and find the "next path" to check in sequence,
            //if we happen to be at the end. For most cases it's just the next step of
            //the loop but if we're at the end we need to "unwind".
            //Going to have to go back to earlier approach of "leaving breadcrumbs", then
            //- even though I went back to the mutable approach to try to avoid that after
            //having difficulties!
            self.all_paths_recursive(
                connections,
                start.clone(),
                target.clone(),
                extended,
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
