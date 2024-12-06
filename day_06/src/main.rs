use std::{collections::HashSet, fs};

//const FILE_PATH: &str = "./example_input.txt";
const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();

    let (first_result, possible_new_obstacles) = solve_first(&input);
    let second_result = solve_second(&input, &possible_new_obstacles);

    println!("First result: {first_result}");
    println!("Second result: {second_result}");
}

#[derive(Eq, Hash, Debug, PartialEq, Default, Clone)]
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
    let mut current_position = input.position.clone();
    let mut current_orientation = input.orientation;

    let max_y = input.board.len() as Num;
    let max_x = input.board[0].len() as Num;

    let mut visited: HashSet<Coord> = HashSet::new();

    // Store every position guard moves into
    let mut possible_new_obstacles = HashSet::new();

    while is_in_boundaries(&current_position, max_x, max_y) {
        visited.insert(current_position.clone());

        let next_move = current_orientation.get_move();
        let next_position = current_position.sum(&next_move);

        if !is_in_boundaries(&next_position, max_x, max_y) {
            current_position = next_position;
            continue;
        }

        if is_obstacle(&next_position, &input.board) {
            current_orientation.rotate();
        } else {
            possible_new_obstacles.insert(next_position.clone());
            current_position = next_position;
        }
    }

    (visited.len(), possible_new_obstacles)
}

fn solve_second(input: &Input, possible_new_obstacles: &HashSet<Coord>) -> usize {
    // For every position on guard's path, create a obstacle and check for loops
    // We are not interested on creating obstacles on positions where guard won't step into
    possible_new_obstacles
        .iter()
        .filter(|new_obstacle| check_loop(input, new_obstacle))
        .count()
}

const MAX_TURNS: usize = 100000;
fn check_loop(input: &Input, blocked_coord: &Coord) -> bool {
    let mut turns = 0;
    let mut current_position = input.position.clone();
    let mut current_orientation = input.orientation;

    let max_y = input.board.len() as Num;
    let max_x = input.board[0].len() as Num;

    while is_in_boundaries(&current_position, max_x, max_y) {
        if turns >= MAX_TURNS {
            return true;
        }

        let next_move = current_orientation.get_move();
        let next_position = current_position.sum(&next_move);

        if !is_in_boundaries(&next_position, max_x, max_y) {
            return false;
        }

        if &next_position == blocked_coord || is_obstacle(&next_position, &input.board) {
            current_orientation.rotate();
            turns += 1;
        } else {
            current_position = next_position;
        }
    }

    false
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
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
