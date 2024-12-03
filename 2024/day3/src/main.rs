use regex::Regex;

type Input = Vec<(Enabled, Mul)>;
type Enabled = bool;
type Mul = (u64, u64);

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures = re.captures_iter(input);
    let mut enabled = true;

    let mut res = Vec::default();
    for capture in captures {
        if is_mul(&capture.get(0).unwrap().as_str()) {
            let first = capture.get(1).unwrap().as_str().parse().unwrap();
            let second = capture.get(2).unwrap().as_str().parse().unwrap();
            res.push((enabled, (first, second)));
        } else {
            enabled = is_enabled(&capture.get(0).unwrap().as_str());
        }
    }

    res
}

fn is_mul(capture: &str) -> bool {
    &capture[..4] == "mul("
}

fn is_enabled(capture: &str) -> bool {
    match capture {
        "do()" => true,
        "don't()" => false,
        _ => unimplemented!(),
    }
}

fn part_one() -> u64 {
    let input = read_input();
    input.iter().fold(0, |acc, (_, (a, b))| a * b + acc)
}

fn part_two() -> u64 {
    let input = read_input();
    input.iter().fold(0, |acc, (enabled, (a, b))| {
        enabled.then_some(a * b + acc).unwrap_or(acc)
    })
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
