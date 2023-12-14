use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use ndarray::{Array2, ArrayView2};

type Input = Grid;

#[derive(Clone)]
struct Grid {
    pub inner: Array2<char>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut grid: Vec<Vec<char>> = vec![];

        for line in s.lines() {
            let line = line.trim();
            grid.push(line.chars().collect());
        }

        let shape = (grid.len(), grid[0].len());
        let chars = grid.clone().into_iter().flatten().collect::<Vec<_>>();
        let grid_arr = Array2::from_shape_vec(shape, chars).unwrap();

        Ok(Grid { inner: grid_arr })
    }
}

impl Grid {
    pub fn tilt_north(&mut self) {
        // skip first row
        loop {
            let mut rocks_moved = 0;
            for i in 1..self.inner.nrows() {
                for j in 0..self.inner.ncols() {
                    let tile = self.inner[(i, j)];

                    if tile == 'O' {
                        let north_tile = self.inner[(i - 1, j)];

                        if north_tile == '.' {
                            self.inner[(i, j)] = '.';
                            self.inner[(i - 1, j)] = 'O';
                            rocks_moved += 1;
                        }
                    }
                }
            }
            if rocks_moved == 0 {
                break;
            }
        }
    }

    pub fn tilt_west(&mut self) {
        // skip first row

        loop {
            let mut rocks_moved = 0;
            for c in 1..self.inner.ncols() {
                for r in 0..self.inner.nrows() {
                    let tile = self.inner[(r, c)];

                    if tile == 'O' {
                        let west_tile = self.inner[(r, c - 1)];

                        if west_tile == '.' {
                            self.inner[(r, c)] = '.';
                            self.inner[(r, c - 1)] = 'O';
                            rocks_moved += 1;
                        }
                    }
                }
            }
            if rocks_moved == 0 {
                break;
            }
        }
    }

    pub fn tilt_east(&mut self) {
        // skip first row

        loop {
            let mut rocks_moved = 0;
            for c in (0..self.inner.ncols() - 1).rev() {
                for r in 0..self.inner.nrows() {
                    let tile = self.inner[(r, c)];

                    if tile == 'O' {
                        let east_tile = self.inner[(r, c + 1)];

                        if east_tile == '.' {
                            self.inner[(r, c)] = '.';
                            self.inner[(r, c + 1)] = 'O';
                            rocks_moved += 1;
                        }
                    }
                }
            }
            if rocks_moved == 0 {
                break;
            }
        }
    }

    pub fn tilt_south(&mut self) {
        // skip first row
        loop {
            let mut rocks_moved = 0;
            for i in (0..self.inner.nrows() - 1).rev() {
                for j in 0..self.inner.ncols() {
                    let tile = self.inner[(i, j)];

                    if tile == 'O' {
                        let south_tile = self.inner[(i + 1, j)];

                        if south_tile == '.' {
                            self.inner[(i, j)] = '.';
                            self.inner[(i + 1, j)] = 'O';
                            rocks_moved += 1;
                        }
                    }
                }
            }
            if rocks_moved == 0 {
                break;
            }
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn load(&self) -> i64 {
        let mut load = 0;
        for i in 0..self.inner.nrows() {
            for j in 0..self.inner.ncols() {
                if self.inner[(i, j)] == 'O' {
                    load += self.inner.nrows() - i
                }
            }
        }
        load as i64
    }
}

fn parse_input(input: &str) -> Result<Input> {
    Grid::from_str(input)
}

fn part1(input: &Input) -> Result<i64> {
    let mut g: Grid = input.clone();

    g.tilt_north();

    Ok(g.load())
}

fn part2(input: &Input) -> Result<i64> {
    todo!("find the cycles automatically");
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
        "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....",
    );

    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 136);
    assert_eq!(part2(&input).unwrap(), 64);
}
