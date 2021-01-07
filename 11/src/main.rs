use itertools::Itertools;
use std::cmp::{max, min};

mod lib;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    column: usize
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Position {
            row: pos.0,
            column: pos.1
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
            rows: self.rows,
            cols: self.cols,
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

    pub fn get_seat_neighbors(&self, pos: Position) -> Vec<Position> {
        let row = pos.row;
        let column = pos.column;
        let min_row_offset = if row == 0 {0} else {row};// max(0, row - 1);
        let min_col_offset = if column == 0 {0} else {column}; //max(0, column - 1);

        let max_row_offset = min(self.rows, row + 1);
        let max_col_offset = min(self.cols, column + 1);

        (min_row_offset..max_row_offset + 1)
            .cartesian_product(min_col_offset..max_col_offset + 1)
            .map_into()
            .filter(|p| *p != pos)
            .collect()

    }

    // pub fn get_seat_neighbors(&self, row: usize, column: usize) -> Vec<Option<&Tile>> {
    //     let min_row_offset = max(0, row - 1);
    //     let min_col_offset = max(0, column - 1);

    //     let max_row_offset = min(self.rows, row + 1);
    //     let max_col_offset = min(self.cols, column + 1);

    //     let neighbor_iter = (min_row_offset..max_row_offset + 1)
    //         .cartesian_product(min_col_offset..max_col_offset + 1);

    //     neighbor_iter
    //         .filter(|(r, c)| *r != row || *c != column)
    //         .map(|(r, c)| self.get_tile(r, c))
    //         .collect()
    // }
}

fn perform_people_arrival_rules(seats: SeatArea) -> SeatArea {
    let modifiable_seats = seats.clone();

    todo!()
}

fn main() {
    let input: Vec<String> = lib::get_input("input.test");
    let seat_area = SeatArea::new(input);

    let x = seat_area.get_seat_neighbors(Position::from((100, 100)));
    println!("{:?}", x);

    println!("Hello, world!");
}
