use std::collections::HashSet;

type Input = (Sheet, Vec<LetterX>, Vec<LetterA>);
type LetterX = Position;
type LetterA = Position;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sheet {
    chars: Vec<Vec<char>>,
    max_y: usize,
    max_x: usize,
}

impl Sheet {
    fn new(sheet: Vec<Vec<char>>) -> Self {
        let max_y = sheet.len() - 1;

        // All lines have the same length
        let max_x = sheet.get(0).unwrap().len() - 1;

        Self {
            chars: sheet,
            max_x,
            max_y,
        }
    }

    fn max_x(&self) -> usize {
        self.max_x
    }
    fn min_x(&self) -> usize {
        0
    }
    fn max_y(&self) -> usize {
        self.max_y
    }
    fn min_y(&self) -> usize {
        0
    }

    fn get_position(&self, position: &Position) -> char {
        let Position { x, y } = *position;
        self.chars[y][x]
    }

    fn new_position(&self, moves: &Vec<Direction>, position: &Position) -> Option<Position> {
        let mut new_position = position.clone();
        for direction in moves.iter() {
            match direction {
                Direction::Up => {
                    let Position { x, y } = new_position;
                    if y <= self.min_y() {
                        return None;
                    }
                    new_position = Position { y: y - 1, x };
                }
                Direction::Down => {
                    let Position { x, y } = new_position;
                    if y >= self.max_y() {
                        return None;
                    }
                    new_position = Position { y: y + 1, x };
                }
                Direction::Left => {
                    let Position { x, y } = new_position;
                    if x <= self.min_x() {
                        return None;
                    }
                    new_position = Position { y, x: x - 1 };
                }
                Direction::Right => {
                    let Position { x, y } = new_position;
                    if x >= self.max_x() {
                        return None;
                    }
                    new_position = Position { y, x: x + 1 };
                }
            }
        }
        Some(new_position)
    }
}

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines();
    let mut input = Vec::default();
    let mut letter_x = Vec::default();
    let mut letter_a = Vec::default();

    for (y, line) in lines.enumerate() {
        let mut row = Vec::default();
        for (x, char) in line.chars().enumerate() {
            if char == 'X' {
                letter_x.push(Position { x, y });
            }
            if char == 'A' {
                letter_a.push(Position { x, y });
            }
            row.push(char);
        }
        input.push(row);
    }
    (Sheet::new(input), letter_x, letter_a)
}

fn part_one() -> u64 {
    let (sheet, letters_x, _) = read_input();

    let word = ['X', 'M', 'A', 'S'];
    let directions = [
        vec![Direction::Up],
        vec![Direction::Down],
        vec![Direction::Left],
        vec![Direction::Right],
        vec![Direction::Up, Direction::Left],
        vec![Direction::Up, Direction::Right],
        vec![Direction::Down, Direction::Left],
        vec![Direction::Down, Direction::Right],
    ];

    let mut count = 0;

    for letter_x in letters_x {
        for direction in &directions {
            let mut position = letter_x.clone();
            let mut found = true;
            for chr in word {
                if sheet.get_position(&position) != chr {
                    found = false;
                    break;
                }
                if let Some(new_position) = sheet.new_position(direction, &position) {
                    position = new_position;
                }
            }
            if found {
                count += 1;
            }
        }
    }
    count
}

fn part_two() -> u64 {
    let (sheet, _, letters_a) = read_input();

    let word = ['M', 'A', 'S'];
    let diagonal1 = [
        vec![],
        vec![Direction::Up, Direction::Left],
        vec![Direction::Down, Direction::Right],
    ];

    let diagonal2 = [
        vec![],
        vec![Direction::Up, Direction::Right],
        vec![Direction::Down, Direction::Left],
    ];

    let mut count = 0;

    for letter_a in letters_a {
        let mut found = true;

        let mut set = HashSet::from(word);
        for direction in &diagonal1 {
            if let Some(position) = sheet.new_position(&direction, &letter_a) {
                if !set.remove(&sheet.get_position(&position)) {
                    found = false;
                    break;
                }
            } else {
                found = false;
                break;
            }
        }

        if !found {
            continue;
        }

        let mut set = HashSet::from(word);
        for direction in &diagonal2 {
            if let Some(position) = sheet.new_position(&direction, &letter_a) {
                if !set.remove(&sheet.get_position(&position)) {
                    found = false;
                    break;
                }
            } else {
                found = false;
                break;
            }
        }

        if found {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
