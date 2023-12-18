use std::{fs, str::FromStr, thread::sleep, time::Duration};

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
    range_length: u64,
    destination_start: u64,
}

impl Almanac {
    fn find_seed_location(&self, seed: u64) -> u64 {
        self.categories
            .iter()
            .fold(seed, |r, cat| cat.find_destination(r))
    }
}

impl Category {
    fn merge(categories: &[Self]) -> Self {
        // TODO: Check for len < 2
        let mut merged_category: Category = categories.last().cloned().unwrap();

        // Collapse the ranges bottom up

        for i in (categories.len() - 2)..=0 {

            let Category { mappings } = &categories[i];
            let mut merged: Vec<Mapping> = vec![];

            for mapping in mappings {
                let filler_mappings: Vec<&Mapping> = merged_category.mappings
                    .iter()
                    .take_while(|x| x.source_start < mapping.source_start)
                    .collect();

                println!("Filler mappings {:?}\n", mapping);
                for item in filler_mappings {
                    merged.push(Mapping {
                        source_start: item.source_start,
                        range_length: item.range_length
                            .min(mapping.source_start - item.source_start),
                        destination_start: item.destination_start,
                    });
                }
                
                let overlapping_mappings: Vec<&Mapping> = merged_category.mappings
                    .iter()
                    .take_while(|x| {
                        // println!("\t{}.max({}) <= {}.min({}) = {} <= {}\n", x.source_start, mapping.destination_start, x.source_end(), mapping.destination_end(), x.source_start.max(mapping.destination_start), x.source_end().min(mapping.destination_end()));

                        x.source_start.max(mapping.destination_start)
                        <= x.source_end().min(mapping.destination_end())
                    })
                    .collect();
                println!("Overlapping mappings {:?}\n", overlapping_mappings);

                /*
                * 2.max(50) <= 12.min(60)
                * 50 <= 12
                * false
                */

                for item in overlapping_mappings {
                    if mapping.source_start < item.source_start {
                        println!("\tFirst if---------");
                        let range_length = item.source_start - mapping.source_start;

                        merged.push(Mapping {
                            source_start: mapping.source_start,
                            range_length,
                            destination_start: mapping.destination_start,
                        });

                        continue;
                    }


                    let start = merged.last().map_or(mapping.source_start, |x| x.source_end());
                    let end = mapping.destination_end().min(item.source_end());
                    
                    println!("start = {}, end = {}, dest_start = {}", start, end, item.destination_start);

                    let range_length = end - start;

                    merged.push(Mapping {
                        source_start: start,
                        range_length,
                        destination_start: item.destination_start + (mapping.destination_start - item.source_start),
                    });

                }
                

                let start = merged.last().map_or(mapping.source_start, |x| x.source_end());

                if start < mapping.source_end() {
                    let range_length = mapping.source_end() - start;

                    merged.push(Mapping {
                        source_start: start,
                        range_length,
                        destination_start: mapping.destination_end() - range_length,
                    });
                }
            }

            merged_category = Category { mappings: merged };
        }

        merged_category
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
               && source < mid_item.source_start + mid_item.range_length {

                return Some(mid_item);
            }

            if source < mid_item.source_start {
                e = mid;
                continue;
            }

            if mid_item.source_start + mid_item.range_length <= source {
                s = mid + 1;
                continue;
            }

        }

        None
    }
}

impl Mapping {
    fn source_end(&self) -> u64 {
        self.source_start + self.range_length
    }

    fn destination_end(&self) -> u64 {
        self.destination_start + self.range_length
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
                        source_start: b,
                        range_length: c,
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


    let mut categories = almanac.categories.clone();

    let merged_category = Category::merge(&categories);

    println!("Final mapping: {:?}", merged_category);

    "".to_owned()
}




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
