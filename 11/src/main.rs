use itertools::Itertools;
use std::{cmp::min, ops::Add};

mod lib;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Tile {
    pub fn swap(&mut self) {
        *self = match self {
            Tile::Floor => panic!(),
            Tile::EmptySeat => Tile::OccupiedSeat,
            Tile::OccupiedSeat => Tile::EmptySeat,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Floor,
            'L' => Tile::EmptySeat,
            '#' => Tile::OccupiedSeat,
            _ => panic!("unknown input"),
        }
    }
}

type SeatLayout = Vec<Vec<Tile>>;

fn parse_input(input: Vec<String>) -> SeatLayout {
    let mut result: Vec<Vec<Tile>> = Vec::new();

    for line in input {
        result.push(line.chars().map_into::<Tile>().collect::<Vec<_>>());
    }

    result
}

fn get_neighbors(seats: &SeatLayout, row: usize, column: usize) -> Vec<Tile> {
    let min_row_offset = if row == 0 { 0 } else { row - 1 }; // max(0, row - 1);
    let min_col_offset = if column == 0 { 0 } else { column - 1 }; //max(0, column - 1);

    let max_row_offset = min(seats.len() - 1, row + 1);
    let max_col_offset = min(seats[0].len() - 1, column + 1);

    // + 1 is here because max-part of range is exclusive
    let max_row_offset = max_row_offset + 1;
    let max_col_offset = max_col_offset + 1;

    (min_row_offset..max_row_offset)
        .cartesian_product(min_col_offset..max_col_offset)
        .filter(|(r, c)| !(*r == row && *c == column))
        .map(|(r, c)| seats[r][c])
        .collect()
}
//[] const OFFSETS: Vec<(i32, i32)> = vec!

const BASE_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), // top left
    (0, -1),  // top
    (-1, 1),  // top right
    (0, 1),   // right
    (1, 1),   // bottom right
    (1, 0),   // bottom
    (1, -1),  // bottom left
    (0, -1),  // left
];

fn is_out_of_bounds(seats: &SeatLayout, offset: &(i32, i32)) -> bool {
    let row = offset.0;
    let col = offset.1;

    row < 0 || row >= seats.len() as i32 || col < 0 || col >= seats[0].len() as i32
}

fn add_tuples(lhs: &(i32, i32), rhs: &(i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn get_neighbors_part_2(seats: &SeatLayout, row: usize, column: usize) -> Vec<Tile> {
    let mut offsets = BASE_OFFSETS
        .iter()
        .copied()
        .map(|t| Some(t))
        .collect::<Vec<_>>();
    let mut result: Vec<Tile> = Vec::new();

    while offsets.iter().any(|o| o.is_some()) {
        for (index, mut offset) in offsets.iter().enumerate() {
            if let Some(o) = offset {
                // eliminate out of bounds offset
                if is_out_of_bounds(seats, o) {
                    offset = &None;
                    continue;
                }

                // test offset
                let curr_row = row + o.0 as usize;
                let curr_col = column + o.1 as usize;
                let curr_tile = seats[curr_row][curr_col];

                match curr_tile {
                    Tile::EmptySeat | Tile::OccupiedSeat => {
                        result.push(curr_tile);
                        offset = &None;
                        continue;
                    }
                    Tile::Floor => {}
                }

                // increment offset
                offset = &Some(add_tuples(o, &BASE_OFFSETS[index]));
            }
        }
    }

    result
}

fn should_swap(seats: &SeatLayout, row: usize, column: usize) -> bool {
    let neighbors = get_neighbors_part_2(seats, row, column);
    let curr_seat = &seats[row][column];

    match curr_seat {
        Tile::EmptySeat => {
            neighbors
                .iter()
                .filter(|t| **t == Tile::OccupiedSeat)
                .count()
                == 0
        }
        Tile::OccupiedSeat => {
            neighbors
                .iter()
                .filter(|t| **t == Tile::OccupiedSeat)
                .count()
                >= 5
        }
        Tile::Floor => false,
    }
}

fn perform_people_arrival_rules(seats: SeatLayout) -> SeatLayout {
    let mut modifiable_seats = seats.clone();

    for i in 0..seats.len() {
        for j in 0..seats[0].len() {
            if should_swap(&seats, i, j) {
                modifiable_seats[i][j].swap();
            }
        }
    }

    let different_seat_count = seats
        .iter()
        .flatten()
        .zip(modifiable_seats.iter().flatten())
        .filter(|(left, right)| **left != **right)
        .count();

    if different_seat_count != 0 {
        return perform_people_arrival_rules(modifiable_seats);
    }

    return modifiable_seats;
}

fn print_seats(seats: &SeatLayout) {
    println!();
    for i in 0..seats.len() {
        for j in 0..seats[0].len() {
            let curr = seats[i][j];
            let printable_char = match curr {
                Tile::Floor => '.',
                Tile::EmptySeat => 'L',
                Tile::OccupiedSeat => '#',
            };
            print!("{}", printable_char);
        }
        println!()
    }
}

fn main() {
    let input: Vec<String> = lib::get_input("input");
    let input = parse_input(input);
    let calculated = perform_people_arrival_rules(input);
    let result = calculated
        .iter()
        .flatten()
        .filter(|t| **t == Tile::OccupiedSeat)
        .count();
    // let seat_area = SeatArea::new(input);

    // let x = seat_area.get_seat_neighbor_positions(Position::new(0, 0));
    // println!("{:?}", calculated);
    print_seats(&calculated);

    println!("RESULT: {}", result);
}
