mod lib;

const PREAMBLE: usize = 25;


fn can_build_sum_from_pair(input: &Vec<u64>, sum: u64) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            } else if (input[i]  + input[j]) == sum {
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

fn main() {
    let input: Vec<u64> = lib::get_input("input");

    let result = get_first_non_summable(&mut input.into_iter());

    println!("RESULT: {}", result.unwrap());

}
