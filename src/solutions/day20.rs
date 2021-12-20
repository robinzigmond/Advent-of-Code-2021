use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Pixel {
    On,
    Off,
}

struct EnhancementAlgorithm(Vec<Pixel>);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CoOrd {
    x: isize,
    y: isize,
}

impl CoOrd {
    fn new(x: isize, y: isize) -> CoOrd {
        CoOrd { x, y }
    }

    fn get_neighbours(&self) -> Vec<CoOrd> {
        let CoOrd { x, y } = self;
        vec![
            CoOrd::new(x - 1, y - 1),
            CoOrd::new(*x, y - 1),
            CoOrd::new(x + 1, y - 1),
            CoOrd::new(x - 1, *y),
            CoOrd::new(*x, *y),
            CoOrd::new(x + 1, *y),
            CoOrd::new(x - 1, y + 1),
            CoOrd::new(*x, y + 1),
            CoOrd::new(x + 1, y + 1),
        ]
    }
}

struct Image(HashMap<CoOrd, Pixel>);

impl Image {
    fn read_pixel<'a>(&'a self, x: isize, y: isize, default: &'a Pixel) -> &'a Pixel {
        self.0.get(&CoOrd::new(x, y)).unwrap_or(default)
    }
}

struct PuzzleInfo {
    algorithm: EnhancementAlgorithm,
    image: Image,
}

impl PuzzleInfo {
    fn new_value(&self, point: CoOrd, default: Pixel) -> Pixel {
        let neighbours = point.get_neighbours();
        let mut binary_str = String::new();
        for CoOrd { x, y } in neighbours {
            binary_str.push_str(match self.image.read_pixel(x, y, &default) {
                Pixel::On => "1",
                Pixel::Off => "0",
            });
        }
        let index = usize::from_str_radix(&binary_str, 2).unwrap();
        let pixel = self.algorithm.0[index];
        pixel
    }

    fn enhance(&mut self, default: Pixel) {
        let mut new = HashMap::new();
        for &coord in self.image.0.keys() {
            let pixel = &self.new_value(coord, default);
            new.insert(coord, *pixel);
            for neighbour in coord.get_neighbours() {
                if let None = new.get(&neighbour) {
                    let pixel = &self.new_value(neighbour, default);
                    new.insert(neighbour, *pixel);
                }
            }
        }
        self.image = Image(new);
    }

    fn enhance_times(&mut self, times: usize) {
        let mut default = Pixel::Off;
        for _ in 0..times {
            // the "twist" is that we can't assume that all "unaccounted for" pixels are off
            // (by that I mean those pixels all of whose neighbours, as well as themselves, are not
            // in the area covered by the image being enhanced).
            // After an enhancement, they will be whatever is first in the "algorithm".
            // We get round this by passing an extra argument to read_pixel, which specifies the default,
            // and threading it through from here.
            // This "default" will depend on which enhancement step we are on. For the first, it will always
            // be Off. For the subsequent ones, it will be the value of either the 0th or 511th value of the
            // "algorithm", depending on whether the previous one was Off or On. This explains the mutable
            // "default" value here and what we do with it.
            self.enhance(default);
            default = match default {
                Pixel::On => self.algorithm.0[511],
                Pixel::Off => self.algorithm.0[0],
            };
        }
    }
}

fn parse_char(c: char) -> Pixel {
    match c {
        '#' => Pixel::On,
        '.' => Pixel::Off,
        _ => panic!("unexpected char, {}", c),
    }
}

fn parse_algorithm(line: &str) -> EnhancementAlgorithm {
    let pixels = line.chars().map(parse_char).collect();
    EnhancementAlgorithm(pixels)
}

fn parse_image(gridlines: &Vec<&str>) -> Image {
    let mut pixels = HashMap::new();
    for (row, line) in gridlines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            pixels.insert(CoOrd::new(col as isize, row as isize), parse_char(char));
        }
    }
    Image(pixels)
}

fn read_file() -> PuzzleInfo {
    let mut file = File::open("./input/input20.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let algorithm = parse_algorithm(lines[0]);
    let image = parse_image(&lines[2..].to_vec());
    PuzzleInfo { algorithm, image }
}

fn solve_part_1(mut info: PuzzleInfo) -> usize {
    info.enhance_times(2);
    info.image
        .0
        .values()
        .filter(|val| if let Pixel::On = val { true } else { false })
        .count()
}

fn solve_part_2(mut info: PuzzleInfo) -> usize {
    info.enhance_times(50);
    info.image
        .0
        .values()
        .filter(|val| if let Pixel::On = val { true } else { false })
        .count()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

pub fn part_2() -> usize {
    let info = read_file();
    solve_part_2(info)
}
