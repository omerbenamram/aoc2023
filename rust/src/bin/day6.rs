use anyhow::{Context, Result};
use aoc2023::regex;
use itertools::Itertools;
use roots::{find_roots_quadratic, Roots};

type Input = Vec<(i64, i64)>;

/// x * y > distance
/// x + y = time
/// solve for x
fn get_num_ways_to_win(time: i64, distance: i64) -> Option<i64> {
    if let Roots::Two([start, stop]) = find_roots_quadratic(1.0, -time as f64, distance as f64) {
        let mut num_valid_numbers = (stop.floor() - start.ceil()) as i64 + 1;

        // if we have perfect integer roots than we need to exclude them (since we need closest integer that is strictly less than the root)
        if stop.fract() == 0.0 {
            num_valid_numbers -= 1;
        }

        if start.fract() == 0.0 {
            num_valid_numbers -= 1;
        }
        Some(num_valid_numbers)
    } else {
        None
    }
}

fn part1(input: &Input) -> Result<i64> {
    let mut result = 1;
    for problem in input {
        let (time, distance) = (problem.0, problem.1);

        result *= get_num_ways_to_win(time, distance).context("problem is unsolvable")?;
    }

    Ok(result)
}

fn part2(input: &Input) -> Result<i64> {
    let mut merged_time = String::new();
    let mut merged_distance = String::new();

    for problem in input {
        let (time, distance) = (problem.0, problem.1);
        merged_time += &format!("{}", time);
        merged_distance += &format!("{}", distance);
    }

    get_num_ways_to_win(
        merged_time.parse().unwrap(),
        merged_distance.parse().unwrap(),
    )
    .context("problem is unsolvable")
}

// Time:      7  15   30
// Distance:  9  40  200
fn input(input: &str) -> Result<Input> {
    let numbers = regex!(r"(\d+)");
    let mut lines = input.lines();

    let time = lines
        .next()
        .map(|line| {
            numbers
                .captures_iter(line)
                .map(|n| n[0].parse::<i64>().expect("regex matches number"))
                .collect_vec()
        })
        .context("expected non empty input")?;

    let distance = lines
        .next()
        .map(|line| {
            numbers
                .captures_iter(line)
                .map(|n| n[0].parse::<i64>().expect("regex matches number"))
                .collect_vec()
        })
        .context("expected input to have 2 lines")?;

    anyhow::ensure!(lines.next().is_none(), "Expected exactly 2 lines");

    Ok(time.into_iter().zip(distance).into_iter().collect_vec())
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[test]
fn test() {
    let input_str = textwrap::dedent(
        "Time:      7  15   30
         Distance:  9  40  200",
    );

    let input = input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 288);
    assert_eq!(part2(&input).unwrap(), 71503);
}
