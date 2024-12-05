#![feature(extract_if)]
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs,
};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(&input);
    let second_result = solve_second(&input);

    println!("First result: {first_result}");
    println!("Second result: {second_result}");
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
        .map(|rule| rule[rule.len() / 2])
        .sum()
}

fn solve_second(input: &Input) -> Num {
    input
        .rules
        .iter()
        .filter_map(|rule| {
            let mut pending = LinkedList::from_iter(rule.iter().cloned());
            let mut rule: Vec<u32> = Vec::new();
            let mut changes = 0;

            while !pending.is_empty() {
                let num = pending.front().unwrap();

                let incompatibilities = input.requirements.get(&num);

                if let Some(incompatibilities) = incompatibilities {
                    let first_incompability = incompatibilities
                        .iter()
                        .filter_map(|incompatibilty| {
                            pending.extract_if(|n| n == incompatibilty).next()
                        })
                        .next();
                    if let Some(first_incompability) = first_incompability {
                        pending.push_front(first_incompability.to_owned());
                        changes += 1;
                        continue;
                    }
                }

                rule.push(pending.pop_front().unwrap());
            }

            if changes == 0 {
                return None;
            }

            Some(rule)
        })
        .map(|rule| rule[rule.len() / 2].to_owned())
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
