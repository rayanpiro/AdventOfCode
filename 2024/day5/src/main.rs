use std::collections::HashMap;

type Input = (Restrictions, Vec<Vec<usize>>);
type Restrictions = HashMap<usize, Vec<usize>>;

fn read_input() -> Input {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    parse_input(&content)
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines();
    let mut restrictions = HashMap::new();
    let mut updates = Vec::default();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut splitted = line.split('|');
        if let Some(first) = splitted.next().and_then(|x| x.parse().ok()) {
            let second = splitted.next().unwrap().parse().unwrap();
            restrictions
                .entry(first)
                .and_modify(|vec: &mut Vec<usize>| vec.push(second))
                .or_insert(vec![second]);
            continue;
        }

        let update_line: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        updates.push(update_line);
    }
    (restrictions, updates)
}

fn find_error(restrictions: &Restrictions, node: &[usize]) -> Option<(usize, usize)> {
    for (value_idx, value) in node.iter().enumerate() {
        for (child_idx, child) in node[value_idx + 1..].iter().enumerate() {
            let Some(restr) = restrictions.get(&child) else {
                continue;
            };
            if !restr.contains(value) {
                continue;
            }
            return Some((value_idx, child_idx + value_idx + 1));
        }
    }
    None
}

fn part_one() -> usize {
    let (restrictions, updates) = read_input();
    let mut count = 0usize;
    for update in updates {
        let middle = update.len() / 2;
        let middle_element = update.get(middle).unwrap().to_owned();
        if find_error(&restrictions, &update).is_some() {
            continue;
        }
        count += middle_element;
    }

    count
}

fn part_two() -> usize {
    let (restrictions, mut updates) = read_input();
    let mut count = 0usize;
    for update in updates.iter_mut() {
        let mut is_invalid = false;
        while let Some(idx_error) = find_error(&restrictions, &update) {
            update.swap(idx_error.0, idx_error.1);
            is_invalid = true;
        }
        if is_invalid {
            let middle = update.len() / 2;
            let middle_element = update.get(middle).unwrap().to_owned();
            count += middle_element;
        }
    }

    count
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
