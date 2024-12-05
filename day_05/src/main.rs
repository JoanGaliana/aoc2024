use std::{
    collections::{HashMap, HashSet},
    fs,
};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(&input);

    println!("First result: {first_result}");
    //println!("Second result: {second_result}");
}

fn solve_first(input: &Input) -> Num {
    input
        .rules
        .iter()
        .filter(|rule| {
            let mut incompatibilities: HashSet<Num> = HashSet::new();

            for num in rule.iter() {
                if incompatibilities.contains(num) {
                    return false;
                }

                if let Some(requirements) = input.requirements.get(num) {
                    requirements.iter().for_each(|requirement| {
                        incompatibilities.insert(requirement.to_owned());
                    });
                }
            }

            true
        })
        .map(|rule| {
            rule.get(rule.len() / 2)
                .map(u32::to_owned)
                .unwrap_or_default()
        })
        .sum()
}

fn solve_first(input: &Input) -> Num {
    input
        .rules
        .iter()
        .filter(|rule| {
            let mut incompatibilities: HashSet<Num> = HashSet::new();

            for num in rule.iter() {
                if incompatibilities.contains(num) {
                    return false;
                }

                if let Some(requirements) = input.requirements.get(num) {
                    requirements.iter().for_each(|requirement| {
                        incompatibilities.insert(requirement.to_owned());
                    });
                }
            }

            true
        })
        .map(|rule| {
            rule.get(rule.len() / 2)
                .map(u32::to_owned)
                .unwrap_or_default()
        })
        .sum()
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let mut input = Input::default();

    let mut first = true;
    for line in contents.lines() {
        if line.is_empty() {
            first = false;
        } else if first {
            let mut split = line.split('|');
            let required = split.next().unwrap().parse::<Num>().unwrap();
            let num = split.next().unwrap().parse::<Num>().unwrap();

            input
                .requirements
                .entry(num)
                .or_insert(Default::default())
                .push(required);
        } else {
            let rule = line
                .split(',')
                .map(|item| item.parse::<Num>().unwrap())
                .collect::<Vec<_>>();

            input.rules.push(rule);
        }
    }

    input
}

#[derive(Default, Debug)]
struct Input {
    requirements: HashMap<Num, Vec<Num>>,
    rules: Vec<Vec<Num>>,
}

type Num = u32;
