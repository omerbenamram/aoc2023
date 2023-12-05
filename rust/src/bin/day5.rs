use anyhow::{Context, Result};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};

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
        // number needs to be mapped
        if range.contains(&n) {
            // 52 -> 50 - if n is 54 then needs to be 52
            if self.source_range_start > self.dest_range_start {
                return Some(n - (self.source_range_start - self.dest_range_start));
            } else {
                return Some(n + (self.dest_range_start - self.source_range_start));
            };
        }

        None
    }
}

#[derive(Clone, Debug)]
struct MaterialMapping {
    name: String,
    inner: Vec<RangeMap>,
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

    pub fn map_range(&self, ns: RangeInclusive<i64>) -> Vec<RangeInclusive<i64>> {
        let mut ranges = Vec::new();
        let other = ns.clone();

        for mapping in &self.inner {
            let dst_end = mapping.source_range_start + mapping.range_len;

            let range = mapping.source_range_start..=dst_end;

            println!("checking map {:?}, against input {:?}", range, other);
            // first check if no overlap at all
            if range.end() < other.start() {
                // ranges.insert(other.clone());
                println!("no overlap");
                continue;
            }

            if range.start() > other.end() {
                // ranges.insert(other.clone());
                println!("no overlap");
                continue;
            }

            match (
                range.start().cmp(other.start()),
                range.end().cmp(other.end()),
            ) {
                // 1   3             10       15
                // ------------------         range
                //     |-------------|--------|  ns
                cmp @ (c1 @ std::cmp::Ordering::Less, std::cmp::Ordering::Less)
                | cmp @ (c1 @ std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                    debug_assert!(range.end() > other.start());
                    println!("{:?}", cmp);
                    // we have part that will be mapped, and part that will not be mapped.
                    let range_to_map = *other.start()..=*range.end();
                    let mapped_start = mapping.map(*other.start()).expect("checked to be in range");

                    let mapped_end = mapping.map(*range.end()).expect("checked to be in range");

                    let mapped_range = mapped_start..=mapped_end;
                    println!("mapping {:?} -> {:?}", range_to_map, mapped_range);
                    ranges.push(mapped_range);

                    let non_mapped = (*range.end() + 1)..=*other.end();
                    println!("adding {:?}", non_mapped);
                    ranges.push(non_mapped);
                }
                cmp @ (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater)
                | cmp @ (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                    debug_assert!(range.start() <= other.end());
                    println!("{:?}", cmp);

                    //      -----------------
                    // |----|------------|
                    // we have part that will be mapped, and part that will not be mapped.
                    let range_to_map = *range.start()..=*other.end();

                    let mapped_start = mapping.map(*range.start()).expect("checked to be in range");
                    let mapped_end = mapping.map(*other.end()).expect("checked to be in range");

                    let mapped_range = mapped_start..=mapped_end;
                    println!("mapping {:?} -> {:?}", range_to_map, mapped_range);
                    ranges.push(mapped_range);

                    let non_mapped = *other.start()..=(*range.start() - 1);
                    println!("adding {:?}", non_mapped);
                    ranges.push(non_mapped);
                }
                cmp @ (std::cmp::Ordering::Less, std::cmp::Ordering::Equal)
                | cmp @ (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater)
                | cmp @ (std::cmp::Ordering::Less, std::cmp::Ordering::Greater)
                | cmp @ (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                    println!("{:?}", cmp);
                    // entierty of ns covered and needs to be mapped
                    // need to find index and map
                    // --------------------------
                    //      ---------------------
                    let mapped_start = mapping.map(*other.start()).expect("checked to be in range");

                    let mapped_end = mapping.map(*other.end()).expect("checked to be in range");

                    let mapped_range = mapped_start..=mapped_end;
                    println!("mapping {:?} -> {:?}", other, mapped_range);
                    ranges.push(mapped_range);
                }
                cmp @ (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                    println!("{:?}", cmp);
                    println!("range: {:?} other: {:?}", range, other);
                    // we now have three ranges!
                    //    --------------------- range
                    // ----------------------------- ns
                    let mapped_start = mapping.map(*range.start()).expect("checked to be in range");
                    let mapped_end = mapping.map(*range.end()).expect("checked to be in range");

                    let mapped_range = mapped_start..=mapped_end;
                    println!("mapping {:?} -> {:?}", range, mapped_range);
                    ranges.push(mapped_range);

                    let non_mapped = *other.start()..=(*range.start() - 1);
                    println!("adding {:?}", non_mapped);
                    ranges.push(non_mapped);

                    let non_mapped = (*range.end() + 1)..=*other.end();
                    println!("adding {:?}", non_mapped);
                    ranges.push(non_mapped);
                }
            }
        }

        if ranges.is_empty() {
            vec![ns]
        } else {
            ranges
        }
    }
}

#[test]
fn test_map_ranges() {
    let mapping = textwrap::dedent(
        "light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13",
    );

    let mapping = MaterialMapping::from_str(&mapping).unwrap();
    assert_eq!(mapping.map_range(74..=88), vec![74..=77, 45..=56]);
}

#[derive(Clone, Debug)]
struct Alamnac {
    seeds: Vec<i64>,
    mappings: Vec<MaterialMapping>,
}

fn delimited_numbers(s: &str) -> Result<Vec<i64>> {
    s.trim()
        .split_whitespace()
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
            .and_then(|l| l.trim().split_once(" "))
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
            .and_then(|l| l.split_once(":"))
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
            let mut results = vec![pair.0..=pair.0 + pair.1];

            for mapping in &self.mappings {
                println!("{}", mapping.name);
                println!("Current: {:#?}", results);
                let mut tmp = vec![];

                for range in results.iter() {
                    let new = mapping.map_range(range.clone());
                    tmp.extend(new)
                }

                let _ = std::mem::replace(&mut results, tmp);
            }

            debug_assert!(results.len() >= 1);

            i += 2;

            if minimum < 0 {
                minimum = results.iter().map(|r| *r.start()).min().unwrap()
            } else {
                minimum = std::cmp::min(minimum, results.iter().map(|r| *r.start()).min().unwrap())
            }
            println!("min: {}", minimum);
        }

        minimum
    }
}

fn part1(input: &Alamnac) -> Result<i64> {
    Ok(input
        .lowest_seed_numbers()
        .iter()
        .cloned()
        .min()
        .context("expected at least one seed")?)
}

fn part2(input: &Alamnac) -> Result<i64> {
    Ok(input.lowest_seed_range_numbers())
}

fn input(input: &str) -> Result<Alamnac> {
    Alamnac::from_str(input)
}

#[test]
fn test() {
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
    // assert_eq!(part1(&input).unwrap(), 35);
    assert_eq!(part2(&input).unwrap(), 46);
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}
