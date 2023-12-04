use anyhow::{Context, Result};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<i32>,
    card_numbers: HashSet<i32>,
}

impl Card {
    pub fn n_matching_numbers(&self) -> usize {
        self.card_numbers
            .intersection(&self.winning_numbers)
            .count()
    }

    pub fn score(&self) -> i32 {
        match self.n_matching_numbers() {
            0 => 0,
            1 => 1,
            n => 2i32.pow((n - 1) as u32),
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(":").context("Expected `Card :...`")?;
        let (winning_numbers, card_numbers) = numbers
            .split_once("|")
            .context("expected <winning_numbers> | <card_numbers>")?;

        let winning_numbers = winning_numbers
            .trim()
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>().context("expected number"))
            .collect::<Result<_>>()?;

        let card_numbers = card_numbers
            .trim()
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>().context("expected number"))
            .collect::<Result<_>>()?;

        Ok(Card {
            winning_numbers,
            card_numbers,
        })
    }
}

fn input(input: &str) -> Result<Vec<Card>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|c| Card::from_str(c))
        .collect::<Result<_>>()
}

fn part1(cards: &[Card]) -> Result<i32> {
    Ok(cards.iter().map(|c| c.score()).sum())
}

fn part2(cards: &[Card]) -> Result<i32> {
    let mut count = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let n_matching = card.n_matching_numbers();

        let amount_of_cards = count[i];

        for idx in (i + 1)..(i + 1 + n_matching) {
            count[idx] += 1 * amount_of_cards;
        }
    }

    Ok(count.into_iter().sum())
}

#[test]
fn test() {
    let test_input = textwrap::dedent(
        "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    );

    let input = input(&test_input).unwrap();
    assert_eq!(part1(&input).unwrap(), 13);
    assert_eq!(part2(&input).unwrap(), 30);
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}
