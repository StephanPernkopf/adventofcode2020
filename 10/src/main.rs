use std::{cmp::min, collections::HashMap};

mod lib;

fn get_differences(mut numbers: Vec<u64>) -> HashMap<u64, u64> {
    numbers.sort();
    let mut differences: HashMap<u64, u64> = HashMap::new();

    // from outlet to first adapter
    numbers.insert(0, 0);

    let last_number = numbers[numbers.len() - 1];
    // from last adapter to device
    numbers.push(last_number + 3);

    println!("{:?}", numbers);
    for i in 0..numbers.len() - 1 {
        *differences.entry(numbers[i + 1] - numbers[i]).or_insert(0) += 1;
    }

    differences
}

fn get_total_arrangements(numbers: &[u64], cache: &mut HashMap<u64, u64>) -> u64 {
    if numbers.len() == 1 {
        return 1;
    }

    let mut count: u64 = 0;
    let curr_number = numbers[0];
    for i in 1..min(4, numbers.len()) {
        if (numbers[i] - curr_number) > 3 {
            break
        } else {
            if cache.contains_key(&numbers[i]) {
                count += cache.get(&numbers[i]).unwrap();
            } else {
                let x = get_total_arrangements(&numbers[i..], cache);
                count += x;
                cache.insert(numbers[i], x);
            }
            //count += 1;
        }
    }

    count// + get_total_arrangements(&numbers[1..])
}

fn main() {
    let file = "input";
    let input: Vec<u64> = lib::get_input(file);
    let result = get_differences(input);

    println!("{:?}", result);
    println!(
        "RESULT: {}",
        result.get(&1).unwrap() * result.get(&3).unwrap()
    );

    let mut input: Vec<u64> = lib::get_input(file);
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort();
    println!("{:?}", input);
    let mut cache = HashMap::new();
    let result = get_total_arrangements(&input, &mut cache);

    println!("RESULT: {}", result);
}
