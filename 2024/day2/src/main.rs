type Input = Vec<Level>;
type Level = Vec<u64>;

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|chunk| chunk.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_bad_level(level: &Level) -> Option<usize> {
    let mut windows = level.windows(2).enumerate();
    let increasing = level[0] < level[1];
    while let Some((idx, [a, b])) = windows.next() {
        let diff = a.abs_diff(*b);
        if diff < 1 || diff > 3 {
            return Some(idx);
        }

        if increasing && a > b {
            return Some(idx);
        } else if !increasing && a < b {
            return Some(idx);
        }
    }
    None
}

fn part_one() -> usize {
    let input = read_input();
    input
        .iter()
        .filter(|level| find_bad_level(level).is_none())
        .count()
}

fn part_two() -> usize {
    let input = read_input();
    input
        .into_iter()
        .filter(|level| {
            let Some(idx) = find_bad_level(level) else {
                return true;
            };

            let to_remove: Vec<usize> = vec![0, idx, idx + 1];

            for idx in to_remove {
                let mut new_level = level.clone();
                new_level.remove(idx);
                if find_bad_level(&new_level).is_none() {
                    return true;
                }
            }

            false
        })
        .count()
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
