use std::fs;

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(&input);
    let second_result = solve_second(input.clone());

    println!("First result: {first_result}");
    println!("Second result: {second_result}");
}

#[derive(PartialEq)]
enum SortDir {
    Asc,
    Desc,
}

impl SortDir {
    fn get(x: Num, y: Num) -> Self {
        if x < y {
            return Self::Asc;
        }

        Self::Desc
    }
}

fn check_items(x: Num, y: Num, direction: &SortDir) -> bool {
    let current_direction = SortDir::get(x, y);
    let distance = x.abs_diff(y);

    if current_direction != *direction || !(1..=3).contains(&distance) {
        return false;
    }

    true
}

fn check_line(line: &[Num]) -> bool {
    if line.len() < 2 {
        return true;
    }

    let dir = SortDir::get(line[0], line[1]);

    for i in 1..line.len() {
        if !check_items(line[i - 1], line[i], &dir) {
            return false;
        }
    }

    true
}

struct PossibleLinesIter {
    line: Vec<Num>,
    first: bool,
    current: usize,
}

impl Iterator for PossibleLinesIter {
    type Item = Vec<Num>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.line.len() {
            return None;
        }

        let mut line = self.line.clone();

        if self.first {
            self.first = false;
            return Some(line);
        }

        line.remove(self.current);
        self.current += 1;

        Some(line)
    }
}

impl PossibleLinesIter {
    fn new(line: &[Num]) -> Self {
        Self {
            current: 0,
            first: true,
            line: line.to_vec(),
        }
    }
}

fn check_line_with_tolerance(line: &[Num]) -> bool {
    PossibleLinesIter::new(line).any(|line| check_line(&line))
}

fn solve_first(input: &Input) -> usize {
    input.iter().filter(|line| check_line(line)).count()
}

fn solve_second(input: Input) -> usize {
    input
        .iter()
        .filter(|line| check_line_with_tolerance(line))
        .count()
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut input: Input = Vec::new();

    for line in contents.split("\n") {
        let mut line_nums = Vec::new();

        for item in line.split(" ") {
            let item = item.parse::<Num>().expect("Could not parse number");
            line_nums.push(item);
        }

        input.push(line_nums);
    }

    input
}

type Input = Vec<Vec<Num>>;
type Num = u32;
