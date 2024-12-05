use std::collections::HashMap;

type Input = (Restrictions, Vec<Vec<usize>>);
type Restrictions = HashMap<usize, Vec<usize>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    value: usize,
    childs: Vec<Node>,
}

impl Node {
    fn new(updates: &Vec<usize>) -> Self {
        Self::build_tree(&mut updates.clone())
    }
    fn build_tree(updates: &mut Vec<usize>) -> Self {
        let mut found_nodes = vec![];
        while updates.len() > 0 {
            let value = updates.pop().unwrap();
            let childs = std::mem::take(&mut found_nodes);
            let node = Self { value, childs };
            found_nodes.push(node);
        }
        found_nodes.pop().unwrap()
    }
}

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

fn find_error(restrictions: &Restrictions, node: &Node) -> Option<(usize, usize)> {
    let mut idx_ins = 0;
    let mut to_inspect = vec![node.to_owned()];
    while idx_ins < to_inspect.len() {
        let value = to_inspect[idx_ins].value;
        for idx_child in 0..to_inspect[idx_ins].childs.len() {
            let child = &to_inspect[idx_ins].childs[idx_child];
            if let Some(restr) = restrictions.get(&child.value) {
                if restr.contains(&value) {
                    return Some((value, child.value));
                }
            }
            to_inspect.push(child.clone());
        }
        idx_ins += 1;
    }

    None
}

fn part_one() -> usize {
    let (restrictions, updates) = read_input();
    let mut count = 0usize;
    for update in updates {
        let middle = update.len() / 2;
        let middle_element = update.get(middle).unwrap().to_owned();
        let node = Node::new(&update);
        if find_error(&restrictions, &node).is_some() {
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
        let mut node = Node::new(update);
        while let Some(error_value) = find_error(&restrictions, &node) {
            let idx_error = (
                update.iter().position(|n| n == &error_value.0).unwrap(),
                update.iter().position(|n| n == &error_value.1).unwrap(),
            );
            update.swap(idx_error.0, idx_error.1);
            node = Node::new(update);
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
