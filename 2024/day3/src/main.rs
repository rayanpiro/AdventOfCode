use regex::Regex;

type Input = Vec<(Enabled, Mul)>;
type Enabled = bool;
type Mul = (u64, u64);

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    let mut slices = Vec::default();

    let enable_re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut enable_captures = enable_re.captures_iter(input);
    let mut last = enable_captures.next().unwrap();
    slices.push((true, &input[0..last.get(0).unwrap().start()]));

    while let Some(current) = enable_captures.next() {
        slices.push((
            is_enabled(last.get(0).unwrap().as_str()),
            &input[last.get(0).unwrap().end()..current.get(0).unwrap().start()],
        ));
        last = current;
    }
    slices.push((
        is_enabled(last.get(0).unwrap().as_str()),
        &input[last.get(0).unwrap().end()..],
    ));

    let mut res = Vec::default();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (enabled, slice) in slices {
        let mul_captures = mul_re.captures_iter(slice);
        for mul_capture in mul_captures {
            let first = mul_capture.get(1).unwrap().as_str().parse().unwrap();
            let second = mul_capture.get(2).unwrap().as_str().parse().unwrap();
            res.push((enabled, (first, second)));
        }
    }

    res
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
