use anyhow::{Context, Result};
use itertools::Itertools;

type Input = Vec<Vec<i64>>;

fn calc_deltas_recursive(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut deltas = vec![sequence.to_vec()];

    let mut current = sequence;
    let mut delta = vec![];

    loop {
        for w in current.windows(2) {
            delta.push(w[1] - w[0]);
        }

        deltas.push(delta.to_vec());

        if (delta.len() > 1) && (delta.iter().all(|&d| d == 0)) {
            break;
        }

        current = &deltas.last().expect("inserted at least one sequence");
        delta.clear();
    }

    deltas
}

fn part1(input: &Input) -> Result<i64> {
    let mut numbers = vec![];

    for sequence in input {
        let deltas = calc_deltas_recursive(sequence);
        numbers.push(
            deltas
                .iter()
                .map(|row| row.iter().last().expect("rows are not empty"))
                .sum::<i64>(),
        );
    }

    Ok(numbers.into_iter().sum())
}

fn part2(input: &Input) -> Result<i64> {
    let mut numbers: Vec<i64> = vec![];

    for sequence in input {
        let deltas = calc_deltas_recursive(sequence);
        let mut acc = 0;

        let mut idx = deltas.len() - 1;
        while idx >= 1 {
            let i1 = deltas[idx - 1].first().unwrap();
            acc = i1 - acc;
            idx -= 1;
        }
        numbers.push(acc);
    }

    Ok(numbers.into_iter().sum())
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().context("invalid number"))
                .collect()
        })
        .collect()
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse_input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[test]
fn test() {
    let input_str = textwrap::dedent(
        "0 3 6 9 12 15
         1 3 6 10 15 21
         10 13 16 21 30 45",
    );

    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 114);
    assert_eq!(part2(&input).unwrap(), 2);
}
