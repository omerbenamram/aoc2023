use std::{
    cmp,
    collections::HashSet,
    ops::{Range, RangeInclusive},
};

use anyhow::Result;
use aoc2023::regex;
use itertools::Itertools;

#[derive(Debug)]
enum SchematicsSymbol {
    Number((usize, RangeInclusive<usize>)),
    Symbol((char, usize)), // position
}

struct Grid {
    inner: Vec<Vec<SchematicsSymbol>>,
}

impl Grid {}

fn part1(lines: &[String]) -> Result<i32> {
    let mut grid = vec![];
    // Collect numbers to a list of number, match_range (per row)
    let numbers_re = regex!(r"(\d+)");
    let symbols_re = regex!(r"([\*\+\$\#\!\@\#\$\%\^\&\*\\\/\=\-\;\:\?])");

    for line in lines {
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

    let mut valid = HashSet::new();
    for (i, line) in grid.iter().enumerate() {
        println!("{} {:?}", i, line);
        for symbol in line {
            match symbol {
                SchematicsSymbol::Number(number) => {
                    let scan_start = i.saturating_sub(1);
                    let scan_stop = cmp::min(i + 1, grid.len() - 1);
                    for line in grid[scan_start..=scan_stop].iter() {
                        println!("scanning {}-{} {:?}", scan_start, scan_stop, line);
                        for other in line {
                            match other {
                                SchematicsSymbol::Number(_) => {}
                                SchematicsSymbol::Symbol((_, position)) => {
                                    if valid.contains(&(i, number)) {
                                        println!("Already have {:?}", number);
                                        continue;
                                    }
                                    // within range
                                    println!("{:?} Trying: {:?}", symbol, other);
                                    if number.1.contains(&(*position)) // account for inclusive
                                        || (*number.1.end() == *position)
                                        || (number.1.start().saturating_sub(1)) == *position
                                    {
                                        println!("Valid {:?}", (i, number));
                                        valid.insert((i, number));
                                    } else {
                                        println!("Skipping {:?}", number)
                                    }
                                }
                            }
                        }
                    }
                }
                SchematicsSymbol::Symbol(_) => {}
            }
        }

        println!("--------------------");
    }

    Ok(valid.iter().map(|(_, number)| number.0 as i32).sum())
}

fn part2(games: &[&str]) -> Result<i32> {
    todo!()
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

    let input = &input.lines().map(|s| s.to_string()).collect_vec();

    assert_eq!(part1(&input).unwrap(), 4361);
    // assert_eq!(part2(&input).unwrap(), 2286);
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = &input.lines().map(|s| s.to_string()).collect_vec();
    println!("Part1: {}", part1(&input).unwrap());
    // println!("Part2: {}", part2(&input).unwrap());
}
