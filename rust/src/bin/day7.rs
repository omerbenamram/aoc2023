use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use aoc2023::regex;
use itertools::Itertools;

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
                11 | 0 => write!(f, "J"),
                12 => write!(f, "Q"),
                13 => write!(f, "K"),
                14 => write!(f, "A"),
                _ => unreachable!(),
            }?
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    // High card, where all cards' labels are distinct: 23456
    HighCard,
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    // Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind,
}

impl Hand {
    fn strength(&self) -> HandKind {
        // all cards are the same
        let counts = self.cards.iter().cloned().counts();
        let mut items = counts.into_iter().collect_vec();
        // sort by card count, and then by card strength
        items.sort_by_key(|item| (item.1, item.0));
        items.reverse();

        let (cards, amounts): (Vec<i32>, Vec<usize>) = items.iter().cloned().unzip();

        let kind = match (cards.as_slice(), amounts.as_slice()) {
            ([_], [5]) => HandKind::FiveOfAKind,
            // joker
            ([_, 0], [4, 1]) => HandKind::FiveOfAKind,
            ([0, _], [4, 1]) => HandKind::FiveOfAKind,
            ([_, 0], [3, 2]) => HandKind::FiveOfAKind,
            ([0, _], [3, 2]) => HandKind::FiveOfAKind,

            ([_, _], [4, 1]) => HandKind::FourOfAKind,
            ([_, _, 0], [3, 1, 1]) => HandKind::FourOfAKind,
            ([0, _, _], [3, 1, 1]) => HandKind::FourOfAKind,
            ([_, 0, _], [2, 2, 1]) => HandKind::FourOfAKind,

            ([_, _, 0], [2, 2, 1]) => HandKind::FullHouse,
            ([_, _], [3, 2]) => HandKind::FullHouse,

            ([_, _, _, 0], [2, ..]) => HandKind::ThreeOfAKind,
            ([0, _, _, _], [2, ..]) => HandKind::ThreeOfAKind,
            ([_, _, _], [3, 1, 1]) => HandKind::ThreeOfAKind,
            ([_, _, _], [2, 2, 1]) => HandKind::TwoPair,
            ([_, _, _, _], [2, ..]) => HandKind::OnePair,
            ([_, _, _, _, 0], [..]) => HandKind::OnePair,
            _ => HandKind::HighCard,
        };

        kind
    }

    fn from_str(s: &str, joker: bool) -> Result<Self> {
        let cards: Vec<i32> = s
            .chars()
            .into_iter()
            .map(|c| match c {
                c if c.is_ascii_digit() => Ok(c.to_digit(10).unwrap() as i32),
                'T' => Ok(10),
                'J' => Ok(if joker { 0 } else { 11 }),
                'Q' => Ok(12),
                'K' => Ok(13),
                'A' => Ok(14),
                _ => Err(anyhow::anyhow!("Invalid card: {}", c)),
            })
            .collect::<Result<_>>()?;

        Ok(Hand { cards })
    }

    fn camel_cmp(&self, other: &Hand) -> Ordering {
        match (self.strength(), other.strength()) {
            (mine, his) if mine == his => tie_break(&self.cards, &other.cards),
            (mine, his) => mine.cmp(&his),
        }
    }
}

#[test]
fn test_camel_cmp() {
    let h1 = Hand::from_str("78543", false).unwrap();
    let h2 = Hand::from_str("63529", false).unwrap();
    assert!(h1.camel_cmp(&h2).is_gt());
}

#[test]
fn test_strength() {
    assert!(matches!(
        Hand::from_str("98633", false).unwrap().strength(),
        HandKind::OnePair
    ));

    assert_eq!(
        Hand::from_str("87543", false).unwrap().strength(),
        HandKind::HighCard
    );

    assert_eq!(
        Hand::from_str("TTTTJ", true).unwrap().strength(),
        HandKind::FiveOfAKind
    );

    assert_eq!(
        Hand::from_str("32T3K", true).unwrap().strength(),
        HandKind::OnePair
    );

    assert_eq!(
        Hand::from_str("12JJ3", true).unwrap().strength(),
        HandKind::ThreeOfAKind
    );
    assert_eq!(
        Hand::from_str("22J33", true).unwrap().strength(),
        HandKind::FullHouse
    );

    for h in ["KTJJT", "QQQJA", "T55J5", "12JJJ"] {
        println!("{}", h);
        assert_eq!(
            Hand::from_str(h, true).unwrap().strength(),
            HandKind::FourOfAKind,
        );
    }
}

// true if a "beats" b, otherwise false;
fn tie_break(a: &[i32], b: &[i32]) -> Ordering {
    debug_assert!(a.len() == b.len());

    for i in 0..a.len() {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    Ordering::Equal
}

fn play(input: &Input) -> Result<i32> {
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

fn input(s: &str, joker: bool) -> Result<Input> {
    let re = regex!(r"([\d\w]+)\s(\d+)");
    let mut hands = vec![];

    for cap in re.captures_iter(s) {
        let hand = Hand::from_str(&cap[1], joker)?;
        let bid = cap[2].parse()?;
        hands.push((hand, bid));
    }

    Ok(hands)
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input_1 = input(&stdin_input, false).unwrap();
    println!("Part1: {}", play(&input_1).unwrap());
    let input = input(&stdin_input, true).unwrap();
    println!("Part2: {}", play(&input).unwrap());
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

    let input1 = input(&input_str, false).unwrap();
    assert_eq!(play(&input1).unwrap(), 6440);
    let input = input(&input_str, true).unwrap();
    assert_eq!(play(&input).unwrap(), 5905);
}
