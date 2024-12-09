use std::fs;

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let second_result = solve_second(&input);

    println!("Second result: {second_result}");
}

fn solve_second(input: &Input) -> usize {
    let index = 0;
    let sum = 0;

    for cell in CompactingIterator::new(input) {
        dbg!(cell);
    }

    sum
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    contents
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Num::try_from(char::to_digit(c, 10).unwrap()).unwrap())
        .enumerate()
        .fold(Default::default(), |mut acc, (index, size)| {
            acc.push(StorageCell::from_input_index(index, size));

            acc
        })
}

struct CompactingIterator {
    storage: Storage,
    index: usize,
}

impl CompactingIterator {
    fn new(storage: &Storage) -> Self {
        Self {
            storage: storage.clone(),
            index: 0,
        }
    }
}

impl Iterator for CompactingIterator {
    type Item = StorageCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.storage.len() {
            return None;
        }

        let mut back_index = self.storage.len() - 1;
        let front = self.storage[self.index];
        match front {
            StorageCell::Used { .. } => {
                self.index += 1;
                Some(front)
            }
            StorageCell::Free { size: free_size } => {
                while back_index > self.index {
                    let back = self.storage[back_index];

                    if let StorageCell::Used { size, .. } = back {
                        if size == free_size {
                            self.index += 1;

                            return Some(self.storage.remove(back_index));
                        }

                        if free_size < size {
                            if let StorageCell::Free { size: free_size } =
                                self.storage.get_mut(self.index).unwrap()
                            {
                                *free_size = free_size.abs_diff(size);
                            }

                            return Some(self.storage.remove(back_index));
                        }
                    }

                    back_index -= 1;
                }
                None
            }
        }
    }
}
// 0x3 -x4 4x4 5x4

#[derive(Clone, Copy, Eq, PartialEq)]
enum StorageCell {
    Used { num: Num, size: usize },
    Free { size: usize },
}

impl StorageCell {
    fn from_input_index(index: usize, size: usize) -> Self {
        if index % 2 != 0 {
            return Self::Free { size };
        }

        Self::Used {
            num: index / 2,
            size,
        }
    }
}

type Storage = Vec<StorageCell>;
type Input = Storage;
type Num = usize;
