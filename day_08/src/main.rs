use std::{
    collections::{HashMap, HashSet},
    fs,
};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let first_result = solve_first(&input);
    // let second_result = solve_second(&input);

    println!("First result:  {first_result}");
    //  println!("Second result: {second_result}");
}

fn solve_first(input: &Input) -> usize {
    let width = Num::try_from(input.width).unwrap();
    let mut antinodes: HashSet<Coord> = Default::default();
    for coords in input.nodes.values() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let antinode = coords[i].get_antinodes(&coords[j]);
                antinode
                    .into_iter()
                    .filter(|coord| coord.is_in_bounds(width))
                    .for_each(|coord| {
                        antinodes.insert(coord);
                    });
            }
        }
    }

    antinodes.len()
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let width = contents.lines().next().unwrap().chars().count();

    let mut nodes: HashMap<char, Vec<Coord>> = HashMap::new();

    contents
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .filter(|(_, c)| c != &'.')
        .for_each(|(char_num, c)| {
            let key = nodes.entry(c).or_default();
            key.push(Coord::from_num(char_num, width));
        });

    Input { width, nodes }
}

#[derive(Default, Debug)]
struct Input {
    width: usize,
    nodes: HashMap<char, Vec<Coord>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: Num,
    y: Num,
}
impl Coord {
    fn get_antinodes(&self, other: &Coord) -> [Coord; 2] {
        let x_distance = self.x - other.x;
        let y_distance = self.y - other.y;

        [
            self.add_num(x_distance, y_distance),
            other.add_num(-x_distance, -y_distance),
        ]
    }

    fn add_num(&self, x: Num, y: Num) -> Coord {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn from_num(n: usize, width: usize) -> Coord {
        Coord {
            x: (n % width).try_into().unwrap(),
            y: (n / width).try_into().unwrap(),
        }
    }

    fn new(x: Num, y: Num) -> Coord {
        Coord { x, y }
    }

    fn is_in_bounds(&self, width: Num) -> bool {
        (0..width).contains(&self.x) && (0..width).contains(&self.y)
    }
}

type Num = i32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_antinodes() {
        let expected = [Coord::new(3, 1), Coord::new(6, 7)];

        let fist = Coord::new(4, 3);
        let second = Coord::new(5, 5);

        let result = fist.get_antinodes(&second);
        assert_eq!(expected, result);

        let mut result = second.get_antinodes(&fist);
        result.reverse();

        assert_eq!(expected, result);
    }
}
