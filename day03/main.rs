use std::{fs, str::FromStr, collections::HashSet};

#[derive(Debug, PartialEq)]
enum SchematicElement {
    Part((usize, u64)),
    Symbol(char),
    Empty,
}

#[derive(Debug)]
struct Schematic {
    width: usize,
    height: usize,
    elements: Vec<SchematicElement>,
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();
        let height = lines.len();
        let width = lines
            .iter()
            .nth(0)
            .ok_or("Should have at least one element")?
            .len();

        let elements = lines
            .iter()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                let chars: Vec<_> = line.chars().collect();
                let mut i = 0;
                let mut elements: Vec<SchematicElement> = vec![];

                while i < chars.len() {
                    if chars[i] == '.' {
                        i += 1;
                        elements.push(SchematicElement::Empty);
                        continue;
                    }

                    if chars[i].is_digit(10) {
                        let mut j = i + 1;
                        let mut num_chars = vec![chars[i]];

                        while j < width && chars[j].is_digit(10) {
                            num_chars.push(chars[j]);
                            j += 1;
                        }

                        let part_number = num_chars
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap();

                        for _ in 0..num_chars.len() {
                            elements.push(SchematicElement::Part((line_idx * width + i, part_number)));
                        }

                        i += num_chars.len();
                        continue;
                    }
                    
                    elements.push(SchematicElement::Symbol(chars[i]));
                    i += 1;
                }

                elements
            })
            .collect();

        Ok(
            Self {
                width,
                height,
                elements,
            }
        )
    }
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");


    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> String {
    let Schematic { width, height, elements } = Schematic::from_str(input)
        .expect("Failed to parse input");

    let mut sum = 0;
    let mut set = HashSet::<(usize, u64)>::new();

    let get_part_number_at = |set: &mut HashSet<(usize, u64)>, x: i64, y: i64| {
        if x < 0 || x >= width as i64 || y < 0 || y >= height as i64 {
            return None;
        }
        
        let i = y as usize * width + x as usize;

        if let SchematicElement::Part((s, num)) = elements[i] {
            let key = (s, num);

            if set.contains(&key) {
                return None
            }

            set.insert(key);
            return Some(num);
        }

        None
    };

    for i in 0..elements.len() {
        let x = (i % width) as i64;
        let y = (i / width) as i64;

        if ! matches!(elements[i], SchematicElement::Symbol(_)) {
            continue;
        }

        sum += get_part_number_at(&mut set, x - 1, y).unwrap_or(0);
        sum += get_part_number_at(&mut set, x - 1, y - 1).unwrap_or(0);
        sum += get_part_number_at(&mut set, x, y - 1).unwrap_or(0);
        sum += get_part_number_at(&mut set, x + 1, y - 1).unwrap_or(0);
        sum += get_part_number_at(&mut set, x + 1, y).unwrap_or(0);
        sum += get_part_number_at(&mut set, x + 1, y + 1).unwrap_or(0);
        sum += get_part_number_at(&mut set, x, y + 1).unwrap_or(0);
        sum += get_part_number_at(&mut set, x - 1, y + 1).unwrap_or(0);
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let Schematic { width, height, elements } = Schematic::from_str(input)
        .expect("Failed to parse input");



    "".to_owned()
}

