use std::str::FromStr;

use aoc2023::{CoordExt, Direction, GridExt};

use anyhow::{bail, Context, Result};
use env_logger::Builder;
use grid::Grid;
use log::debug;
use pathfinding::prelude::dijkstra;

type Input = HeatLossMaze;

#[derive(Clone)]
struct HeatLossMaze(Grid<i64>);

impl FromStr for HeatLossMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut grid: Vec<Vec<char>> = vec![];

        for line in s.lines() {
            let line = line.trim();
            grid.push(line.chars().collect());
        }

        let shape = (grid.len(), grid[0].len());
        let chars = grid
            .clone()
            .into_iter()
            .flatten()
            .map(|c| i64::from_str(&format!("{}", c)).context("expected number"))
            .collect::<Result<Vec<_>>>()?;
        let grid_arr = grid::Grid::from_vec(chars, shape.1);

        Ok(HeatLossMaze(grid_arr))
    }
}

fn go(coordinate: (i64, i64), direction: Direction) -> (i64, i64) {
    match direction {
        Direction::Left => coordinate.left(),
        Direction::Right => coordinate.right(),
        Direction::Up => coordinate.up(),
        Direction::Down => coordinate.down(),
    }
}

type PossibleCrucibleMove = ((i64, i64), Direction, i32);

impl HeatLossMaze {
    pub fn min_heatloss_path(&self) -> Result<i64> {
        let possible_starts = vec![((0, 0), Direction::Down, 0), ((0, 0), Direction::Right, 0)];

        let mut distances = vec![];

        for start in possible_starts {
            let p: Option<(Vec<PossibleCrucibleMove>, i64)> = dijkstra(
                &start,
                |(c, d, dc)| {
                    let mut possible = vec![];

                    for direction in &[
                        Direction::Left,
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                    ] {
                        let next_coord = go(*c, *d);
                        if *direction == d.opposite() {
                            continue;
                        }

                        if direction == d {
                            if *dc < 2 {
                                let next = (next_coord, *direction, *dc + 1);
                                if let Some(weight) = self.0.get_coordinate(next_coord) {
                                    possible.push((next, *weight));
                                }
                            }
                        } else {
                            let next = (next_coord, *direction, 0);
                            if let Some(weight) = self.0.get_coordinate(next_coord) {
                                possible.push((next, *weight));
                            }
                        }
                    }
                    possible
                },
                |((r, c), _, _)| {
                    (*r == (self.0.rows() - 1) as i64) && (*c == (self.0.cols() - 1) as i64)
                },
            );
            if let Some((path, cost)) = &p {
                for node in path {
                    debug!("{:?}", node)
                }

                distances.push(*cost);
            }
        }

        distances.into_iter().min().context("no path found")
    }

    pub fn min_heatloss_path_ultra(&self) -> Result<i64> {
        let possible_starts = vec![((0, 0), Direction::Down, 0), ((0, 0), Direction::Right, 0)];

        let mut distances = vec![];

        for start in possible_starts {
            debug!("======= {:?} ========", start);
            let p: Option<(Vec<PossibleCrucibleMove>, i64)> = dijkstra(
                &start,
                |(c, d, dc)| {
                    let mut possible = vec![];

                    if *dc <= 3 {
                        let next_coord = go(*c, *d);
                        let next = (next_coord, *d, *dc + 1);
                        if let Some(weight) = self.0.get_coordinate(next_coord) {
                            possible.push((next, *weight));
                        }

                        debug!("from: {:?} possible {:?}", (c, d, dc), possible);
                        return possible;
                    }

                    for direction in &[
                        Direction::Left,
                        Direction::Up,
                        Direction::Down,
                        Direction::Right,
                    ] {
                        let next_coord = go(*c, *direction);

                        if *direction == d.opposite() {
                            continue;
                        }

                        let next = if *dc > 3 && *dc <= 9 {
                            if direction == d {
                                Some((next_coord, *direction, *dc + 1))
                            } else {
                                Some((next_coord, *direction, 1))
                            }
                        } else {
                            if direction != d {
                                Some((next_coord, *direction, 1))
                            } else {
                                None
                            }
                        };

                        if let Some(next) = next {
                            if let Some(weight) = self.0.get_coordinate(next_coord) {
                                possible.push((next, *weight))
                            }
                        }
                    }
                    debug!("from: {:?} possible {:?}", (c, d, dc), possible);
                    possible
                },
                |((r, c), _, dc)| {
                    (*r == (self.0.rows() - 1) as i64)
                        && (*c == (self.0.cols() - 1) as i64)
                        && *dc >= 4
                },
            );

            if let Some((path, cost)) = &p {
                for node in path {
                    debug!("{:?}", node)
                }

                distances.push(*cost);
            }
        }

        distances.into_iter().min().context("no path found")
    }
}

fn parse_input(s: &str) -> Result<Input> {
    HeatLossMaze::from_str(s)
}

fn part1(input: &Input) -> Result<i64> {
    input.min_heatloss_path()
}

fn part2(input: &Input) -> Result<i64> {
    input.min_heatloss_path_ultra()
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse_input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2023::testing::*;

    #[test]
    fn test() {
        init_logging();
        let input_str = r"2413432311323
                          3215453535623
                          3255245654254
                          3446585845452
                          4546657867536
                          1438598798454
                          4457876987766
                          3637877979653
                          4654967986887
                          4564679986453
                          1224686865563
                          2546548887735
                          4322674655533
        ";

        let input = parse_input(&input_str).unwrap();
        assert_eq!(part1(&input).unwrap(), 102);
        assert_eq!(part2(&input).unwrap(), 94);
    }

    #[test]
    fn test_ultra() {
        init_logging();
        let input_str = r"111111111111
                          999999999991
                          999999999991
                          999999999991
                          999999999991
        ";

        let input = parse_input(&input_str).unwrap();
        assert_eq!(part2(&input).unwrap(), 71);
    }
}
