use itertools::Itertools;
use regex::Regex;

use anyhow::{bail, Result};

fn part1(calibration_values: &[&str]) -> Result<i32> {
    let mut sum = 0;

    for line in calibration_values {
        let chars = line.chars().filter(|c| c.is_ascii_digit()).collect_vec();

        if chars.is_empty() {
            bail!("Expected at least one digit");
        }

        let code1 = chars.first().expect("non empty");
        let code2 = chars.last().expect("non empty");

        sum += format!("{}{}", code1, code2)
            .parse::<i32>()
            .expect("two digits will produce a valid number");
    }

    Ok(sum)
}

fn to_digit(s: &str) -> Result<i32> {
    let c = s
        .chars()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty string"))?;

    Ok(match c {
        '0'..='9' => s.parse().unwrap(),
        _ => match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => anyhow::bail!("String is not a number"),
        },
    })
}

fn part2(calibration_values: &[&str]) -> Result<i32> {
    let re =
        Regex::new("(\\d|one|two|three|four|five|six|seven|eight|nine)").expect("regex is valid");

    let mut sum = 0;

    for line in calibration_values {
        let mut start = 0;
        let mut matches = vec![];

        // allow overlapping matches
        while let Some(mat) = re.find(&line[start..]) {
            matches.push(mat);
            start += mat.start() + 1;
        }

        if matches.is_empty() {
            bail!("Expected at least one digit");
        }

        let code1 = matches.first().expect("non empty").as_str();
        let code2 = matches.last().expect("non empty").as_str();

        sum += format!("{}{}", to_digit(code1)?, to_digit(code2)?)
            .parse::<i32>()
            .expect("two digits will produce a valid number");
    }

    Ok(sum)
}

#[test]
fn test_part2() {
    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    eighthree"; // overlapping

    assert_eq!(
        part2(&input.lines().filter(|line| !line.is_empty()).collect_vec()).unwrap(),
        281 + 83
    );
}

#[test]
fn test_part1() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    assert_eq!(
        part1(&input.lines().filter(|line| !line.is_empty()).collect_vec()).unwrap(),
        142
    );
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = &input.lines().collect_vec();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}
