use petgraph::graphmap::UnGraphMap;
use petgraph::{algo::dijkstra, graphmap::DiGraphMap};
use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Context, Result};

type Node = (i64, i64);
// Start, Graph
type Input = (Node, HashMap<Node, Vec<Node>>);

fn part1(input: &Input) -> Result<i64> {
    let mut graph = DiGraphMap::new();
    for (node, edges) in &input.1 {
        for edge in edges {
            graph.add_edge(*node, *edge, 1);
        }
    }

    let start = input.0;
    let distances = dijkstra(&graph, start, None, |_| 1);
    let max_distance = *distances.values().max().unwrap_or(&0);
    Ok(max_distance)
}

fn part2(input: &Input) -> Result<i64> {
    todo!();
}

fn parse_input(input: &str) -> Result<Input> {
    let mut graph = HashMap::new();
    let mut start = None;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().filter(|i| !i.is_whitespace()).enumerate() {
            let i = i as i64;
            let j = j as i64;
            let e = graph.entry((i, j)).or_insert_with(Vec::new);

            match c {
                '.' => continue,
                'F' => {
                    e.push((i, j + 1));
                    e.push((i + 1, j));
                }
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
                    e.push((i - 1, j));
                    e.push((i, j + 1));
                }
                'J' => {
                    e.push((i - 1, j));
                    e.push((i, j - 1));
                }
                'S' => {
                    start = Some((i, j));
                }
                _ => bail!("unexpected char '{}'", c),
            }
        }
    }

    // because start shape is not given, we have to extrapolate it after finishing
    // building the graph.
    if let Some(start) = start {
        let mut edges = vec![];
        for (i, j) in graph.keys() {
            if graph.get(&(*i, *j)).unwrap().contains(&start) {
                edges.push((*i, *j));
            }
        }
        graph.entry(start).or_insert_with(Vec::new).extend(&edges)
    }

    Ok((
        start.context("expected graph to contain start node")?,
        graph,
    ))
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
    assert_eq!(part1(&input).unwrap(), 4);
    // assert_eq!(part2(&input).unwrap(), 2);

    let input_str = textwrap::dedent(
        "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...",
    );
    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 8);
}
