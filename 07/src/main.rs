use std::{fmt::Debug, fs::File, path::Path, str::FromStr};
use std::io::{self, BufRead};

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

struct BagDefinition {
    color: String,
    inner_bag: Vec<(u32, BagDefinition)>
}

fn parse_input(input: Vec<String>) -> Vec<BagDefinition> {
    /* 

    BagDefiniton 
    : Color "bags contain" ContentDefinition
    ;

    ContentDefinition 
    : NoContent
    | MultipleContent* SingleContent
    ;

    SingleContent
    : number Color "bag."
    ;

    MultipleContent
    : number Color "bags,"
    ;

    Color
    : string string
    ;

    NoContent
    : "no other bags"
    ;
    
    */

    for line in input {

    }
    todo!()
}

fn main() {
    let input: Vec<String> = get_input("./input.test");
    let bag_definitions = parse_input(input);
}
