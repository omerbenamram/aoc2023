use anyhow::{Context, Result};

type Input = Vec<Vec<i64>>;

fn calc_deltas_recursive(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut deltas = vec![sequence.to_vec()];

    let mut current = sequence;

    while {
        let delta: Vec<i64> = current.windows(2).map(|w| w[1] - w[0]).collect();
        let stop = delta.iter().all(|&d| d == 0); // stop when all delta is zeros.
        deltas.push(delta); // calculate stop first to avoid clone :)
        !stop
    } {
        current = deltas.last().unwrap();
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
    let numbers: Vec<i64> = input
        .iter()
        .map(|sequence| {
            let deltas = calc_deltas_recursive(sequence);
            deltas
                .iter()
                .rev()
                .skip(1)
                .fold(0, |acc, row| row.first().expect("rows are not empty") - acc)
        })
        .collect();

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
