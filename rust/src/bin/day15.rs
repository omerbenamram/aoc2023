use anyhow::{Context, Result};

type Input<'a> = Vec<&'a str>;

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
fn hash(string: &str) -> usize {
    let mut hash = 0;

    for char in string.chars() {
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
}

fn parse_input(s: &str) -> Result<Input> {
    Ok(s.split(',').collect())
}

fn part1(input: &Input) -> Result<i64> {
    Ok(input.into_iter().map(|s| hash(s) as i64).sum())
}

type Label<'a> = &'a str;
type Lens<'a> = (Label<'a>, i64);

fn part2(input: &Input) -> Result<i64> {
    let mut hash_table = vec![Vec::<Lens>::new(); 256];

    for instruction in input {
        if instruction.contains('=') {
            let (label, value) = instruction.split_once('=').expect("checked contains");
            let lens_power = value.parse().context("expected <label>=<number>")?;
            let slots = &mut hash_table[hash(label)];

            match slots.iter_mut().find(|lens| lens.0 == label) {
                Some(lens) => lens.1 = lens_power,
                None => slots.push((label, lens_power)),
            }
        } else {
            let label = instruction.trim_end_matches('-');
            let slots = &mut hash_table[hash(label)];

            // Remove
            slots.retain(|lens| lens.0 != label);
        }
    }

    // calc focusing power
    let mut focusing_power = 0;
    for (i, bucket) in hash_table.into_iter().enumerate() {
        for (j, s) in bucket.into_iter().enumerate() {
            focusing_power += (i + 1) * (j + 1) * s.1 as usize
        }
    }

    Ok(focusing_power as i64)
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse_input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[test]
fn test() {
    let input_str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 1320);
    assert_eq!(part2(&input).unwrap(), 145);
}
