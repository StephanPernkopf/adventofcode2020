use std::collections::HashMap;

mod lib;

#[derive(PartialEq, Eq, Hash)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        return match s {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => panic!(),
        };
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let mut split = s.splitn(2, ' ');
        let op = split.next().unwrap();
        let op = Operation::from(op);

        let arg = split.next().unwrap();
        let arg = arg.parse::<i32>().unwrap();

        Instruction {
            operation: op,
            argument: arg,
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Instruction> {
    input.into_iter().map(Instruction::from).collect()
}

fn get_acc_on_infinity_loop(instructions: &Vec<Instruction>, swap_index: usize) -> (i32, bool) {
    let mut accumulator: i32 = 0;
    let mut instruction_ptr: usize = 0;
    let mut step_count = 1;

    let mut executed_instructions: HashMap<usize, &Instruction> = HashMap::new();
    let mut curr_instruction = instructions.get(instruction_ptr).unwrap();

    while !executed_instructions.contains_key(&instruction_ptr) {
        executed_instructions.insert(instruction_ptr, curr_instruction);
        let mut curr_operation = &curr_instruction.operation;
        if instruction_ptr == swap_index {
            match curr_operation {
                Operation::Nop => curr_operation = &Operation::Jmp,
                Operation::Jmp => curr_operation = &Operation::Nop,
                _ => panic!(),
            }
        }
        match curr_operation {
            Operation::Nop => instruction_ptr += 1,
            Operation::Acc => {
                accumulator += curr_instruction.argument;
                instruction_ptr += 1;
            }
            Operation::Jmp => {
                instruction_ptr = (instruction_ptr as i32 + curr_instruction.argument) as usize
            }
        }

        step_count += 1;
        let tmp = instructions.get(instruction_ptr);
        if tmp.is_none() {
            return (accumulator, true);
        } else {
            curr_instruction = tmp.unwrap()
        }
    }

    (accumulator, false)
}

fn find_next_instruction_index_to_change(
    instructions: &Vec<Instruction>,
    curr_index: usize,
) -> Option<usize> {
    for i in curr_index..instructions.len() {
        match instructions.get(i).unwrap().operation {
            Operation::Nop | Operation::Jmp => return Some(i),
            _ => continue,
        }
    }

    None
}

fn get_acc_for_correct_instruction_set(instructions: Vec<Instruction>) -> Option<i32> {
    let mut curr_index: Option<usize> = find_next_instruction_index_to_change(&instructions, 0);

    while curr_index.is_some() {
        let (acc, has_finished) = get_acc_on_infinity_loop(&instructions, curr_index.unwrap());
        if has_finished {
            return Some(acc);
        }
        curr_index = find_next_instruction_index_to_change(&instructions, curr_index.unwrap() + 1);
    }

    None
}

fn main() {
    let input: Vec<String> = lib::get_input("input.test");
    let instructions = parse_input(input);

    let result = get_acc_for_correct_instruction_set(instructions);

    println!("RESULT: {}", result.unwrap());
}
