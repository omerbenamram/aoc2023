use anyhow::{Context, Result};
use itertools::Itertools;
use log::debug;

use std::{cmp, ops::Range, str::FromStr};

#[derive(Clone, Debug)]
struct RangeMap {
    dest_range_start: i64,
    source_range_start: i64,
    range_len: i64,
}

impl RangeMap {
    pub fn map(&self, n: i64) -> Option<i64> {
        let dst_end = self.source_range_start + self.range_len;
        let range = self.source_range_start..=dst_end;

        if range.contains(&n) {
            let offset = self.dest_range_start - self.source_range_start;
            Some(n + offset)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct MaterialMapping {
    #[allow(unused)]
    name: String,
    inner: Vec<RangeMap>,
}

// A ∩ B
fn intersect_ranges(a: Range<i64>, b: Range<i64>) -> Option<Range<i64>> {
    let start = cmp::max(a.start, b.start);
    let end = cmp::min(a.end, b.end);
    if start < end {
        Some(start..end)
    } else {
        None
    }
}

/// A - B
/// Given A, B -> will return sections present in A but not in B
/// A  |>----<|------|>-------<|
/// B          ------
///
///
/// A  |>-----------------<|----
/// B                       ------
///            C
fn subtract_ranges(a: Range<i64>, b: Range<i64>) -> Vec<Range<i64>> {
    let mut difference = Vec::new();
    if a.start < b.start {
        difference.push(a.start..b.start);
    }
    if a.end > b.end {
        difference.push(b.end..a.end);
    }
    difference
}

impl MaterialMapping {
    pub fn map(&self, n: i64) -> i64 {
        for mapping in &self.inner {
            if let Some(n) = mapping.map(n) {
                return n;
            }
        }
        n
    }

    pub fn map_range(&self, ns: Range<i64>) -> Vec<Range<i64>> {
        let mut ranges = Vec::new();
        let mut input = vec![ns];

        for mapping in &self.inner {
            let mut tmp = Vec::new();
            let dst_end = mapping.source_range_start + mapping.range_len;
            let range = mapping.source_range_start..dst_end;

            for other in &input {
                // If no intersection simply fall through
                if let Some(intersection) = intersect_ranges(other.clone(), range.clone()) {
                    let mapped_start = mapping
                        .map(intersection.start)
                        .expect("checked intersection");

                    // Intersecting range is mapped.
                    let mapped_end = mapping.map(intersection.end).expect("checked intersection");
                    ranges.push(mapped_start..mapped_end);

                    // We check the rest of the mappings not covered by intersection against other maps.
                    tmp.extend(subtract_ranges(other.clone(), range.clone()));
                } else {
                    tmp.push(other.clone());
                }
            }
            input = tmp;
        }

        ranges.extend(input);
        ranges
    }
}

#[derive(Clone, Debug)]
struct Alamnac {
    seeds: Vec<i64>,
    mappings: Vec<MaterialMapping>,
}

fn delimited_numbers(s: &str) -> Result<Vec<i64>> {
    s.split_whitespace()
        .map(|n| n.trim().parse::<i64>().context("expected number"))
        .collect::<Result<_>>()
}

impl FromStr for MaterialMapping {
    type Err = anyhow::Error;

    //seed-to-soil map:
    //50 98 2
    //52 50 48
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut lines = s.lines();

        let (name, _) = lines
            .next()
            .and_then(|l| l.trim().split_once(' '))
            .context("expetec first line: seed-to-soil map:")?;

        let inner = lines
            .map(|l| {
                let v = delimited_numbers(l)?;
                let dest_range_start = v[0];
                let source_range_start = v[1];
                let range_len = v[2];
                Ok(RangeMap {
                    dest_range_start,
                    source_range_start,
                    range_len,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            name: name.to_string(),
            inner,
        })
    }
}

impl FromStr for Alamnac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        // read header:
        let mut lines = s.lines();
        let (_, seeds) = lines
            .next()
            .and_then(|l| l.split_once(':'))
            .context("expetec first line: seeds: ...")?;

        let seeds = delimited_numbers(seeds)?;
        let mut mappings = vec![];
        let _ = lines.next(); // skip empty

        let mappings_text = lines.collect_vec();

        for (_, group) in &mappings_text.into_iter().group_by(|line| !line.is_empty()) {
            let s = group.collect::<Vec<&str>>();
            let s = s.join("\n");
            if s.is_empty() {
                continue;
            }
            mappings.push(MaterialMapping::from_str(&s).context("failed to parse group")?);
        }

        Ok(Alamnac { seeds, mappings })
    }
}

impl Alamnac {
    pub fn lowest_seed_numbers(&self) -> Vec<i64> {
        let mut result = Vec::new();
        for seed in self.seeds.clone() {
            let mut next = seed;

            for mapping in &self.mappings {
                next = mapping.map(next);
            }
            result.push(next);
        }

        result
    }

    pub fn lowest_seed_range_numbers(&self) -> i64 {
        let mut i = 0;
        let mut minimum = -1;

        while i <= self.seeds.len() - 2 {
            let pair = (self.seeds[i], self.seeds[i + 1]);
            debug!("=========({:?})==========", pair.0..pair.0 + pair.1);

            #[allow(clippy::single_range_in_vec_init)]
            let mut results = vec![pair.0..pair.0 + pair.1];

            for mapping in &self.mappings {
                debug!("{}", mapping.name);
                debug!("Current: {:#?}", results);
                let mut tmp = vec![];

                for range in results.iter() {
                    let new = mapping.map_range(range.clone());
                    tmp.extend(new)
                }

                let _ = std::mem::replace(&mut results, tmp);
            }

            debug_assert!(!results.is_empty());

            i += 2;

            if minimum < 0 {
                minimum = results.iter().map(|r| r.start).min().unwrap()
            } else {
                minimum = cmp::min(minimum, results.iter().map(|r| r.start).min().unwrap())
            }
        }

        minimum
    }
}

fn part1(input: &Alamnac) -> Result<i64> {
    input
        .lowest_seed_numbers()
        .iter()
        .cloned()
        .min()
        .context("expected at least one seed")
}

fn part2(input: &Alamnac) -> Result<i64> {
    Ok(input.lowest_seed_range_numbers())
}

fn input(input: &str) -> Result<Alamnac> {
    Alamnac::from_str(input)
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use log::LevelFilter;
    use maplit::hashset;

    use super::*;

    #[test]
    fn test_alamnac() {
        use env_logger::Builder;
        use std::io::Write;

        Builder::new()
            .is_test(true)
            .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
            .filter(None, LevelFilter::Debug)
            .init();

        let test_input = textwrap::dedent(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            ",
        );

        let input = input(&test_input).unwrap();
        assert_eq!(part1(&input).unwrap(), 35);
        assert_eq!(part2(&input).unwrap(), 46);
    }

    #[test]
    fn test_map_ranges() {
        // 77..100 -> 45..63
        // 45..64 -> 81..100
        // 64..77 -> 68..81
        let mapping = textwrap::dedent(
            "light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13",
        );

        let mapping = MaterialMapping::from_str(&mapping).unwrap();
        assert_eq!(mapping.map(77), 45);
        assert_eq!(mapping.map(88), 56);
        // first 77..88 is mapped to 45..56
        // then 74..77 should be mapped to 78..81
        assert_eq!(
            mapping
                .map_range(74..88)
                .into_iter()
                .collect::<HashSet<_>>(),
            hashset![78..81, 45..56]
        );
    }
}
