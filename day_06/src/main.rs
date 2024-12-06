use rayon::prelude::*;
use std::{collections::HashSet, fs};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let (first_result, stepped_in_coords) = solve_first(&input);
    let second_result = solve_second(&input, &stepped_in_coords);

    println!("First result: {first_result}");
    println!("Second result: {second_result}");
}

#[derive(Eq, Hash, Debug, PartialEq, Default, Clone, Copy)]
struct Coord {
    x: Num,
    y: Num,
}

impl Coord {
    fn new(x: Num, y: Num) -> Self {
        Coord { x, y }
    }

    fn sum(&self, other: &Self) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn is_in_boundaries(Coord { x, y }: &Coord, max_x: Num, max_y: Num) -> bool {
    (0..max_x).contains(x) && (0..max_y).contains(y)
}

fn is_obstacle(coord: &Coord, board: &Board) -> bool {
    let x = usize::try_from(coord.x).unwrap();
    let y = usize::try_from(coord.y).unwrap();

    board[y][x] == Tile::Obstacle
}

fn solve_first(input: &Input) -> (usize, HashSet<Coord>) {
    let mut current_position = input.position;
    let mut current_orientation = input.orientation;

    let max_y = input.board.len() as Num;
    let max_x = input.board[0].len() as Num;

    let mut visited: HashSet<Coord> = HashSet::new();

    // Store every position guard moves into
    let mut stepped_in_coords = HashSet::new();

    while is_in_boundaries(&current_position, max_x, max_y) {
        visited.insert(current_position);

        let next_move = current_orientation.get_move();
        let next_position = current_position.sum(&next_move);

        if !is_in_boundaries(&next_position, max_x, max_y) {
            current_position = next_position;
            continue;
        }

        if is_obstacle(&next_position, &input.board) {
            current_orientation.rotate();
        } else {
            stepped_in_coords.insert(next_position);
            current_position = next_position;
        }
    }

    (visited.len(), stepped_in_coords)
}

fn solve_second(input: &Input, stepped_in_coords: &HashSet<Coord>) -> usize {
    // For every position on guard steps in, create a obstacle and check for loops
    // We are not interested on creating obstacles on positions where guard won't step into
    stepped_in_coords
        .par_iter()
        .filter(|new_obstacle| check_loop(input, new_obstacle))
        .count()
}

fn check_loop(input: &Input, blocked_coord: &Coord) -> bool {
    let mut current_position = input.position;
    let mut current_orientation = input.orientation;
    let mut visited: HashSet<(Coord, Orientation)> = HashSet::new();

    let max_y = input.board.len() as Num;
    let max_x = input.board[0].len() as Num;

    while is_in_boundaries(&current_position, max_x, max_y) {
        if visited.contains(&(current_position, current_orientation)) {
            return true;
        }

        let next_move = current_orientation.get_move();
        let next_position = current_position.sum(&next_move);

        if !is_in_boundaries(&next_position, max_x, max_y) {
            return false;
        }

        if &next_position == blocked_coord || is_obstacle(&next_position, &input.board) {
            current_orientation.rotate();
        } else {
            visited.insert((current_position, current_orientation));
            current_position = next_position;
        }
    }

    false
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Orientation {
    #[default]
    Up,
    Left,
    Right,
    Down,
}

impl Orientation {
    fn get_move(&self) -> Coord {
        match self {
            Self::Up => Coord::new(0, -1),
            Self::Left => Coord::new(-1, 0),
            Self::Right => Coord::new(1, 0),
            Self::Down => Coord::new(0, 1),
        }
    }

    fn rotate(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Blank,
    Obstacle,
    Guard(Orientation),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Blank),
            '#' => Ok(Self::Obstacle),
            '^' => Ok(Self::Guard(Orientation::Up)),
            '>' => Ok(Self::Guard(Orientation::Left)),
            'v' => Ok(Self::Guard(Orientation::Down)),
            '<' => Ok(Self::Guard(Orientation::Right)),
            _ => Err(()),
        }
    }
}

fn read_input() -> Input {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let mut input = Input::default();

    for (line_num, line) in contents.lines().enumerate() {
        let board_line = line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let tile = Tile::try_from(c).unwrap();

                if let Tile::Guard(orientation) = tile {
                    input.position =
                        Coord::new(Num::try_from(i).unwrap(), Num::try_from(line_num).unwrap());
                    input.orientation = orientation;
                };

                tile
            })
            .collect();

        input.board.push(board_line);
    }

    input
}

type Board = Vec<Vec<Tile>>;

#[derive(Default, Debug)]
struct Input {
    board: Board,
    position: Coord,
    orientation: Orientation,
}

type Num = i16;
