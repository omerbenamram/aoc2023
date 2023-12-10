use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Context, Result};

type Node = (i64, i64);
type Input = HashMap<Node, Vec<Node>>;

fn part1(input: &Input) -> Result<i64> {
    todo!("bfs")
    // let mut q = VecDeque::new();
    // let mut visited = HashSet::new();

    // q.push_back(value);
}

fn part2(input: &Input) -> Result<i64> {
    todo!();
}

fn parse_input(input: &str) -> Result<Input> {
    let mut graph = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let i = i as i64;
            let j = j as i64;
            let e = graph.entry((i, j)).or_insert_with(Vec::new);

            match c {
                '.' => continue,
                '7' => {
                    e.push((i, j - 1));
                    e.push((i + 1, j));
                }
                '|' => {
                    e.push((i + 1, j));
                    e.push((i - 1, j))
                }
                '-' => {
                    e.push((i, j - 1));
                    e.push((i, j + 1));
                }
                'L' => {
                    e.push((i + 1, j));
                    e.push((i, j + 1));
                }
                'J' => {
                    e.push((i + 1, j));
                    e.push((i, j - 1));
                }
                // TODO: we assume start is connected everywhere, not sure if this is OK.
                'S' => {
                    e.push((i + 1, j));
                    e.push((i - 1, j));
                    e.push((i, j - 1));
                    e.push((i, j + 1));
                }
                _ => bail!("unexpected char '{}'", c),
            }
        }
    }

    Ok(graph)
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
        ".....
        .S-7.
        .|.|.
        .L-J.
        .....",
    );

    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 8);
    // assert_eq!(part2(&input).unwrap(), 2);
}
