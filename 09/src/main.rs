mod lib;
use itertools::Itertools;


const PREAMBLE: usize = 25;

fn can_build_sum_from_pair(input: &Vec<u64>, sum: u64) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            } else if (input[i] + input[j]) == sum {
                return true;
            }
        }
    }

    false
}

fn get_first_non_summable<I>(iter: &mut I) -> Option<u64>
where
    I: Iterator<Item = u64>,
{
    let mut last_numbers: Vec<u64> = Vec::with_capacity(PREAMBLE);
    for _ in 0..PREAMBLE {
        last_numbers.push(iter.next().unwrap());
    }

    while let Some(sum) = iter.next() {
        if !can_build_sum_from_pair(&last_numbers, sum) {
            return Some(sum);
        }
        // TODO: shift
        last_numbers.remove(0);
        last_numbers.push(sum);
    }

    None
}

fn find_contiguous_sum(numbers: Vec<u64>, sum: u64) -> Vec<u64> {
    for i in 0..numbers.len() {
        let curr_combination = numbers.iter().combinations(i);
    }
    todo!()
}

fn find_encryption_weakness<I>(iter: &mut I, first_non_summable: u64) -> Option<u64>
where
    I: Iterator<Item = u64>,
{
    let mut numbers = Vec::new();
    while let Some(num) = iter.next() {
        if num == first_non_summable {
            break;
        }

        numbers.push(num);
    }

    let contiguous_sum_numbers = find_contiguous_sum(numbers, first_non_summable);
    let min = contiguous_sum_numbers.iter().min().unwrap();
    let max = contiguous_sum_numbers.iter().min().unwrap();

    Some(min + max)
}

fn main() {
    let input: Vec<u64> = lib::get_input("input");

    let first_non_summable = get_first_non_summable(&mut input.into_iter());

    // sorry too lazy
    let input: Vec<u64> = lib::get_input("input");

    let result = find_encryption_weakness(&mut input.into_iter(), first_non_summable.unwrap());

    println!("RESULT: {}", result.unwrap());
}
