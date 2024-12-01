use std::{collections::HashMap, fs};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(input.clone());
    let second_result = solve_second(input.clone());

    println!("First result: {first_result}");
    println!("Second result: {second_result}");
}

fn solve_first(input: Input) -> u32 {
    let Input {
        mut left,
        mut right,
    } = input;

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn solve_second(input: Input) -> u32 {
    let Input { left, right } = input;

    let mut frequencies: HashMap<u32, u32> = HashMap::new();

    for num in right {
        let frequency = frequencies.entry(num).or_default();
        *frequency += 1;
    }

    left.iter()
        .map(|num| {
            let frequency = frequencies.get(num).unwrap_or(&0);
            num * frequency
        })
        .sum()
}

fn read_input() -> Input {
    // --snip--
    println!("In file {FILE_PATH}");

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut input = Input::new();

    for line in contents.split("\n") {
        let mut split = line.split("   ");

        let left_str = split.next().expect("str missing");
        let right_str = split.next().expect("str missing");

        let left_num = left_str.parse::<u32>().expect("Could not parse number");
        let right_num = right_str.parse::<u32>().expect("Could not parse number");

        input.left.push(left_num);
        input.right.push(right_num);
    }

    input
}

#[derive(Clone)]
struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}
impl Input {
    fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
        }
    }
}
