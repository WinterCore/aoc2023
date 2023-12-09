use std::{fs, str::FromStr, collections::{HashSet, HashMap}};

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");


    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[derive(Debug)]
struct Card {
    winning_nums: Vec<u64>,
    my_nums: Vec<u64>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        
        let (_id_str, rest) = trimmed
            .strip_prefix("Card")
            .and_then(|x| x.trim().split_once(':'))
            .ok_or("Failed to parse")?;

        let (winning_nums_str, my_nums_str) = rest
            .split_once('|')
            .ok_or("Failed to parse")?;


        let winning_nums = winning_nums_str
            .trim()
            .split_whitespace()
            .map(|x| x.parse().ok())
            .collect::<Option<Vec<u64>>>()
            .ok_or("Failed to parse")?;

        let my_nums = my_nums_str
            .trim()
            .split_whitespace()
            .map(|x| x.parse().ok())
            .collect::<Option<Vec<u64>>>()
            .ok_or("Failed to parse")?;

        Ok(
            Self {
                winning_nums,
                my_nums,
            }
        )
    }
}

fn part1(input: &str) -> String {
    let cards = input
        .trim()
        .lines()
        .map(|x| Card::from_str(x))
        .collect::<Result<Vec<Card>, <Card as FromStr>::Err>>()
        .expect("Failed to parse");

    let ans = cards
        .iter()
        .fold(0u64, |a, c| {
            let set: HashSet<u64> = HashSet::from_iter(c.winning_nums.iter().cloned());
            let winning_nums_count = c.my_nums
                .iter()
                .filter(|num| set.contains(num))
                .collect::<Vec<&u64>>()
                .len();
            
            let points = {
                if winning_nums_count == 0 {
                    0
                } else {
                    2u64.pow(winning_nums_count as u32 - 1)
                }
            };

            a + points
        });
    

    ans.to_string()
}

fn part2(input: &str) -> String {
    let cards = input
        .trim()
        .lines()
        .map(|x| Card::from_str(x))
        .collect::<Result<Vec<Card>, <Card as FromStr>::Err>>()
        .expect("Failed to parse");

    type Copies = u64;

    let mut map: HashMap<usize, Copies> = HashMap::new();

    let mut total_copies: u64 = 0;

    for i in 0..cards.len() {
        let card = &cards[i];

        let winning_nums_count = card.my_nums
            .iter()
            .filter(|num| card.winning_nums.contains(num))
            .collect::<Vec<&u64>>()
            .len();

        let copies = *map.get(&i).unwrap_or(&1);

        total_copies += copies;

        for j in (i + 1)..((i + 1 + winning_nums_count).min(cards.len())) {
            let child_copies = map.entry(j).or_insert(1);

            *child_copies += copies;
        }
    }
    

    total_copies.to_string()
}
