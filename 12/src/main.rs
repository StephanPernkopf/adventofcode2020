mod lib;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
enum Direction {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl From<u16> for Direction {
    fn from(degrees: u16) -> Self {
        let degrees = degrees % 360;

        match degrees {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!("Turn degrees must be multiple of 90"),
        }
    }
}

impl Direction {
    pub fn with_turn(&self, degrees: u16, turn: &Turn) -> Direction {
        let degrees = degrees % 360;
        if degrees % 90 != 0 {
            panic!("Turn degrees must be multiple of 90");
        }

        let self_degrees = *self as u16;
        // avoid underflows -> degrees is smaller than 360
        let self_degrees = self_degrees + 360;

        let result_degrees = match turn {
            Turn::Left => self_degrees - degrees,
            Turn::Right => self_degrees + degrees,
        };

        Direction::from(result_degrees)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Direction(Direction),
    Turn(Turn),
    Forward,
}

struct Command {
    instruction: Instruction,
    units: u16,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let mut iter = s.chars();
        let first = iter.next().unwrap();
        let number = iter.collect::<String>().parse::<u16>().unwrap();

        let instruction = match first {
            'N' => Instruction::Direction(Direction::North),
            'S' => Instruction::Direction(Direction::South),
            'E' => Instruction::Direction(Direction::East),
            'W' => Instruction::Direction(Direction::West),
            'F' => Instruction::Forward,
            'L' => Instruction::Turn(Turn::Left),
            'R' => Instruction::Turn(Turn::Right),
            _ => panic!(),
        };

        Command {
            instruction,
            units: number,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    pub x: i32, // West -> 0 -> East
    pub y: i32, // North -> 0 -> South
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.x == 0 && self.y == 0 {
            return write!(f, "(0, 0)");
        }
        let north_south = if self.y > 0 { "S" } else { "N" };
        let west_east = if self.x > 0 { "E" } else { "W" };

        write!(
            f,
            "({}{}, {}{})",
            self.y.abs(),
            north_south,
            self.x.abs(),
            west_east
        )
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn add_into_direction(&self, dir: &Direction, units: i32) -> Coordinate {
        match dir {
            Direction::North => Coordinate::new(self.x, self.y - units),
            Direction::East => Coordinate::new(self.x + units, self.y),
            Direction::South => Coordinate::new(self.x, self.y + units),
            Direction::West => Coordinate::new(self.x - units, self.y),
        }
    }

    pub fn multiply(&self, waypoint: &Coordinate, times: u16) -> Coordinate {
        let x = waypoint.x * times as i32;
        let y = waypoint.y * times as i32;
        Coordinate::new(self.x + x, self.y + y)
    }
}

fn manhattan_dist(lhs: &Coordinate, rhs: &Coordinate) -> i32 {
    (lhs.x - rhs.x).abs() + (lhs.y - rhs.y).abs()
}

fn _get_last_location(commands: &Vec<Command>, starting_location: &Coordinate) -> Coordinate {
    let mut curr_orientation = Direction::East;
    let mut curr_location = Coordinate::new(starting_location.x, starting_location.y);

    for command in commands {
        match &command.instruction {
            Instruction::Direction(dir) => {
                curr_location = curr_location.add_into_direction(&dir, command.units as i32);
            }
            Instruction::Turn(turn) => {
                curr_orientation = curr_orientation.with_turn(command.units, turn);
            }
            Instruction::Forward => {
                curr_location =
                    curr_location.add_into_direction(&curr_orientation, command.units as i32);
            }
        }
    }

    curr_location
}

fn convert_to_right_rotation_degrees(degrees: u16) -> u16 {
    let degrees = degrees % 360;
    360 - degrees
}

fn rotate_clockwise_around_origin(point: &Coordinate, degrees: u16) -> Coordinate {
    let degrees = degrees % 360;

    match degrees {
        0 => point.clone(),
        90 => Coordinate::new(-point.y, point.x),
        180 => Coordinate::new(-point.x, -point.y),
        270 => Coordinate::new(point.y, -point.x),
        _ => panic!("Turn degrees must be multiple of 90")
    }
}

fn get_last_location_with_waypoint(
    commands: &Vec<Command>,
    ship_starting_location: &Coordinate,
    waypoint_starting_location: &Coordinate,
) -> Coordinate {
    let mut waypoint_location = waypoint_starting_location.clone();
    let mut ship_location = ship_starting_location.clone();

    for command in commands {
        match &command.instruction {
            Instruction::Direction(dir) => {
                waypoint_location =
                    waypoint_location.add_into_direction(&dir, command.units as i32);
            }
            Instruction::Turn(turn) => {
                let degrees = match turn {
                    Turn::Left => convert_to_right_rotation_degrees(command.units),
                    Turn::Right => command.units,
                };

                waypoint_location = rotate_clockwise_around_origin(&waypoint_location, degrees);
            }
            Instruction::Forward => {
                ship_location = ship_location.multiply(&waypoint_location, command.units);
            }
        }
    }

    ship_location
}

fn main() {
    let input = lib::get_input::<String>("input");

    let parsed_input = input.into_iter().map(Command::from).collect::<Vec<_>>();

    let starting_location = Coordinate::new(0, 0);
    let waypoint_starting_location = Coordinate::new(10, -1);
    // let last_location = get_last_location(&parsed_input, &starting_location);
    let last_location = get_last_location_with_waypoint(
        &parsed_input,
        &starting_location,
        &waypoint_starting_location,
    );

    let manhattan_dist = manhattan_dist(&starting_location, &last_location);

    println!("RESULT: {}", manhattan_dist);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_coordinate_display() {
        let c = Coordinate::new(4, 4);
        let print_string = format!("{}", c);
        assert_eq!("(4S, 4E)", print_string);

        let c = Coordinate::new(-4, -4);
        let print_string = format!("{}", c);
        assert_eq!("(4N, 4W)", print_string);

        let c = Coordinate::new(0, 0);
        let print_string = format!("{}", c);
        assert_eq!("(0, 0)", print_string);
    }

    #[test]
    fn test_manhattan_dist() {
        let c1 = Coordinate::new(4, 4);
        let c2 = Coordinate::new(0, 0);

        let dist = manhattan_dist(&c1, &c2);
        assert_eq!(8, dist);

        let c1 = Coordinate::new(17, 8);
        let c2 = Coordinate::new(0, 0);

        let dist = manhattan_dist(&c1, &c2);
        assert_eq!(25, dist);

        let c1 = Coordinate::new(-17, -8);
        let c2 = Coordinate::new(0, 0);

        let dist = manhattan_dist(&c1, &c2);
        assert_eq!(25, dist);

        let c1 = Coordinate::new(0, 0);
        let c2 = Coordinate::new(0, 0);

        let dist = manhattan_dist(&c1, &c2);
        assert_eq!(0, dist);
    }

    #[test]
    fn test_instruction_parsing() {
        let x = String::from("N32");
        let c = Command::from(x);
        assert_eq!(Instruction::Direction(Direction::North), c.instruction);

        let x = String::from("S32");
        let c = Command::from(x);
        assert_eq!(Instruction::Direction(Direction::South), c.instruction);

        let x = String::from("W32");
        let c = Command::from(x);
        assert_eq!(Instruction::Direction(Direction::West), c.instruction);

        let x = String::from("E32");
        let c = Command::from(x);
        assert_eq!(Instruction::Direction(Direction::East), c.instruction);

        let x = String::from("L90");
        let c = Command::from(x);
        assert_eq!(Instruction::Turn(Turn::Left), c.instruction);

        let x = String::from("R90");
        let c = Command::from(x);
        assert_eq!(Instruction::Turn(Turn::Right), c.instruction);

        let x = String::from("F2");
        let c = Command::from(x);
        assert_eq!(Instruction::Forward, c.instruction);
    }

    #[test]
    fn test_direction_with_turn() {
        let d = Direction::North;
        let turn_right = d.with_turn(90, &Turn::Right);
        let turn_left = d.with_turn(90, &Turn::Left);

        assert_eq!(Direction::East, turn_right);
        assert_eq!(Direction::West, turn_left);

        let turn_right = d.with_turn(180, &Turn::Right);
        let turn_left = d.with_turn(180, &Turn::Left);

        assert_eq!(Direction::South, turn_right);
        assert_eq!(Direction::South, turn_left);

        let turn_right = d.with_turn(270, &Turn::Right);
        let turn_left = d.with_turn(270, &Turn::Left);

        assert_eq!(Direction::West, turn_right);
        assert_eq!(Direction::East, turn_left);

        let turn_right = d.with_turn(360, &Turn::Right);
        let turn_left = d.with_turn(360, &Turn::Left);

        assert_eq!(Direction::North, turn_right);
        assert_eq!(Direction::North, turn_left);

        let turn_right = d.with_turn(540, &Turn::Right);
        let turn_left = d.with_turn(540, &Turn::Left);

        assert_eq!(Direction::South, turn_right);
        assert_eq!(Direction::South, turn_left);
    }

    #[test]
    fn test_coordinate_add() {
        let c = Coordinate::new(0, 0);
        assert_eq!(
            Coordinate::new(1, 0),
            c.add_into_direction(&Direction::East, 1)
        );
        assert_eq!(
            Coordinate::new(0, 1),
            c.add_into_direction(&Direction::South, 1)
        );
        assert_eq!(
            Coordinate::new(0, -1),
            c.add_into_direction(&Direction::North, 1)
        );
        assert_eq!(
            Coordinate::new(-1, 0),
            c.add_into_direction(&Direction::West, 1)
        );
    }
}
