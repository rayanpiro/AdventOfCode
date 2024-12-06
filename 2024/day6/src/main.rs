use rayon::prelude::*;
use std::collections::HashSet;

type Input = Map;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    obstacles: HashSet<Position>,
    guard: Guard,
    max_y: usize,
    max_x: usize,
}

impl Map {
    fn new(guard: Guard, obstacles: HashSet<Position>, max_y: usize, max_x: usize) -> Self {
        Self {
            obstacles,
            guard,
            max_y,
            max_x,
        }
    }

    fn next(&mut self) -> Option<Guard> {
        let new_position = self.guard.direction.forward(&self.guard.position)?;
        if new_position.x > self.max_x || new_position.y > self.max_y {
            return None;
        }
        if self.obstacles.contains(&new_position) {
            self.guard.direction = self.guard.direction.turn_right();
        } else {
            self.guard.position = new_position;
        }
        Some(self.guard)
    }
}

impl Direction {
    fn forward(&self, position: &Position) -> Option<Position> {
        let mut position = *position;
        match self {
            Direction::Up => {
                if position.y <= 0 {
                    return None;
                }
                position.y -= 1;
            }
            Direction::Down => {
                position.y += 1;
            }
            Direction::Left => {
                if position.x <= 0 {
                    return None;
                }
                position.x -= 1;
            }
            Direction::Right => {
                position.x += 1;
            }
        };
        Some(position)
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }
}

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines();
    let max_y = input.lines().count() - 1;
    // All lines have the same length
    let max_x = input.lines().next().unwrap().len() - 1;
    let mut obstacles = HashSet::default();
    let mut guard_starting_position = None;

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    obstacles.insert(Position { x, y });
                }
                '^' => {
                    guard_starting_position = Some(Guard {
                        position: Position { x, y },
                        direction: Direction::Up,
                    })
                }
                '>' => {
                    guard_starting_position = Some(Guard {
                        position: Position { x, y },
                        direction: Direction::Right,
                    })
                }
                '<' => {
                    guard_starting_position = Some(Guard {
                        position: Position { x, y },
                        direction: Direction::Left,
                    })
                }
                'v' => {
                    guard_starting_position = Some(Guard {
                        position: Position { x, y },
                        direction: Direction::Down,
                    })
                }
                _ => {}
            }
        }
    }
    Map::new(guard_starting_position.unwrap(), obstacles, max_y, max_x)
}

fn part_one() -> usize {
    let mut input = read_input();
    let mut guard_walk = HashSet::from([input.guard.position]);
    while let Some(guard) = input.next() {
        guard_walk.insert(guard.position);
    }
    guard_walk.len()
}

fn part_two() -> usize {
    let input = read_input();
    let mut guard_walk = Vec::from([input.guard]);

    {
        let mut input = input.clone();
        while let Some(guard) = input.next() {
            guard_walk.push(guard);
        }
    }

    let loop_obstacles: HashSet<Position> = guard_walk
        .into_par_iter()
        .filter_map(|guard| {
            let mut input = input.clone();
            let next_position = guard.direction.forward(&guard.position)?;
            let mut visited: HashSet<Guard> = HashSet::new();

            input.obstacles.insert(next_position);
            while let Some(guard) = input.next() {
                if visited.contains(&guard) {
                    return Some(next_position);
                }
                visited.insert(guard);
            }
            None
        })
        .collect();

    loop_obstacles.iter().count()
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
