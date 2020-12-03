use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::{fmt::Debug, fs::File};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_input<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    // unwrap is acceptable because of known input
    read_lines("./input")
        .unwrap()
        .map(|line| line.unwrap().parse::<T>().unwrap())
        .collect::<Vec<_>>()
}

struct Point {
    x: usize,
    y: usize
}

fn usize_add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

impl Point {
    pub fn with_offset(&self, x: i32, y: i32) -> Point {
        Point {
            x: usize_add(self.x, x).unwrap(),
            y: usize_add(self.y, y).unwrap(),

        }
    }
}

struct Area {
    x_size: usize,
    y_size: usize,
    area: Vec<char>,
}

const TREE_CHAR: char = '#';
const _OPEN_SPACE_CHAR: char = '#';


impl Area {
    pub fn new(input: Vec<String>) -> Area {
        Area {
            x_size: input[0].len(),
            y_size: input.len(),
            area: input.iter().flat_map(|s| s.chars()).collect()
        }
    }

    fn index_fn(&self, point: &Point) -> usize {
        // when x is out of bounds to the right start from the left side again
        let x = point.x % self.x_size;
        point.y * self.x_size + x
    }

    pub fn index_has_tree(&self, point: &Point) -> bool {
        let index = self.index_fn(point);
        self.area[index] == TREE_CHAR
    }

    pub fn finished(&self, point: &Point) -> bool {
        self.y_size <= point.y
    }
}


fn calculate_number_of_trees(input: Vec<String>, offset_x: i32, offset_y: i32) -> usize {
    let area= Area::new(input);

    let mut p: Point = Point{x:0, y:0};
    let mut tree_count: usize = 0;

    while !area.finished(&p) {
        if area.index_has_tree(&p) {
            tree_count += 1;
        }

        p = p.with_offset(offset_x, offset_y);
    }

    tree_count
}

fn main() {
    println!("Hello, world!");

    let input: Vec<String> = get_input::<String>();

    let result = calculate_number_of_trees(input, 3, 1);

    println!("RESULT: {}", result);
}
