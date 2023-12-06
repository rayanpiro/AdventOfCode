use regex::Regex;
use std::fs;

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let mut values = vec![];
    for line in input.lines() {
        // To accept cases like oneight (18) or twone (21). Isn't explained but is required.
        let mut index = 0;
        let mut captures = vec![];
        while let Some(capture) = re.find_at(&line, index) {
            captures.push(capture.as_str().to_string());
            index = capture.start() + 1;
        }

        let first = captures.get(0);
        let last = captures.last();

        let tup = match (first, last) {
            (Some(f), Some(l)) => (f.as_str().to_string(), l.as_str().to_string()),
            (Some(f), None) => (f.as_str().to_string(), f.as_str().to_string()),
            (None, None) => continue,
            _ => unimplemented!("Impossible situation!"),
        };
        values.push(tup);
    }
    dbg!(&values);
    let mut sum = 0;
    values.iter().for_each(|(f, l)| {
        let first = NUMBERS.iter().position(|n| n == f);
        let last = NUMBERS.iter().position(|n| n == l);
        let string = match (first, last) {
            (Some(f), Some(l)) => format!("{}{}", f + 1, l + 1),
            (Some(f), None) => format!("{}{}", f + 1, l),
            (None, Some(l)) => format!("{}{}", f, l + 1),
            (None, None) => format!("{}{}", f, l),
        };
        dbg!(&string);
        sum += string.parse::<i32>().unwrap();
    });
    dbg!(sum);
}
