use std::{collections::VecDeque, fs, str::FromStr, thread::sleep, time::Duration};

fn main() {
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let almanac = match Almanac::from_str(&contents) {
        Err(err) => panic!("{}", err),
        Ok(v) => v,
    };

    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    categories: Vec<Category>,
}

#[derive(Debug, Clone, PartialEq)]
struct Category {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, PartialEq)]
struct Mapping {
    source_start: u64,
    source_end: u64,
    destination_start: u64,
    destination_end: u64,
}

impl Mapping {
    fn new(source_start: u64, destination_start: u64, len: u64) -> Self {
        Mapping {
            source_start,
            source_end: source_start + len,
            destination_start,
            destination_end: destination_start + len
        }
    }

    fn len(&self) -> u64 {
        assert!(self.source_end - self.source_start == self.destination_end - self.destination_start);

        self.source_end - self.source_start
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn shrink_start_by(&mut self, len: u64) {
        self.source_start += len;
        self.destination_start += len;
    }

    fn shrink_end_by(&mut self, len: u64) {
        assert!(self.source_end >= len && self.destination_end >= len);

        self.source_end -= len;
        self.destination_end -= len;
    }
}

impl Almanac {
    fn find_seed_location(&self, seed: u64) -> u64 {
        self.categories
            .iter()
            .fold(seed, |r, cat| cat.find_destination(r))
    }
}

impl Category {
    fn merge(&self, source: &Self) -> Self {
        let mut sources = source.mappings.clone();
        let mut destinations = self.mappings.clone();
        let mut merged: Vec<Mapping> = Vec::new();
        sources.sort_by_key(|x| x.source_start);
        destinations.sort_by_key(|x| x.source_start);
        println!("Merging: \n\t{:?}\n\t{:?}", sources, destinations);

        // Map sources
        while ! sources.is_empty() {
            let a = &sources[0];

            if a.is_empty() {
                sources.remove(0);
                continue;
            }

            println!("Wot:\n\t{:?}", a);
            let b = match destinations
                .iter()
                .find(|b| b.source_end > a.destination_start && b.source_start < a.destination_end) {
                Some(b) => b,
                None => {
                    merged.push(sources.remove(0));
                    continue;
                }
            };
            println!("\t{:?}", b);

         
            if a.destination_start < b.source_start {
                let len = b.source_start - a.destination_start;
                println!("if {:?}", Mapping::new(a.source_start, a.destination_start, len));
                merged.push(Mapping::new(a.source_start, a.destination_start, len));
                (&mut sources[0]).shrink_start_by(len);

                continue;
            }

            if a.destination_start >= b.source_start {
                let len = (a.destination_end - a.destination_start).min(b.source_end - a.destination_start);
                let shift = a.destination_start - b.source_start;
                println!("else {:?}", Mapping::new(a.source_start, b.destination_start + shift, len));
                merged.push(Mapping::new(a.source_start, b.destination_start + shift, len));
                (&mut sources[0]).shrink_start_by(len);

                continue;
            }
        }

        merged.sort_by_key(|x| x.source_start);

        println!("\nOut mappings:");
        for mapping in merged.iter() {
            println!("{:?}", mapping)
        }
        println!("-------------------------------");

        // Fill gaps with destinations
        let mut i = 0;
        
        let mut fallthrough_gaps: Vec<Mapping> = Vec::new();

        while ! destinations.is_empty() && i < merged.len() {
            let x = &destinations[0];
            let y = &merged[i];

            if x.is_empty() {
                println!("Found empty {:?}, skipping...", x);
                destinations.remove(0);
                continue;
            }

            println!("Comparing\n\t{:?}\n\t{:?}", x, y);
            
            if x.source_start < y.source_start {
                let len = x.len().min(y.source_start - x.source_start);
                fallthrough_gaps.push(Mapping::new(x.source_start, x.destination_start, len));
                (&mut destinations[0]).shrink_start_by(len);
                println!("If");

                continue;
            }

            if x.source_start < y.source_end && x.source_end > y.source_start {
                let len = x.len().min(y.source_end - x.source_start);
                (&mut destinations[0]).shrink_start_by(len);
                println!("Else");

                continue;
            }

            i += 1;
        }

        fallthrough_gaps.extend(destinations);

        println!("Fallthrough:");
        for mapping in fallthrough_gaps.iter() {
            println!("{:?}", mapping)
        }

        merged.extend(fallthrough_gaps);
        merged.sort_by_key(|x| x.source_start);

        Category { mappings: merged }
    }

    /**
     * Find the destination of a source by using binary search
     */
    fn find_destination(&self, source: u64) -> u64 {
        match self.find_matching_mapping(source) {
            Some(category) => {
                let shift = source - category.source_start;

                return category.destination_start + shift;
            },
            None => source,
        }
    }

    fn find_matching_mapping(&self, source: u64) -> Option<&Mapping> {
        let (mut s, mut e) = (0, self.mappings.len());

        while s < e {
            let mid = (e - s) / 2 + s;
            let mid_item = &self.mappings[mid];
            
            if mid_item.source_start <= source
               && source < mid_item.source_end {

                return Some(mid_item);
            }

            if source < mid_item.source_start {
                e = mid;
                continue;
            }

            if mid_item.source_end <= source {
                s = mid + 1;
                continue;
            }

        }

        None
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds_str, rest_str) = s.trim().split_once("\n\n").ok_or("Failed to parse seeds")?;
        let seeds = seeds_str
            .strip_prefix("seeds: ")
            .and_then(|s| s
                .trim()
                .split(' ')
                .map(|s| s.parse().ok())
                .collect::<Option<Vec<u64>>>()
            ).ok_or("Failed to parse seeds")?;

        let categories = rest_str
            .split("\n\n")
            .map(|cs| cs
                .split("\n")
                .skip(1)
                .map(Mapping::from_str)
                .collect::<Result<Vec<Mapping>, String>>()
                .map_err(|x| format!("{x} | Category: {cs}"))
                .map(|mut mappings| {
                    mappings.sort_by_key(|x| x.source_start);

                    Category { mappings }
                })
            )
            .collect::<Result<Vec<Category>, String>>()?;

        Ok(
            Self {
                seeds,
                categories,
            }
        )
    }
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split_whitespace()
            .map(|s| s.parse().ok())
            .collect::<Option<Vec<u64>>>()
            .ok_or(format!("Failed to parse mapping: {}", s))?;

        if nums.len() != 3 {
            return Err(
                format!("Mapping has an incorect number of parameters {}", s)
            );
        }

        match nums[..] {
            [a, b, c] => Ok(Self::new(a, b, c)),
            _ => Err(format!("Mapping has an incorect number of parameters {}", s)),
        }
    }
}

fn part1(almanac: &Almanac) -> String {

    let lowest_location = almanac.seeds
        .iter()
        .map(|s| almanac.find_seed_location(*s))
        .min()
        .expect("Should have at least 1 location");

    lowest_location.to_string()
}

fn part2(almanac: &Almanac) -> String {
    let seed_ranges: Vec<_> = almanac.seeds
        .chunks(2)
        .map(|chunk| (chunk[0], (chunk[0] + chunk[1])))
        .collect();

    let mut lowest_location = u64::MAX;

    let merged = almanac.categories.clone().into_iter().rev().take(3).reduce(|a, b| a.merge(&b));

    println!("Final mapping: {:#?}", merged);

    "".to_owned()
}



#[cfg(test)]
mod tests {
    use crate::{Category, Mapping};

    // #[test]
    fn test1() {
        let a = Category {
            mappings: vec![
                Mapping::new(0, 69, 1),
                Mapping::new(1, 0, 69),
            ],
        };

        let b = Category {
            mappings: vec![
                Mapping::new(60, 56, 37),
                Mapping::new(56, 93, 4),
            ],
        };


        let actual = b.merge(&a);
        let expected = Category {
            mappings: vec![
                Mapping::new(0, 65, 1),
                Mapping::new(1, 0, 56),
                Mapping::new(57, 93, 4),
                Mapping::new(61, 56, 9),
                Mapping::new(70, 66, 27),
            ],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let a = Category {
            mappings: vec![
                Mapping::new(0, 65, 1),
                Mapping::new(1, 0, 56),
                Mapping::new(57, 93, 4),
                Mapping::new(61, 56, 9),
                Mapping::new(70, 66, 27),
            ],
        };

        let b = Category {
            mappings: vec![
                Mapping::new(45, 77, 23),
                Mapping::new(68, 64, 13),
                Mapping::new(81, 45, 19),
            ],
        };


        let actual = a.merge(&b);
        let expected = Category {
            mappings: vec![
                Mapping::new(0, 65, 1),
                Mapping::new(1, 0, 44),
                Mapping::new(45, 73, 20),
                Mapping::new(65, 97, 3),
                Mapping::new(68, 59, 6),
                Mapping::new(74, 66, 7),
                Mapping::new(81, 44, 12),
                Mapping::new(93, 93, 4),
                Mapping::new(97, 56, 3),
            ],
        };

        assert_eq!(expected, actual);
    }
}
