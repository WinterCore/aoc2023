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

#[derive(Debug, Clone)]
struct Category {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Clone)]
struct Mapping {
    source_start: u64,
    source_end: u64,
    destination_start: u64,
    destination_end: u64,
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
        let mut _sources = source.mappings.clone();
        _sources.sort_by_key(|x| x.destination_start);
        let mut sources = VecDeque::from(_sources);
        let mut destinations = VecDeque::from(self.mappings.clone());
        let mut merged: Vec<Mapping> = Vec::new();

        while ! sources.is_empty() && ! destinations.is_empty() {
            let s = &mut sources[0];
            let d = &mut destinations[0];

            // Copy from destination
            if d.source_start < s.destination_start {
                let len = (s.destination_start - d.source_start).min(d.source_start - d.source_end);

                let mapping = Mapping {
                    source_start: d.source_start,
                    source_end: d.source_start + len,
                    destination_start: d.destination_start,
                    destination_end: d.destination_start + len,
                };

                merged.push(mapping);
                d.source_start += len;
                d.destination_start += len;

                if d.source_end - d.source_start == 0 {
                    destinations.pop_front();
                }

                continue;
            }


            // Copy from source


        }

        
        unimplemented!()
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
            [a, b, c] => 
                Ok(
                    Self {
                        destination_start: a,
                        destination_end: a + c,
                        source_start: b,
                        source_end: b + c,
                    }
                ),
            _ =>
                Err(
                    format!("Mapping has an incorect number of parameters {}", s)
                ),
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


    let categories = almanac.categories.clone().into_iter().rev().take(2).collect::<Vec<Category>>();

    let merged = categories.into_iter().reduce(|a, b| a.merge(&b));

    println!("Final mapping: {:?}", merged);

    "".to_owned()
}



/*

#[cfg(test)]
mod tests {
    use crate::{Category, Mapping};

    #[test]
    fn test1() {
        let a = Category {
            mappings: vec![
                Mapping { source_start: 8, range_length: 10, destination_start: 5 },
            ],
        };

        let b = Category {
            mappings: vec![
                Mapping { source_start: 2, range_length: 10, destination_start: 3 },
            ],
        };

        // 2-8   -> 3-9
        // 8-12  -> 6-10
        // 12-18 -> 9-15

        println!("\n\nstart-----------------------\n");

        let result = Category::merge(&vec![a, b]);

        println!("{:?}\n", result);
        println!("done-----------------------\n\n");
    }
}
*/
