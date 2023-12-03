use std::fs;

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");
    

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> u32 {
    let parsed: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x
             .chars()
             .filter_map(|l| l.to_digit(10))
             .collect())
        .collect();

    parsed
        .iter()
        .fold(0, |a, x| {
            let first = x.first().unwrap_or(&0);
            let last = x.last().unwrap_or(&0);

            a + (first * 10 + last)
        })
}

const WORD_NUMBERS: [&'static str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn try_parse_number(input: &str) -> Option<(u32, usize)> {
    if let Some(digit) = input.chars().nth(0).unwrap().to_digit(10) {
        return Some((digit, 1));
    }

    let lower = input.to_lowercase();

    let idx_maybe = WORD_NUMBERS
        .iter()
        .position(|wn| lower.starts_with(*wn));
    

    if let Some(idx) = idx_maybe {
        let len = WORD_NUMBERS[idx].len();

        return Some((idx as u32 + 1, len));
    }

    None
}

fn parse_numbers_in_string(input: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = vec![];
    let mut i = 0;

    while i < input.len() {
        let number_maybe = try_parse_number(&input[i..]);

        if let Some((num, _len)) = number_maybe {
            nums.push(num);
            // I wish they had talked about this in the description
            // so I didn't have to spend 30 minutes trying to find
            // the bug

            // i += len; WRONG
            i += 1; // RIGHT
            continue;
        }

        i += 1;
    }

    nums
}

fn part2(input: &str) -> u32 {
    let parsed: Vec<(Vec<u32>, &str)> = input
        .lines()
        .map(|line| (parse_numbers_in_string(line), line))
        .collect();
    
    parsed
        .iter()
        .fold(0, |a, (x, _str)| {
            let first = x.first().unwrap_or(&0);
            let last = x.last().unwrap_or(&0);
            // println!("{} {}", first * 10 + last, str);

            a + (first * 10 + last)
        })
}
