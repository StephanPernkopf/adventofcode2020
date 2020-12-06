use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;
use std::{
    collections::HashSet,
    io::{self, BufRead},
};
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
struct GroupAnswer {
    answers: HashSet<char>,
}

impl GroupAnswer {
    fn new(answers_with_duplicates: Vec<char>) -> GroupAnswer {
        GroupAnswer {
            answers: HashSet::from_iter(answers_with_duplicates.into_iter()),
        }
    }

    fn get_total_answers(&self) -> usize {
        self.answers.len()
    }
}

fn parse_input(input: Vec<String>) -> Vec<GroupAnswer> {
    let mut group_answers: Vec<GroupAnswer> = Vec::new();
    let mut curr_answers: Vec<char> = Vec::new();

    for line in input {
        if line.is_empty() {
            group_answers.push(GroupAnswer::new(curr_answers));
            curr_answers = Vec::new();
        }
        curr_answers.extend(line.chars());
    }

    if !curr_answers.is_empty() {
        group_answers.push(GroupAnswer::new(curr_answers));
    }

    group_answers
}

fn main() {
    let input: Vec<String> = get_input("./input");
    let group_answers = parse_input(input);

    let result: usize = group_answers
        .iter()
        .map(GroupAnswer::get_total_answers)
        .sum();

    println!("Result: {}", result);
}
