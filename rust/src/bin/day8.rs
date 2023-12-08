use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use aoc2023::regex;
use maplit::hashset;

type Instructions = String;
type Graph = HashMap<String, Vec<String>>;
type Input = (Instructions, Graph);

fn input(s: &str) -> Result<Input> {
    let mut lines = s.lines();
    let instructions = lines.next().context("expected input to be not empty")?;
    lines.next().context("expected blank line")?; // skip newline

    //AAA = (BBB, CCC)
    let re = regex!(r"(?P<node>\w{3}) = \((?P<left>\w{3}),\s(?P<right>\w{3})\)");

    let mut graph = Graph::new();

    for line in lines.into_iter() {
        let capture = re.captures(line.trim()).context("invalid input")?;
        let node = capture["node"].to_string();
        let left = capture["left"].to_string();
        let right = capture["right"].to_string();

        let e = graph.entry(node).or_insert_with(Vec::new);
        e.push(left);
        e.push(right);
    }

    Ok((instructions.to_string(), graph))
}

fn part1(input: &Input) -> Result<i64> {
    path_len_from(&input.1, &input.0, "AAA")
}

// Finds path len to a node terminating at "Z", starting from some node.
fn path_len_from(graph: &Graph, instructions: &str, node: &str) -> Result<i64> {
    let mut steps = 0;
    let mut at = node;

    for inst in instructions.chars().cycle() {
        match inst {
            'L' => at = &graph[at][0],
            'R' => at = &graph[at][1],
            _ => bail!("Unexpected instruction"),
        }
        steps += 1;
        if at.ends_with("Z") {
            break;
        }
    }

    Ok(steps)
}

// Finds path len to a node terminating at "Z", starting from some node.
fn cycle_len(graph: &Graph, instructions: &str, node: &str) -> Result<i64> {
    let mut steps = 0;
    let mut at = node;

    for inst in instructions.chars().cycle() {
        match inst {
            'L' => at = &graph[at][0],
            'R' => at = &graph[at][1],
            _ => bail!("Unexpected instruction"),
        }
        steps += 1;
        if at == node {
            break;
        }
    }

    Ok(steps)
}

fn prime_factorization(i: i64) -> Vec<i64> {
    let mut factors = vec![];
    for p in 2..i {
        if i % p == 0 {
            factors.push(p as i64)
        }
    }

    factors
}

fn part2(input: &Input) -> Result<i64> {
    let instuctions = &input.0;
    let graph = &input.1;

    // Start at all nodes that end with "A"
    let at = graph
        .keys()
        .filter(|node| node.ends_with("A"))
        .cloned()
        .collect::<Vec<String>>();

    let mut prime_factors = hashset! {};
    for node in at {
        let path_len = path_len_from(graph, instuctions, &node)?;
        prime_factors.extend(prime_factorization(path_len));
    }

    Ok(prime_factors.into_iter().product())
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[test]
fn test_no_cycle() {
    let input_str = textwrap::dedent(
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)",
    );

    let input = input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 2);
}

#[test]
fn test_cycle() {
    let input_str = textwrap::dedent(
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)",
    );

    let input = input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 6);
}

#[test]
fn test_part2() {
    let input_str = textwrap::dedent(
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)",
    );

    let input = input(&input_str).unwrap();
    assert_eq!(part2(&input).unwrap(), 6);
}
