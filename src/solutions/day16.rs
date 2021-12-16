use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}

#[derive(Debug, Clone)]
enum PacketContent {
    Literal(usize),
    Children(Vec<Box<Packet>>),
}

fn to_binary(hex: &str) -> String {
    let mut res = String::new();
    for char in hex.chars() {
        let binary = match char {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            c => panic!("invalid hex character: {}", c),
        };
        res.push_str(binary);
    }
    res
}

// returns both the decoded integer and any portion of the string left over
// - as do all the similar methods below
fn parse_literal(binary: &str) -> (usize, String) {
    let char_vec: Vec<char> = binary.chars().collect();
    let chunks = char_vec.chunks(5);
    let mut actual_binary = String::new();
    let mut char_count = 0;
    for chunk in chunks {
        let first = chunk[0];
        let four_digits: String = chunk[1..5].to_owned().into_iter().collect();
        actual_binary.push_str(&four_digits);
        char_count += 5;
        if first == '0' {
            break;
        }
    }
    let num = usize::from_str_radix(&actual_binary, 2).unwrap();
    let remaining = &binary[char_count..];
    (num, remaining.to_owned())
}

fn parse_children_bitlength(binary: &str) -> (Vec<Box<Packet>>, String) {
    let length = usize::from_str_radix(&binary[0..15], 2).unwrap();
    let to_parse = &binary[15..];
    let mut currently_parsing = to_parse.clone();
    let mut chars_parsed = 0;
    let mut packets_parsed = vec![];
    let mut remaining;
    while chars_parsed < length {
        let length_to_parse = currently_parsing.len();
        let parse_result = parse_packet(currently_parsing.to_owned());
        let packet = parse_result.0;
        remaining = parse_result.1;
        chars_parsed += length_to_parse - remaining.len();
        currently_parsing = &remaining;
        packets_parsed.push(Box::new(packet));
    }
    if chars_parsed > length {
        panic!("didn't parse the right number of chars!");
    }
    let remaining = if chars_parsed == to_parse.len() {
        String::new()
    } else if chars_parsed < to_parse.len() {
        (&to_parse[chars_parsed..]).to_owned()
    } else {
        panic!("parsed more characters than available?! wtf??")
    };
    (packets_parsed, remaining)
}

fn parse_children_numpackets(binary: &str) -> (Vec<Box<Packet>>, String) {
    let numpackets = usize::from_str_radix(&binary[0..11], 2).unwrap();
    let to_parse = &binary[11..];
    let mut currently_parsing = to_parse;
    let mut chars_parsed = 0;
    let mut packets_parsed = vec![];
    let mut remaining;
    while packets_parsed.len() < numpackets {
        let length_to_parse = currently_parsing.len();
        let parse_result = parse_packet(currently_parsing.to_owned());
        let packet = parse_result.0;
        remaining = parse_result.1;
        chars_parsed += length_to_parse - remaining.len();
        currently_parsing = &remaining;
        packets_parsed.push(Box::new(packet));
    }
    let remaining = if chars_parsed == to_parse.len() {
        String::new()
    } else if chars_parsed < to_parse.len() {
        (&to_parse[chars_parsed..]).to_owned()
    } else {
        panic!("parsed more characters than available?! wtf??")
    };
    (packets_parsed, remaining)
}

fn parse_children(binary: &str) -> (Vec<Box<Packet>>, String) {
    let length_type_id = binary.chars().nth(0).unwrap();
    let remaining = &binary[1..];
    let still_remaining;
    let parsed;
    match length_type_id {
        '0' => {
            let parse_result = parse_children_bitlength(remaining);
            parsed = parse_result.0;
            still_remaining = parse_result.1;
        }
        '1' => {
            let parse_result = parse_children_numpackets(remaining);
            parsed = parse_result.0;
            still_remaining = parse_result.1;
        }
        c => {
            panic!("unexpected binary char: {}", c);
        }
    }
    (parsed, still_remaining)
}

fn parse_packet(binary: String) -> (Packet, String) {
    let version = u8::from_str_radix(&binary[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&binary[3..6], 2).unwrap();
    let rest = &binary[6..];
    let remaining;
    let content = if type_id == 4 {
        let (literal, left) = parse_literal(rest);
        remaining = left;
        PacketContent::Literal(literal)
    } else {
        let (children, left) = parse_children(rest);
        remaining = left;
        PacketContent::Children(children)
    };

    (
        Packet {
            version,
            type_id,
            content,
        },
        remaining,
    )
}

fn parse_hex(hex_str: String) -> (Packet, String) {
    let binary = to_binary(&hex_str);
    parse_packet(binary)
}

fn read_file() -> Packet {
    let mut file = File::open("./input/input16.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parsed = parse_hex(contents);
    let packet = parsed.0;
    let remaining = parsed.1;
    // sanity check of what's remaining
    for leftover in remaining.chars() {
        if leftover != '0' {
            panic!("non-0 character left over! (remaining was {}", remaining);
        }
    }
    packet
}

fn get_version_sum(packet: Packet) -> usize {
    let mut version_sum = packet.version as usize;
    if let PacketContent::Children(children) = packet.content {
        for child in children {
            version_sum += get_version_sum(*child);
        }
    }
    version_sum
}

fn get_value(packet: Packet) -> usize {
    match packet.content {
        PacketContent::Literal(val) => val,
        PacketContent::Children(children) => {
            let values = children.iter().map(|child| get_value(*child.clone()));
            let as_vec: Vec<usize> = values.clone().collect();
            match packet.type_id {
                0 => values.sum(),
                1 => values.product(),
                2 => values.min().unwrap(),
                3 => values.max().unwrap(),
                5 => {
                    if as_vec[0] > as_vec[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if as_vec[0] < as_vec[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if as_vec[0] == as_vec[1] {
                        1
                    } else {
                        0
                    }
                }
                id => panic!("unknown type id: {}", id),
            }
        }
    }
}

pub fn part_1() -> usize {
    let packet = read_file();
    get_version_sum(packet)
}

pub fn part_2() -> usize {
    let packet = read_file();
    get_value(packet)
}
