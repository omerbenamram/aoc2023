use anyhow::{anyhow, Result};
use std::{collections::HashMap, str::FromStr};

type Input = Grid;

#[derive(Clone)]
struct Grid {
    pub inner: grid::Grid<char>,
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
        let grid_arr = grid::Grid::from_vec(chars, shape.1);

        Ok(Grid { inner: grid_arr })
    }
}

impl Grid {
    pub fn tilt_north(&mut self) {
        // skip first row
        loop {
            let mut rocks_moved = 0;
            for i in 1..self.inner.rows() {
                for j in 0..self.inner.cols() {
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
        self.inner.rotate_right();
        self.tilt_north();
        self.inner.rotate_left();
    }

    pub fn tilt_east(&mut self) {
        self.inner.rotate_left();
        self.tilt_north();
        self.inner.rotate_right();
    }

    pub fn tilt_south(&mut self) {
        self.inner.rotate_half();
        self.tilt_north();
        self.inner.rotate_half();
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn load(&self) -> i64 {
        let mut load = 0;
        for i in 0..self.inner.rows() {
            for j in 0..self.inner.cols() {
                if self.inner[(i, j)] == 'O' {
                    load += self.inner.rows() - i
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
    let mut seen = HashMap::new();
    let mut scores = HashMap::new();
    let maxval = 1_000_000_000;

    let mut array = input.clone();
    let mut i = 0;

    while i < maxval {
        // hash array state, ugly, but works.
        let h = array.inner.iter().collect::<String>();

        if seen.contains_key(&h) {
            let cycle_length = i - seen[&h];
            let index = seen[&h] + (maxval - seen[&h]) % cycle_length;
            return Ok(*scores.get(&index).unwrap());
        }

        seen.insert(h, i);
        scores.insert(i, array.load());
        array.cycle();
        i += 1;
    }

    Err(anyhow!("solution not found"))
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
