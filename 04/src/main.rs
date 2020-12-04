use std::path::Path;
use std::str::FromStr;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};
use std::{fmt::Debug, fs::File};
use regex::Regex;

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

trait Valid {
    fn is_valid(&self) -> bool;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum FieldType {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl FromStr for FieldType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field_type = match s {
            "byr" => FieldType::BirthYear,
            "iyr" => FieldType::IssueYear,
            "eyr" => FieldType::ExpirationYear,
            "hgt" => FieldType::Height,
            "hcl" => FieldType::HairColor,
            "ecl" => FieldType::EyeColor,
            "pid" => FieldType::PassportID,
            "cid" => FieldType::CountryID,
            _ => return Err(()),
        };

        Ok(field_type)
    }
}

struct PassportField {
    field_type: FieldType,
    field: String,
}

impl PassportField {
    pub fn new(field_type: &str, field: &str) -> PassportField {
        if field_type.is_empty() || field.is_empty() {
            panic!("empty field_type '{}' or field '{}'", field_type, field)
        }
        PassportField {
            field_type: FieldType::from_str(field_type).expect("Couldn't parse field string"),
            field: String::from(field),
        }
    }
}

fn is_height_valid(height: &str) -> bool {
    // number + [in|cm] => len of at least 3
    if height.len() < 3 {
        return false;
    }
    let split_index = height.len() - 2;

    let suffix = &height[split_index..];
    let number = &height[..split_index];

    match suffix {
        "cm" => is_number_in_range(number, 150, 193),
        "in" => is_number_in_range(number, 59, 76),
        _ => false
    }
}

fn is_number_in_range(number: &str, begin: u32, end: u32) -> bool {
    let number = number.parse::<u32>();
    if number.is_err() {
        return false;
    }

    let number = number.unwrap();

    number >= begin && number <= end
}

impl Valid for PassportField {
    fn is_valid(&self) -> bool {
        let field = self.field.as_str();
        match &self.field_type {
            FieldType::EyeColor => match field {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            FieldType::Height => is_height_valid(field),
            FieldType::PassportID => field.len() == 9 && field.parse::<u32>().is_ok(),
            FieldType::CountryID => true,
            FieldType::ExpirationYear => is_number_in_range(field, 2020, 2030),
            FieldType::BirthYear => is_number_in_range(field, 1920, 2002),
            FieldType::IssueYear => is_number_in_range(field, 2010, 2020),
            FieldType::HairColor => {
                let re = Regex::new("#[0-9a-f]{6}").unwrap();
                re.is_match(field)
            }
        }
    }
}

const NEEDED_FIELDS: &'static [FieldType] = &[
    FieldType::BirthYear,
    //FieldType::CountryID,
    FieldType::ExpirationYear,
    FieldType::EyeColor,
    FieldType::HairColor,
    FieldType::Height,
    FieldType::IssueYear,
    FieldType::PassportID,
];

struct Passport {
    pub fields: HashMap<FieldType, PassportField>,
}

impl Passport {
    pub fn new(input_fields: Vec<PassportField>) -> Passport {
        let mut fields: HashMap<FieldType, PassportField> =
            HashMap::with_capacity(input_fields.len());

        for f in input_fields {
            fields.insert(f.field_type, f);
        }

        Passport { fields }
    }
}

impl Valid for Passport {
    fn is_valid(&self) -> bool {
        for key in NEEDED_FIELDS {
            if !self.fields.contains_key(key) {
                return false;
            }
        }

        for field in self.fields.values() {
            if !field.is_valid() {
                return false;
            }
        }

        true
    }
}

fn parse_input(input: Vec<String>) -> Vec<Passport> {
    let mut result: Vec<Passport> = Vec::new();
    let mut curr_fields: Vec<PassportField> = Vec::new();

    for line in input {
        if line.is_empty() {
            result.push(Passport::new(curr_fields));
            curr_fields = Vec::new();

            continue;
        }

        let fields = line
            .split_whitespace()
            .map(|f| {
                let split = f.splitn(2, ':').collect::<Vec<_>>();
                (split[0], split[1])
            })
            .map(|t| PassportField::new(t.0, t.1));

        curr_fields.extend(fields);
    }

    if !curr_fields.is_empty() {
        result.push(Passport::new(curr_fields));
    }

    result
}

fn main() {
    let input = get_input::<String>("./input");
    let passports = parse_input(input);


    let result = passports.iter().filter(|p| p.is_valid()).count();

    println!("RESULT: {}", result);
}
