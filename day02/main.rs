use std::{fs, str::FromStr};


#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err("Failed to parse CubeColor".to_owned()),
        }
    }
}

type RevealedCube = (CubeColor, u32);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<RevealedCube>>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("Game ").ok_or("Parsing failed")?;
        let id_str: Vec<char> = stripped.chars().take_while(|c| c.is_digit(10)).collect();
        let sets: Vec<Vec<RevealedCube>> = (&stripped[(id_str.len() + 2)..])
            .split(';')
            .map(|set_str| {
                set_str
                    .trim()
                    .split(',')
                    .filter_map(|str| {
                        let (count_str, color_str) = str.trim().split_once(' ')?;
                        let cube_color = CubeColor::from_str(color_str).ok()?;
                        let cube_count: u32 = count_str.parse().ok()?;

                        Some((cube_color, cube_count))
                    })
                    .collect()
            }).collect();

        Ok(
            Game {
                id: id_str.iter().collect::<String>().parse().map_err(|_| "Failed to parse game id")?,
                sets,
            }
        )
    }
}


fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let games: Vec<Game> = contents
        .trim()
        .lines()
        .map(|line| Game::from_str(line).expect(&format!("Failed to parse line {}", line)))
        .collect(); 


    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}


fn part1(games: &Vec<Game>) -> String {

    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;

    let answer = games
        .iter()
        .fold(0, |sum, game| {
            let valid = game.sets
                .iter()
                .all(|sets| {
                    sets
                        .iter()
                        .all(|(color, count)| {
                            match color {
                                CubeColor::Red => red_count >= *count,
                                CubeColor::Green => green_count >= *count,
                                CubeColor::Blue => blue_count >= *count,
                            }
                        })
                });

            if valid {
                return sum + game.id;
            }

            sum
        });

    answer.to_string()
}


fn part2(games: &Vec<Game>) -> String {
    let ans = games
        .iter()
        .fold(0, |ans, game| {
            let [red_count, green_count, blue_count] = game.sets
                .iter()
                .map(|sets| {
                    let get_count = |color| {
                        sets
                            .iter()
                            .find(|x| x.0 == color)
                            .map(|x| x.1)
                            .unwrap_or(0)
                    };

                    let red_count = get_count(CubeColor::Red);
                    let green_count = get_count(CubeColor::Green);
                    let blue_count = get_count(CubeColor::Blue);

                    [red_count, green_count, blue_count]
                })
                .fold([0, 0, 0], |max, set| {
                    [
                        max[0].max(set[0]),
                        max[1].max(set[1]),
                        max[2].max(set[2]),
                    ]
                });

            let power = red_count * blue_count * green_count;
            
            ans + power
        });

    ans.to_string()
}
