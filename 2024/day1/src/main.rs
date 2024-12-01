type Input = (Vec<u64>, Vec<u64>);

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines();
    let mut left = Vec::default();
    let mut right = Vec::default();
    for line in lines {
        let first_space = line.find(" ").unwrap();
        let first_space_reverse = line.rfind(" ").unwrap();
        let left_element = line[0..first_space].parse().unwrap();
        let right_element = line[first_space_reverse + 1..line.len()].parse().unwrap();
        left.push(left_element);
        right.push(right_element);
    }
    (left, right)
}

fn part_one() -> u64 {
    let mut input = read_input();
    input.0.sort();
    input.1.sort();
    let mut total_distance = 0;
    for (left, right) in input.0.iter().zip(input.1.iter()) {
        let distance = left.abs_diff(*right);
        total_distance += distance;
    }
    total_distance
}

fn part_two() -> u64 {
    let input = read_input();
    let mut similarity_score = 0;
    input.0.iter().for_each(|left| {
        let left_times_in_right = input.1.iter().filter(|&right| left == right).count() as u64;
        let score = left * left_times_in_right;
        similarity_score += score;
    });
    similarity_score
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
