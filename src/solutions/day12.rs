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
    /*
    The problem remaining with the below is simply the checks on "have we previously found this path",
    when we do find one. In the biggest example, with 226 paths, this isn't so bad (although it still takes
    20-30 seconds), but the real data has well over 1000 paths (possibly tens or hundreds of thousands for all
    I know!), and this makes the algorithm take longer and longer. As well as require us to keep in memory an
    ever-growing array of found paths.
    This could all be avoided by somehow expanding the previous optimisation whereby after finding a path we
    return to the same path for the next search. We need to "remember" ALL paths that have been closed off
    (completely explored).
    Or, probably better - remember those we still have to explore! Not clear how to handle backtracking though.
    The aim is to guarantee that the algorithm will never return the same path more than once. Then things
    will be arranged efficiently, and those expensive O(n^2) checks on "have we found it yet" can be completely
    eliminated.
    BUT HOW???
    */
    fn all_paths_recursive(
        &self,
        connections: &HashMap<Room, Vec<&Room>>,
        start: Room,
        target: Room,
        so_far: Path,
        nexts: Vec<Option<Room>>,
        already_known: Vec<Path>,
    ) -> Vec<Path> {
        //println!("recursive call!");
        println!("currently traversed: {:?}", so_far);
        println!("nexts currently {:?}", nexts);
        println!("paths found: {}", already_known.len());
        assert_eq!(so_far.path.len(), nexts.len());
        //println!("current path has length {:?}", so_far.path.len());
        //println!("paths already found: {:?}", already_known);
        //println!("found {:?} paths so far", already_known.len());
        let mut all_paths = already_known.clone();
        let current_room = so_far.path.iter().last().unwrap();
        let current_path = so_far.clone();
        if current_room == &target {
            // we've reached the target so can just add the current path to the collection- IF
            // it's not there already!
            //if all_paths.contains(&current_path) {
            //    return all_paths;
            //}
            println!("found end room");
            all_paths.push(current_path.clone());
            let mut next_path = nexts.clone();
            next_path.reverse();
            //algorithm - walk forwards along the now reversed "next_path" (which contains options)
            //while doing so, move backwards along the current_path (which should be the same length!)
            //if the first value is a "some", replace the corresponding (last) element of the path with
            //the "next one" (inside the Some)
            //if it's a None, shorten the path, and keep going backwards until a Some is found
            //if ALL are Nones - not sure, is that even possible?
            //should only be so when we've reached the end, so hopefully OK
            //println!("current path is {:?}", current_path);
            //println!("next path is {:?}", next_path);
            //assert_eq!(current_path.path.len(), next_path.len());
            let length = next_path.len();
            let mut next_room = Room {
                name: String::from("SHOULD NOT EXIST"),
            }; // need to initialise with something!
            let mut next_index = 0;
            for i in 0..length {
                if let Some(room) = &next_path[i] {
                    next_index = i;
                    next_room = room.clone();
                    break;
                }
            }
            next_path.reverse();
            let mut next_path = current_path.path[0..(length - next_index - 1)].to_vec();
            next_path.push(next_room);
            println!("next path is {:?}", next_path);
            return self.all_paths_recursive(
                connections,
                start.clone(),
                target,
                Path { path: next_path },
                nexts[0..(length - next_index)].to_vec(),
                all_paths,
            );
        }
        let mut can_go_next: Vec<&&Room> = connections
            .get(current_room)
            .unwrap()
            .iter()
            .filter(|&room| so_far.is_valid(room))
            .collect();
        //.clone();
        // add fake room so that the following loop goes through all rooms
        let fake_room = &&Room {
            name: String::from("FAKE"),
        };
        can_go_next.push(fake_room);
        for (&&next, &&following) in can_go_next.iter().zip(can_go_next.iter().skip(1)) {
            //if so_far.is_valid(next) {
            let mut new_nexts = nexts.clone();
            if following.name == "FAKE" {
                new_nexts.push(None);
            } else {
                new_nexts.push(Some(following.clone()));
            }
            let extended = current_path.add_room(next.clone());
            // now the recursive call to find the way forward from here
            let additional_paths = self.all_paths_recursive(
                connections,
                start.clone(),
                target.clone(),
                extended,
                new_nexts,
                all_paths.clone(),
            );
            for new_path in &additional_paths {
                // add the new path if we haven't already found it
                //if !all_paths.contains(&new_path) {
                all_paths.push(new_path.clone());
                //}
            }
            //}
        }
        all_paths
    }

    fn all_paths(&self, start: Room, end: Room) -> Vec<Path> {
        let connections = self.connections_cached();
        self.all_paths_recursive(
            &connections,
            start.clone(),
            end,
            Path {
                path: vec![start.clone()],
            },
            vec![None],
            vec![],
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
