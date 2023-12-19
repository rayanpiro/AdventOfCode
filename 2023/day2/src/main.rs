use regex::Regex;
use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let game_re = Regex::new(r"Game (\d+): ").unwrap();
    let sets_re = Regex::new(r" (\d+ (?:blue|red|green))[,;]?").unwrap();
    let mut values = vec![];
    'outer: for line in input.lines() {
        // First star
        let game_id: u32 = game_re
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let sets = sets_re.captures_iter(&line);
        let mut red_needed = 0;
        let mut green_needed = 0;
        let mut blue_needed = 0;
        for set in sets {
            let mut set = set.get(1).unwrap().as_str().split_whitespace();
            let num: u32 = set.next().unwrap().parse().unwrap();
            let set_color = set.next().unwrap();

            // First star
            // if set_color == "red" && num > MAX_RED
            //     || set_color == "blue" && num > MAX_BLUE
            //     || set_color == "green" && num > MAX_GREEN
            // {
            //     continue 'outer;
            // }

            // Second star
            match set_color {
                "red" => {
                    red_needed = std::cmp::max(red_needed, num);
                }
                "green" => {
                    green_needed = std::cmp::max(green_needed, num);
                }
                "blue" => {
                    blue_needed = std::cmp::max(blue_needed, num);
                }
                _ => {
                    unimplemented!("Never should get there!")
                }
            }
        }

        // First star
        // values.push(game_id);

        // Second star
        values.push(red_needed * green_needed * blue_needed);
    }
    let mut sum = 0;
    values.iter().for_each(|id| {
        sum += id;
    });
    dbg!(sum);
}
