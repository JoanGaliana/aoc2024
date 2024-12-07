use std::fs;

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(&input);
    let second_result = solve_second(&input);

    println!("First result:  {first_result}");
    println!("Second result: {second_result}");
}

fn solve_first(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|line| check_valid(0, 0, line))
        .map(|line| line.test_value)
        .sum()
}
fn check_valid(current_value: Num, pos: usize, input: &Input) -> bool {
    if current_value == input.test_value && pos == input.operands.len() {
        return true;
    }
    if pos >= input.operands.len() || current_value > input.test_value {
        return false;
    }

    let current_mul_value = match pos {
        0 => 1,
        _ => current_value,
    };
    let mul_result = current_mul_value * input.operands[pos];
    if check_valid(mul_result, pos + 1, input) {
        return true;
    }

    let sum_result = current_value + input.operands[pos];
    check_valid(sum_result, pos + 1, input)
}

fn solve_second(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|line| check_valid_second(0, 0, line))
        .map(|line| line.test_value)
        .sum()
}

fn check_valid_second(current_value: Num, pos: usize, input: &Input) -> bool {
    if current_value == input.test_value && pos == input.operands.len() {
        return true;
    }
    if pos >= input.operands.len() || current_value > input.test_value {
        return false;
    }

    let current_operand = input.operands[pos];

    let current_mul_value = match pos {
        0 => 1,
        _ => current_value,
    };
    let mul_result = current_mul_value * current_operand;
    if check_valid_second(mul_result, pos + 1, input) {
        return true;
    }

    let sum_result = current_value + current_operand;
    if check_valid_second(sum_result, pos + 1, input) {
        return true;
    }

    let digits = current_operand.checked_ilog10().unwrap_or_default() + 1;
    let concatenate_value = current_value * 10_usize.pow(digits);
    let concatenate_result = concatenate_value + current_operand;
    check_valid_second(concatenate_result, pos + 1, input)
}

fn read_input() -> Vec<Input> {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    contents
        .lines()
        .map(|line| {
            let mut split = line.split(':');
            let test_value = split.next().unwrap().parse().unwrap();
            let operands = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();

            Input {
                test_value,
                operands,
            }
        })
        .collect()
}

#[derive(Default, Debug)]
struct Input {
    test_value: Num,
    operands: Vec<Num>,
}

type Num = usize;
