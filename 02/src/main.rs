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

#[derive(Debug)]
struct PasswordPolicy {
    pub min: u32,
    pub max: u32,
    pub test_char: char,
    pub password: String,
}

impl PasswordPolicy {
    pub fn new(min: u32, max: u32, test_char: char, password: &str) -> PasswordPolicy {
        PasswordPolicy {
            min,
            max,
            test_char,
            password: String::from(password),
        }
    }
}

fn parse_password_policy(line: String) -> PasswordPolicy {
    let mut splits = line.split_ascii_whitespace().collect::<Vec<_>>();
    if splits.len() != 3 {
        panic!("input malformed");
    }

    let password_to_test = splits.pop().unwrap();
    let char = splits.pop().unwrap().trim_end_matches(':');
    let min_max = splits.pop().unwrap();

    let min_max = min_max
        .split('-')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    PasswordPolicy::new(
        min_max[0],
        min_max[1],
        char.chars().next().unwrap(),
        password_to_test,
    )
}

fn parse_input(input: Vec<String>) -> Vec<PasswordPolicy> {
    input.into_iter().map(parse_password_policy).collect()
}

fn is_valid_v1(password_policy: &PasswordPolicy) -> bool {
    let occurrence = password_policy.password.match_indices(password_policy.test_char).count();

    occurrence >= password_policy.min as usize && occurrence <= password_policy.max as usize
}

fn is_valid_v2(password_policy: &PasswordPolicy) -> bool {
    let password = &password_policy.password;
    let test_char = &password_policy.test_char;

    let first_occurence = password.chars().nth((password_policy.min -1) as usize).unwrap();
    let second_occurence = password.chars().nth((password_policy.max -1) as usize).unwrap();


    // passw >= password_policy.min as usize && occurrence <= password_policy.max as usize
    (&first_occurence == test_char) ^ (&second_occurence == test_char)
}

fn main() {
    let input: Vec<String> = get_input::<String>();
    let password_policies = parse_input(input);

    let valid_password_count = password_policies.into_iter().filter(is_valid_v1).count();

    println!("Result_v1: {}", valid_password_count);
    let input: Vec<String> = get_input::<String>();
    let password_policies = parse_input(input);

    let valid_password_count = password_policies.into_iter().filter(is_valid_v2).count();

    println!("Result_v2: {}", valid_password_count);
}
