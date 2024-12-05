use std::fs;

use regex::Regex;

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();
    let second_input = read_input_second();

    let first_result = solve_first(&input);
    let second_result = solve_second(&second_input);

    println!("First result: {first_result}");
    println!("Second result: {second_result}");
}

fn solve_first(input: &Input) -> Num {
    input.iter().map(Mul::run).sum()
}

fn solve_second(input: &Input) -> Num {
    solve_first(input)
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(&contents)
        .map(|capture| Mul {
            left: capture.get(1).unwrap().as_str().parse().unwrap(),
            right: capture.get(2).unwrap().as_str().parse().unwrap(),
        })
        .collect()
}

fn read_input_second() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let filter_dont_regex = Regex::new(r"don't.+?(?=do\(\)|$)").unwrap();

    let filtered_content = filter_dont_regex.replace_all(&contents, "");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(&filtered_content)
        .map(|capture| Mul {
            left: capture.get(1).unwrap().as_str().parse().unwrap(),
            right: capture.get(2).unwrap().as_str().parse().unwrap(),
        })
        .collect()
}

#[derive(Debug)]
struct Mul {
    left: Num,
    right: Num,
}

impl Mul {
    fn run(&self) -> Num {
        self.left * self.right
    }
}

type Input = Vec<Mul>;
type Num = usize;
