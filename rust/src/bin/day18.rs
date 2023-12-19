use anyhow::{bail, Context, Ok, Result};
use aoc2023::{regex, Direction};
use std::str::FromStr;

type Input = Instructions;

#[derive(Clone)]
struct Instructions(Vec<((Direction, i64), (Direction, i64))>);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let re = regex!(r"([RDLU]) (\d+) \(#([\d\w]+)\)");

        let inner = re
            .captures_iter(s)
            .map(|cap| {
                let direction = match cap[1].chars().next().unwrap() {
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    _ => unreachable!("regex cannot match this"),
                };
                let count = cap[2]
                    .parse::<i64>()
                    .context(format!("expected digit: `{}`", &cap[2]))?;
                let hex = cap[3].to_string();
                let hex_count = i64::from_str_radix(&hex[0..5], 16).unwrap();
                let hex_direction = match hex.chars().nth(5).unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => bail!("Unexpected digit"),
                };

                Ok(((direction, count), (hex_direction, hex_count)))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Instructions(inner))
    }
}

fn parse_input(s: &str) -> Result<Input> {
    Instructions::from_str(s)
}

fn solve(instructions: &[(Direction, i64)]) -> Result<i64> {
    let mut pos = (0., 0.);
    let mut line_string = vec![pos];

    for instruction in instructions {
        let count = instruction.1 as f64;
        let new_pos = match instruction.0 {
            Direction::Left => (pos.0, pos.1 - count),
            Direction::Right => (pos.0, pos.1 + count),
            Direction::Up => (pos.0 - count, pos.1),
            Direction::Down => (pos.0 + count, pos.1),
        };
        line_string.push(new_pos);
        pos = new_pos;
    }

    let boundry_len: i64 = instructions.iter().map(|i| i.1).sum();
    let area = shoelace_area(&line_string) as i64;

    let num_points = area - (boundry_len / 2) + 1;

    Ok(num_points + boundry_len as i64)
}

fn shoelace_area(coords: &[(f64, f64)]) -> f64 {
    let n = coords.len();
    let mut sum = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        sum += coords[i].0 * coords[j].1;
        sum -= coords[j].0 * coords[i].1;
    }
    sum.abs() / 2.0
}

fn part1(input: &Input) -> Result<i64> {
    let instructions: Vec<(Direction, i64)> = input.0.iter().cloned().map(|i| i.0).collect();
    solve(&instructions)
}

fn part2(input: &Input) -> Result<i64> {
    let instructions: Vec<(Direction, i64)> = input.0.iter().cloned().map(|i| i.1).collect();
    solve(&instructions)
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse_input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2023::testing::*;

    #[test]
    fn test() {
        init_logging();
        let input_str = textwrap::dedent(
            "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
        ",
        );

        let input = parse_input(&input_str).unwrap();
        assert_eq!(part1(&input).unwrap(), 62);
        assert_eq!(part2(&input).unwrap(), 952408144115);
    }
}
