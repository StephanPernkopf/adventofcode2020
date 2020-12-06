use std::{collections::HashMap, iter::FromIterator};
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
    nr_of_persons: usize,
    answers: HashMap<char, usize>,
}

impl GroupAnswer {
    fn new(all_answers: Vec<char>, nr_of_persons: usize) -> GroupAnswer {
        let mut answers = HashMap::new();
        for c in all_answers {
            if !answers.contains_key(&c) {
                answers.insert(c, 0);
            }

            *answers.get_mut(&c).unwrap() += 1;
        }

        GroupAnswer {
            nr_of_persons,
            answers
        }
    }

    fn get_mutual_answers(&self) -> usize {
        self.answers.values().filter(|a| **a == self.nr_of_persons).count()
    }
}

fn parse_input(input: Vec<String>) -> Vec<GroupAnswer> {
    let mut group_answers: Vec<GroupAnswer> = Vec::new();
    let mut curr_answers: Vec<char> = Vec::new();
    let mut curr_nr_of_persons: usize = 0;

    for line in input {
        if line.is_empty() {
            group_answers.push(GroupAnswer::new(curr_answers, curr_nr_of_persons));
            curr_answers = Vec::new();
            curr_nr_of_persons = 0;
            continue;
        }
        curr_answers.extend(line.chars());
        curr_nr_of_persons += 1;
    }

    if !curr_answers.is_empty() {
        group_answers.push(GroupAnswer::new(curr_answers, curr_nr_of_persons));
    }

    group_answers
}

fn main() {
    let input: Vec<String> = get_input("./input");
    let group_answers = parse_input(input);

    let result: usize = group_answers
        .iter()
        .map(GroupAnswer::get_mutual_answers)
        .sum();

    println!("Result: {}", result);
}
