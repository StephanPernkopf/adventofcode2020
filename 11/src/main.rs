use itertools::Itertools;
use std::cmp::min;

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

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        Position { row, column }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Position {
            row: pos.0,
            column: pos.1,
        }
    }
}

struct SeatArea {
    rows: usize,
    cols: usize,
    seats: Vec<Tile>,
}

impl Clone for SeatArea {
    fn clone(&self) -> Self {
        SeatArea {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            seats: self.seats.iter().copied().collect(),
        }
    }
}

impl SeatArea {
    pub fn new(seats: Vec<String>) -> SeatArea {
        SeatArea {
            rows: seats.len(),
            cols: seats[0].len(),
            seats: seats
                .into_iter()
                .map(|row| row.chars().map(|c| Tile::from(c)).collect::<Vec<_>>())
                .flatten()
                .collect::<Vec<_>>(),
        }
    }

    pub fn get_tile(&self, pos: Position) -> Option<&Tile> {
        let row = pos.row;
        let column = pos.column;
        if row >= self.rows || column >= self.cols {
            return None;
        }
        self.seats.get(row * self.cols + column)
    }

    pub fn get_seat_neighbor_positions(&self, pos: Position) -> Vec<Position> {
        let row = pos.row;
        let column = pos.column;
        let min_row_offset = if row == 0 { 0 } else { row }; // max(0, row - 1);
        let min_col_offset = if column == 0 { 0 } else { column }; //max(0, column - 1);

        let max_row_offset = min(self.rows, row + 1);
        let max_col_offset = min(self.cols, column + 1);

        (min_row_offset..max_row_offset + 1)
            .cartesian_product(min_col_offset..max_col_offset + 1)
            .map_into() // map tuples to positions
            .filter(|p| *p != pos)
            .collect()
    }
}
*/

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

fn should_swap(seats: &SeatLayout, row: usize, column: usize) -> bool {
    let neighbors = get_neighbors(seats, row, column);
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
                >= 4
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
