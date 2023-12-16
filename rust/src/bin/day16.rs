use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use aoc2023::{CoordExt, GridExt};

use anyhow::{anyhow, Result};
use grid::Grid;
use rayon::prelude::*;

type Input = MirrorMaze;

#[derive(Clone, Copy)]
enum Tile {
    // '/'
    RightMirror,
    // '\'
    LeftMirror,
    // '.'
    Ground,
    // |
    VerticalSplitter,
    // -
    HorizontalSplitter,
}

#[derive(Clone)]
struct MirrorMaze(Grid<Tile>);

impl FromStr for MirrorMaze {
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
            .map(|c| match c {
                '/' => Ok(Tile::RightMirror),
                '\\' => Ok(Tile::LeftMirror),
                '.' => Ok(Tile::Ground),
                '|' => Ok(Tile::VerticalSplitter),
                '-' => Ok(Tile::HorizontalSplitter),
                _ => Err(anyhow!("Unknown tile")),
            })
            .collect::<Result<Vec<_>>>()?;
        let grid_arr = grid::Grid::from_vec(chars, shape.1);

        Ok(MirrorMaze(grid_arr))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn go(coordinate: (i64, i64), direction: Direction) -> (i64, i64) {
    match direction {
        Direction::Left => coordinate.left(),
        Direction::Right => coordinate.right(),
        Direction::Up => coordinate.up(),
        Direction::Down => coordinate.down(),
    }
}

impl MirrorMaze {
    pub fn num_energized_tiles(&self, start_at: (i64, i64), going: Direction) -> usize {
        let mut energized_tiles = HashSet::new();
        let mut visited_tiles = HashMap::new();

        let grid = &self.0;
        let mut beam_heads = VecDeque::new();

        beam_heads.push_back((start_at, going));
        energized_tiles.insert(start_at);

        while let Some((beam, direction)) = beam_heads.pop_front() {
            // if we didn't fall off map
            if let Some(tile) = grid.get_coordinate(beam) {
                energized_tiles.insert(beam);

                // Check if we've visited this tile in this direction before
                if visited_tiles.get(&(beam, direction)).is_some() {
                    // We've visited this tile in this direction before, so we're in a loop
                    continue;
                }
                // Mark this tile as visited in this direction
                visited_tiles.insert((beam, direction), true);

                match tile {
                    // '/'
                    Tile::RightMirror => {
                        let new_direction = match direction {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                        };
                        beam_heads.push_back((go(beam, new_direction), new_direction));
                    }
                    // '\'
                    Tile::LeftMirror => {
                        let new_direction = match direction {
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                        };
                        beam_heads.push_back((go(beam, new_direction), new_direction));
                    }
                    Tile::Ground => {
                        beam_heads.push_back((go(beam, direction), direction));
                    }
                    // '|'
                    Tile::VerticalSplitter => {
                        match direction {
                            Direction::Left | Direction::Right => {
                                beam_heads.push_back((go(beam, Direction::Up), Direction::Up));
                                beam_heads.push_back((go(beam, Direction::Down), Direction::Down));
                            }
                            Direction::Up | Direction::Down => {
                                beam_heads.push_back((go(beam, direction), direction));
                            }
                        };
                    }
                    // '-'
                    Tile::HorizontalSplitter => match direction {
                        Direction::Left | Direction::Right => {
                            beam_heads.push_back((go(beam, direction), direction));
                        }
                        Direction::Up | Direction::Down => {
                            beam_heads.push_back((go(beam, Direction::Left), Direction::Left));
                            beam_heads.push_back((go(beam, Direction::Right), Direction::Right));
                        }
                    },
                };
            }
        }

        energized_tiles.len()
    }
}

fn parse_input(s: &str) -> Result<Input> {
    MirrorMaze::from_str(s)
}

fn part1(input: &Input) -> Result<i64> {
    Ok(input.num_energized_tiles((0, 0), Direction::Right) as i64)
}

fn part2(input: &Input) -> Result<i64> {
    let ncols = input.0.cols();
    let nrows = input.0.rows();

    let mut inputs = vec![];

    for i in 0..ncols {
        inputs.push(((0, i as i64), Direction::Down));
        inputs.push(((nrows as i64, i as i64), Direction::Up));
    }

    for i in 0..nrows {
        inputs.push(((i as i64, 0), Direction::Right));
        inputs.push(((i as i64, ncols as i64), Direction::Left));
    }

    Ok(inputs
        .par_iter()
        .map(|(start_at, going)| input.num_energized_tiles(*start_at, *going))
        .max()
        .unwrap() as i64)
}

fn main() {
    let stdin_input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse_input(&stdin_input).unwrap();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[test]
fn test() {
    let input_str = r".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....    
    ";

    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 46);
    assert_eq!(part2(&input).unwrap(), 51);
}
