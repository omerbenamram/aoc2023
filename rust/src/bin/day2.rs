use std::str::FromStr;

use itertools::Itertools;

use anyhow::{Error, Result};
use aoc2023::regex;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: i32,
    showings: Vec<Vec<(i32, Color)>>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.showings
            .iter()
            .flatten()
            .all(|showing| match showing.1 {
                Color::Red => showing.0 <= 12,
                Color::Green => showing.0 <= 13,
                Color::Blue => showing.0 <= 14,
            })
    }

    // The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
    pub fn power(&self) -> i32 {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for showing in self.showings.iter().flatten() {
            match showing.1 {
                Color::Red => max_red = max_red.max(showing.0),
                Color::Green => max_green = max_green.max(showing.0),
                Color::Blue => max_blue = max_blue.max(showing.0),
            }
        }

        max_red * max_green * max_blue
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"Game (?P<id>\d+): (?P<showings>.+)");
        let re_showing = regex!(r"(?P<count>\d+) (?P<color>\w+)");

        let caps = re
            .captures(s)
            .ok_or_else(|| Error::msg("Invalid game string"))?;

        let id = caps["id"].parse()?;

        let showings = caps["showings"]
            .split(';')
            .map(|showing| {
                let mut showings = vec![];
                for cap in re_showing.captures_iter(showing) {
                    let count = cap["count"]
                        .parse()
                        .ok()
                        .expect("this always matches a number");

                    let color = match &cap["color"] {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => return Err(Error::msg(format!("Invalid color `{}`", &cap["color"]))),
                    };
                    showings.push((count, color));
                }
                Ok(showings)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, showings })
    }
}

fn part1(games: &[Game]) -> Result<i32> {
    Ok(games
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum())
}

fn part2(games: &[Game]) -> Result<i32> {
    Ok(games.iter().map(|game| game.power()).sum())
}

#[test]
fn test() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let input = &input
        .lines()
        .map(|line| Game::from_str(&line).unwrap())
        .collect_vec();

    assert_eq!(part1(&input).unwrap(), 8);
    assert_eq!(part2(&input).unwrap(), 2286);
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = &input
        .lines()
        .map(|line| Game::from_str(&line).unwrap())
        .collect_vec();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}
