use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn perform_challenge(input: Vec<u32>) -> Result<u32, ()> {
    for i in &input {
        for j in &input {
            for k in &input {
                let curr_sum = i + j + k;

                if curr_sum == 2020u32 {
                    return Ok(i * j * k);
                } else if curr_sum > 2020u32 {
                    break;
                }
            }
        }
    }

    todo!()
}

fn main() {
    println!("Hello, world!");

    // unwrap is acceptable because of known input
    let mut lines = read_lines("./input")
        .unwrap()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    lines.sort();
    let result = perform_challenge(lines);
    println!("RESULT: {}", result.unwrap());
}
