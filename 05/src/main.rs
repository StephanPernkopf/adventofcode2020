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

fn get_input<T>(file: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    // unwrap is acceptable because of known input
    read_lines(file)
        .unwrap()
        .map(|line| line.unwrap().parse::<T>().unwrap())
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct SeatIndex(u32, u32);

#[derive(Debug)]
struct Range(u32, u32);

enum TakeType {
    Upper,
    Lower,
}

impl From<&char> for TakeType {
    fn from(c: &char) -> Self {
        match c {
            'F' | 'L' => TakeType::Lower,
            'B' | 'R' => TakeType::Upper,
            _ => panic!()
        }
    }
}

fn perform_binary_partition(range: &Range, partition_take: &TakeType) -> Range {
    let middle = ((range.1 - range.0) / 2) + range.0;

    match partition_take {
        TakeType::Upper => Range(middle + 1, range.1),
        TakeType::Lower => Range(range.0, middle),
    }
}

fn calc(definition: &[char], init_range: Range) -> u32 {
    let mut r: Range = init_range;

    let definition = definition
        .iter()
        .map(|c| TakeType::from(c))
        .collect::<Vec<_>>();

    for take_type in &definition {
        r = perform_binary_partition(&r, take_type);
    }

    r.0
}

fn calc_row(row_definition: &[char]) -> u32 {
    if row_definition.len() != 7 {
        panic!()
    }

    calc(&row_definition, Range(0, 127))
}

fn calc_column(column_definition: &[char]) -> u32 {
    if column_definition.len() != 3 {
        panic!()
    }

    calc(&column_definition, Range(0, 7))
}

fn calc_seat_index(boarding_pass: &Vec<char>) -> SeatIndex {
    let row_definition = &boarding_pass[0..7];
    let column_definition = &boarding_pass[7..];

    return SeatIndex(calc_row(row_definition), calc_column(column_definition));
}

fn calc_seat_id(seat_index: SeatIndex) -> u32 {
    seat_index.0 * 8 + seat_index.1
}

fn main() {
    let input: Vec<String> = get_input("./input");
    let input = input
        .iter()
        .map(|boarding_pass| boarding_pass.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = input.iter().map(calc_seat_index).map(calc_seat_id).collect::<Vec<_>>();
    result.sort();

    for i in 0..result.len() - 1 {
        if result[i] + 1 != result[i + 1] {
            println!("{}, ", result[i] + 1);
        }
    }
}
