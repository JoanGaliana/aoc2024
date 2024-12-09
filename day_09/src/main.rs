use std::{collections::VecDeque, fs};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(&input);
    //let second_result = solve_second(&input);

    println!("First result:  {first_result}");
    //println!("Second result: {second_result}");
}

fn solve_first(input: &Input) -> usize {
    CompactingIterator::new(input)
        .enumerate()
        .map(|(index, n)| n * index)
        .sum()
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    contents
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Num::try_from(char::to_digit(c, 10).unwrap()).unwrap())
        .enumerate()
        .fold(Default::default(), |mut acc, (index, num)| {
            let cell = StorageCell::from_input_index(index);

            for _ in 0..num {
                acc.push_back(cell);
            }

            acc
        })
}

struct CompactingIterator {
    storage: Storage,
}

impl CompactingIterator {
    fn new(storage: &Storage) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl Iterator for CompactingIterator {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        let front = self.storage.pop_front();

        if front.is_none() {
            return None;
        }

        match front.unwrap() {
            StorageCell::Used(n) => Some(n),
            StorageCell::Free => {
                while let Some(back) = self.storage.pop_back() {
                    if let StorageCell::Used(n) = back {
                        return Some(n);
                    }
                }
                None
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum StorageCell {
    Used(Num),
    Free,
}

impl StorageCell {
    fn from_input_index(index: usize) -> Self {
        if index % 2 != 0 {
            return Self::Free;
        }

        Self::Used(index / 2)
    }
}

type Storage = VecDeque<StorageCell>;
type Input = Storage;
type Num = usize;
