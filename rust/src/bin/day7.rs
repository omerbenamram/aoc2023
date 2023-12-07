use std::{cmp::Ordering, path::Display, str::FromStr};

use anyhow::Result;
use aoc2023::regex;
use itertools::{Either, Itertools};

type Bid = i32;
type Input = Vec<(Hand, Bid)>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Hand {
    cards: Vec<i32>,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.cards {
            match c {
                2..=9 => write!(f, "{}", c),
                10 => write!(f, "T"),
                11 => write!(f, "J"),
                12 => write!(f, "Q"),
                13 => write!(f, "K"),
                14 => write!(f, "A"),
                _ => unreachable!(),
            }?
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HandKind {
    // Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind(i32),
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind(i32, i32),
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse(i32, i32),
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind(i32, Vec<i32>),
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair(i32, i32, i32),
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair(i32, Vec<i32>),
    // High card, where all cards' labels are distinct: 23456
    HighCard(i32, Vec<i32>),
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let cards: Vec<i32> = s
            .chars()
            .into_iter()
            .map(|c| match c {
                c if c.is_ascii_digit() => Ok(c.to_digit(10).unwrap() as i32),
                'T' => Ok(10),
                'J' => Ok(11),
                'Q' => Ok(12),
                'K' => Ok(13),
                'A' => Ok(14),
                _ => Err(anyhow::anyhow!("Invalid card: {}", c)),
            })
            .collect::<Result<_>>()?;

        Ok(Hand { cards })
    }
}

impl Hand {
    fn strength(&self) -> HandKind {
        // all cards are the same
        let counts = self.cards.iter().cloned().counts();
        let mut items = counts.into_iter().collect_vec();
        // sort by card count
        items.sort_by_key(|item| item.1);
        items.reverse();

        let (cards, amounts): (Vec<i32>, Vec<usize>) = items.iter().cloned().unzip();

        let kind = match (cards.as_slice(), amounts.as_slice()) {
            (&[single], _) => HandKind::FiveOfAKind(single),
            (&[card1, card2], [4, 1]) => HandKind::FourOfAKind(card1, card2),
            (&[card1, card2], [3, 2]) => HandKind::FullHouse(card1, card2),
            (&[card1, card2, card3], [3, 1, 1]) => {
                HandKind::ThreeOfAKind(card1, vec![card2, card3])
            }
            (&[card1, card2, card3], [2, 2, 1]) => {
                if card1 > card2 {
                    HandKind::TwoPair(card1, card2, card3)
                } else {
                    HandKind::TwoPair(card2, card1, card3)
                }
            }
            (&[card1, card2, card3, card4], [2, ..]) => {
                HandKind::OnePair(card1, vec![card2, card3, card4])
            }
            _ => {
                let mut cards = self.cards.clone();
                cards.sort();
                let max_card = cards.pop().unwrap();
                cards.reverse();

                HandKind::HighCard(max_card, cards)
            }
        };

        kind
    }

    fn poker_cmp(&self, other: &Hand) -> Ordering {
        match (self.strength(), other.strength()) {
            (HandKind::FiveOfAKind(mine), HandKind::FiveOfAKind(other)) => mine.cmp(&other),
            (HandKind::FiveOfAKind(_), _) => Ordering::Greater,

            (HandKind::FourOfAKind(_, _), HandKind::FiveOfAKind(_)) => Ordering::Less,
            (
                HandKind::FourOfAKind(mine_four, mine_remaining),
                HandKind::FourOfAKind(other_four, other_remaining),
            ) => match mine_four.cmp(&other_four) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => mine_remaining.cmp(&other_remaining),
            },
            (HandKind::FourOfAKind(_, _), _) => Ordering::Greater,

            (HandKind::FullHouse(_, _), HandKind::FiveOfAKind(_) | HandKind::FourOfAKind(_, _)) => {
                Ordering::Less
            }
            (HandKind::FullHouse(mine_h, mine_l), HandKind::FullHouse(other_h, other_l)) => {
                match mine_h.cmp(&other_h) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => mine_l.cmp(&other_l),
                }
            }
            (HandKind::FullHouse(_, _), _) => Ordering::Greater,
            (
                HandKind::ThreeOfAKind(_, _),
                HandKind::FiveOfAKind(_) | HandKind::FourOfAKind(_, _) | HandKind::FullHouse(_, _),
            ) => Ordering::Less,

            (
                HandKind::ThreeOfAKind(mine_h, mine_rest),
                HandKind::ThreeOfAKind(other_h, other_rest),
            ) => match mine_h.cmp(&other_h) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => comapre_rests(&mine_rest, &other_rest),
            },
            (HandKind::ThreeOfAKind(_, _), _) => Ordering::Greater,

            (
                HandKind::TwoPair(mine_high, mine_high2, mine_remaining),
                HandKind::TwoPair(other_high, other_high2, other_remaining),
            ) => match mine_high.cmp(&other_high) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match mine_high2.cmp(&other_high2) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => mine_remaining.cmp(&other_remaining),
                },
            },
            (HandKind::TwoPair(_, _, _), HandKind::OnePair(_, _) | HandKind::HighCard(_, _)) => {
                Ordering::Greater
            }
            (HandKind::TwoPair(_, _, _), _) => Ordering::Less,

            (
                HandKind::OnePair(mine_high, mine_rest),
                HandKind::OnePair(other_high, other_rest),
            ) => match mine_high.cmp(&other_high) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => comapre_rests(&mine_rest, &other_rest),
            },
            (HandKind::OnePair(_, _), HandKind::HighCard(_, _)) => Ordering::Greater,
            (HandKind::OnePair(_, _), _) => Ordering::Less,
            (
                HandKind::HighCard(mine_high, mine_rest),
                HandKind::HighCard(other_high, other_rest),
            ) => match mine_high.cmp(&other_high) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => comapre_rests(&mine_rest, &other_rest),
            },
            (HandKind::HighCard(_, _), _) => Ordering::Less,
        }
    }

    fn camel_cmp(&self, other: &Hand) -> Ordering {
        match (self.strength(), other.strength()) {
            (HandKind::FiveOfAKind(mine), HandKind::FiveOfAKind(other)) => mine.cmp(&other),
            (HandKind::FiveOfAKind(_), _) => Ordering::Greater,

            (HandKind::FourOfAKind(_, _), HandKind::FiveOfAKind(_)) => Ordering::Less,
            (HandKind::FourOfAKind(_, _), HandKind::FourOfAKind(_, _)) => {
                comapre_rests(&self.cards, &other.cards)
            }
            (HandKind::FourOfAKind(_, _), _) => Ordering::Greater,

            (HandKind::FullHouse(_, _), HandKind::FiveOfAKind(_) | HandKind::FourOfAKind(_, _)) => {
                Ordering::Less
            }
            (HandKind::FullHouse(_, _), HandKind::FullHouse(_, _)) => {
                comapre_rests(&self.cards, &other.cards)
            }
            (HandKind::FullHouse(_, _), _) => Ordering::Greater,
            (
                HandKind::ThreeOfAKind(_, _),
                HandKind::FiveOfAKind(_) | HandKind::FourOfAKind(_, _) | HandKind::FullHouse(_, _),
            ) => Ordering::Less,

            (HandKind::ThreeOfAKind(_, _), HandKind::ThreeOfAKind(_, _)) => {
                comapre_rests(&self.cards, &other.cards)
            }
            (HandKind::ThreeOfAKind(_, _), _) => Ordering::Greater,

            (HandKind::TwoPair(_, _, _), HandKind::TwoPair(_, _, _)) => {
                comapre_rests(&self.cards, &other.cards)
            }
            (HandKind::TwoPair(_, _, _), HandKind::OnePair(_, _) | HandKind::HighCard(_, _)) => {
                Ordering::Greater
            }
            (HandKind::TwoPair(_, _, _), _) => Ordering::Less,

            (HandKind::OnePair(_, _), HandKind::OnePair(_, _)) => {
                comapre_rests(&self.cards, &other.cards)
            }
            (HandKind::OnePair(_, _), HandKind::HighCard(_, _)) => Ordering::Greater,
            (HandKind::OnePair(_, _), _) => Ordering::Less,
            (HandKind::HighCard(_, _), HandKind::HighCard(_, _)) => {
                comapre_rests(&self.cards, &other.cards)
            }
            (HandKind::HighCard(_, _), _) => Ordering::Less,
        }
    }
}

#[test]
fn test_camel_cmp() {
    let h1 = Hand::from_str("78543").unwrap();
    let h2 = Hand::from_str("63529").unwrap();
    assert!(h1.camel_cmp(&h2).is_gt());
}

#[test]
fn test_strength() {
    assert!(matches!(
        Hand::from_str("98633").unwrap().strength(),
        HandKind::OnePair(3, _)
    ));

    assert_eq!(
        Hand::from_str("87543").unwrap().strength(),
        HandKind::HighCard(8, vec![7, 5, 4, 3])
    );
}

// true if a "beats" b, otherwise false;
fn comapre_rests(a: &[i32], b: &[i32]) -> Ordering {
    // let mut a = a.to_vec();
    // a.sort();
    // a.reverse();
    // let mut b = b.to_vec();
    // b.sort();
    // b.reverse();

    debug_assert!(a.len() == b.len());

    for i in 0..a.len() {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    Ordering::Equal
}

fn part1(input: &Input) -> Result<i32> {
    let mut input = input.clone();
    input.sort_by(|hand_and_bid, other| hand_and_bid.0.camel_cmp(&other.0));

    for i in &input {
        print!("{}: {}  ", i.0, i.1);
        println!("{:?}", i.0.strength());
    }

    Ok(input
        .iter()
        .enumerate()
        .map(|(i, bid)| (i + 1) as i32 * bid.1)
        .sum::<i32>())
}

fn input(s: &str) -> Result<Input> {
    let re = regex!(r"([\d\w]+)\s(\d+)");
    let mut hands = vec![];

    for cap in re.captures_iter(s) {
        let hand = Hand::from_str(&cap[1])?;
        let bid = cap[2].parse()?;
        hands.push((hand, bid));
    }

    Ok(hands)
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    // println!("Part2: {}", part2(&input).unwrap());
}

#[test]
fn test() {
    let input_str = textwrap::dedent(
        "32T3K 765
         T55J5 684
         KK677 28
         KTJJT 220
         QQQJA 483",
    );

    let input = input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 6440);
    // assert_eq!(part2(&input).unwrap(), 71503);
}
