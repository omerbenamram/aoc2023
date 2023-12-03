use std::{cmp, collections::HashSet, ops::RangeInclusive, str::FromStr};

use anyhow::Result;
use aoc2023::regex;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SchematicsSymbol {
    Number((usize, RangeInclusive<usize>)),
    Symbol((char, usize)), // position
}

impl SchematicsSymbol {
    pub fn into_number(&self) -> usize {
        match self {
            SchematicsSymbol::Number((n, _)) => *n,
            _ => panic!("Called `into_number` on a `Symbol` variant"),
        }
    }
}

// Represent grid as tuples of positions and ranges
struct SchematicsGrid {
    pub inner: Vec<Vec<SchematicsSymbol>>,
}

impl FromStr for SchematicsGrid {
    type Err = ();

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut grid = vec![];
        // Collect numbers to a list of number, match_range (per row)
        let numbers_re = regex!(r"(\d+)");
        let symbols_re = regex!(r"([\*\+\$\#\!\@\#\$\%\^\&\*\\\/\=\-\;\:\?])");

        for line in s.lines() {
            let mut l = vec![];
            for number in numbers_re.captures_iter(line) {
                let m = number.get(0).unwrap();

                let r = m.range();
                l.push(SchematicsSymbol::Number((
                    m.as_str().parse().expect("regex matches numbers"),
                    r.start..=r.end,
                )))
            }

            for number in symbols_re.captures_iter(line) {
                let m = number.get(0).unwrap();

                l.push(SchematicsSymbol::Symbol((
                    m.as_str().chars().next().unwrap(),
                    m.range().start,
                )))
            }

            grid.push(l)
        }

        Ok(SchematicsGrid { inner: grid })
    }
}

impl SchematicsGrid {
    // for numbers, finds neighboring symbols, for symbols neighbor numbers
    pub fn neighbors(
        &self,
        line: usize,
        symbol: &SchematicsSymbol,
    ) -> HashSet<(usize, SchematicsSymbol)> {
        let grid = &self.inner;

        let scan_start = line.saturating_sub(1);
        let scan_stop = cmp::min(line + 1, grid.len() - 1);

        let mut neighbors = HashSet::<(usize, SchematicsSymbol)>::new();

        for (i, line) in grid[scan_start..=scan_stop].iter().enumerate() {
            for other in line {
                match (symbol, other) {
                    (SchematicsSymbol::Number(_), SchematicsSymbol::Number(_)) => {}
                    (SchematicsSymbol::Number(number), SchematicsSymbol::Symbol((_, position)))
                    | (SchematicsSymbol::Symbol((_, position)), SchematicsSymbol::Number(number)) =>
                    {
                        if number.1.contains(position) // account for inclusive
                            || (*number.1.end() == *position)
                            || (number.1.start().saturating_sub(1)) == *position
                        {
                            neighbors.insert((scan_start + i, other.clone()));
                        }
                    }
                    (SchematicsSymbol::Symbol(_), SchematicsSymbol::Symbol(_)) => {}
                }
            }
        }

        neighbors
    }
}

fn part1(grid: &SchematicsGrid) -> Result<i32> {
    let mut valid = HashSet::new();

    for (i, line) in grid.inner.iter().enumerate() {
        for symbol in line {
            if let SchematicsSymbol::Number(number) = symbol {
                if !grid.neighbors(i, &symbol).is_empty() {
                    valid.insert((i, number));
                }
            }
        }
    }

    Ok(valid.into_iter().map(|(_, number)| number.0 as i32).sum())
}

fn part2(grid: &SchematicsGrid) -> Result<i32> {
    let mut valid = HashSet::new();
    let mut sum = 0;

    for (i, line) in grid.inner.iter().enumerate() {
        for symbol in line {
            if let SchematicsSymbol::Symbol((s, _)) = symbol {
                if !valid.contains(&(i, symbol)) && *s == '*' {
                    let neighbors = grid
                        .neighbors(i, symbol)
                        .into_iter()
                        .map(|n| n.1.into_number())
                        .collect_vec();

                    if neighbors.len() == 2 {
                        valid.insert((i, symbol));

                        sum += neighbors[0] * neighbors[1]
                    }
                }
            }
        }
    }

    Ok(sum as i32)
}

#[test]
fn test() {
    let input = textwrap::dedent(
        "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    .....144.-
    ",
    );

    let input = SchematicsGrid::from_str(&input).unwrap();

    assert_eq!(part1(&input).unwrap(), 4361);
    assert_eq!(part2(&input).unwrap(), 467835);
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = SchematicsGrid::from_str(&input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}
