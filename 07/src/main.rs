use std::{collections::HashSet, io::{self, BufRead}, thread::current};
use std::{fmt::Debug, fs::File, path::Path, str::FromStr};

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
    inner_bags: Vec<(u32, BagDefinition)>,
}

impl BagDefinition {
    pub fn new(color: String) -> BagDefinition {
        BagDefinition::with_inner_bags(color, Vec::new())
    }

    pub fn with_inner_bags(color: String, inner_bags: Vec<(u32, BagDefinition)>) -> BagDefinition {
        BagDefinition { color, inner_bags }
    }
}

fn parse_inner_bag_definition(input: &str) -> Option<(u32, BagDefinition)> {
    let mut input = input.trim().split(' ');
    // input == "no other bags"
    if input.clone().count() == 3 {
        return None;
    }

    let count = input.next().unwrap().parse().unwrap();
    let color = format!("{} {}", input.next().unwrap(), input.next().unwrap());
    let bag = BagDefinition::new(color);

    Some((count, bag))
}

fn parse_child_bag_definitions(input: &str) -> Vec<(u32, BagDefinition)> {
    let bag_definitions = input.strip_suffix('.').unwrap().split(',');

    bag_definitions
        .map(parse_inner_bag_definition)
        .flatten()
        .collect()
}

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
fn parse_bag_definition(bag_definition: String) -> BagDefinition {
    let mut split = bag_definition.splitn(2, " contain ");
    let bag = split.next().unwrap();

    // bag must contain exactly 3 elements --> 2 words color 1 word "bags"
    let mut iter = bag.split(' ');
    let bag_color = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());

    let inner_bags = split.next().unwrap();
    let inner_bags = parse_child_bag_definitions(inner_bags);

    BagDefinition::with_inner_bags(bag_color, inner_bags)
}

fn parse_input(input: Vec<String>) -> Vec<BagDefinition> {
    input.into_iter().map(parse_bag_definition).collect()
}

fn get_curr_parents<'a>(bags: &'a Vec<BagDefinition>, color: &str) -> Vec<&'a str> {
    bags.iter()
        .filter(|bag| {
            bag.inner_bags
                .iter()
                .any(|(_count, inner_bag)| inner_bag.color == color)
        })
        .map(|bag| &bag.color[..])
        .collect()
}

fn get_parent_bags<'a>(bags: Vec<BagDefinition>, wanted_color: String) -> HashSet<String> {
    let mut wanted_colors = HashSet::new();
    let mut curr_colors = vec![&wanted_color[..]];
    let mut tmp_colors = Vec::new();

    while !curr_colors.is_empty() {
        for &curr_color in curr_colors.iter() {
            let mut parents = get_curr_parents(&bags, curr_color);
            // if children of bags contain curr_color
            if !parents.is_empty() {
                wanted_colors.extend(&mut parents.iter().map(|color| String::from(*color)));
                tmp_colors.append(&mut parents);
            }
        }
        curr_colors.clear();
        curr_colors.append(&mut tmp_colors);
    }

    wanted_colors
}

fn count_siblings(bags: &Vec<&BagDefinition>, wanted_color: &str) -> u32 {
    let current_bag = bags
        .iter()
        .filter(|bag| bag.color == wanted_color)
        .next()
        .unwrap();

    if current_bag.inner_bags.is_empty() {
        return 0;
    }

    let mut curr_bag_child_count: u32 = 0;

    for (inner_child_count, child) in current_bag.inner_bags.iter() {
        let inner_siblings = count_siblings(bags, &child.color);
        curr_bag_child_count += if inner_siblings == 0 {
            inner_child_count.clone()
        } else {
            inner_child_count * inner_siblings
        };
    }
    
    // add curr bag
    curr_bag_child_count + 1
}

fn main() {
    let input: Vec<String> = get_input("./input");
    let bag_definitions = parse_input(input);

    // let bags = get_parent_bags(bag_definitions, String::from("shiny gold"));

    // println!("RESULT: {}", bags.len());

    let siblings = count_siblings(&bag_definitions.iter().collect(), "shiny gold");
    // counts top bag as sibling --> to lazy to fix
    let siblings = siblings - 1;

    println!("RESULT: {}", siblings);
}
