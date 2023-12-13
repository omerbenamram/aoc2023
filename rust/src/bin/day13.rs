use anyhow::Result;
use ndarray::{Array2, ArrayView2};

type Input = Vec<Array2<char>>;

fn parse_input(input: &str) -> Result<Input> {
    let mut grids = vec![];

    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            let shape = (grid.len(), grid[0].len());
            let chars = grid.clone().into_iter().flatten().collect::<Vec<_>>();
            let grid_arr = Array2::from_shape_vec(shape, chars).unwrap();
            grids.push(grid_arr);
            grid.clear();
            continue;
        }

        grid.push(line.chars().collect());
    }

    // last has no space after
    let shape = (grid.len(), grid[0].len());
    let chars = grid.clone().into_iter().flatten().collect::<Vec<_>>();
    let grid_arr = Array2::from_shape_vec(shape, chars).unwrap();
    grids.push(grid_arr);

    Ok(grids)
}

fn iter_mirrors<'a: 'b, 'b>(g: &'a ArrayView2<'b, char>) -> impl Iterator<Item = i64> + 'a + 'b {
    let mut i = 1;
    std::iter::from_fn(move || {
        if i == g.nrows() {
            None
        } else {
            for start in i..g.nrows() {
                let start = start as i64;
                let mut left = start - 1;
                let mut right = start;

                let mut mirror = true;

                loop {
                    if right == g.nrows() as i64 || left < 0 {
                        break;
                    }

                    let c1 = g.row(left as usize);
                    let c2 = g.row(right as usize);

                    if c1 != c2 {
                        mirror = false;
                        break;
                    }

                    left -= 1;
                    right += 1;
                }

                i += 1;
                if mirror {
                    return Some(start);
                }
            }
            None
        }
    })
}

fn part1(input: &Input) -> Result<i64> {
    let mut sum = 0;
    for grid in input {
        if let Some(mirror) = iter_mirrors(&grid.t()).next() {
            sum += mirror;
        } else {
            if let Some(mirror) = iter_mirrors(&grid.view()).next() {
                sum += 100 * mirror
            }
        }
    }

    Ok(sum)
}

fn part2(input: &Input) -> Result<i64> {
    let mut sum = 0;
    for grid in input {
        if let Some(mirror) = iter_mirrors(&grid.t()).next() {
            'outer: for i in 0..grid.nrows() {
                for j in 0..grid.ncols() {
                    let mut copy = grid.clone();
                    if grid[(i, j)] == '#' {
                        copy[(i, j)] = '.';
                    } else {
                        copy[(i, j)] = '#';
                    }

                    if let Some(mirror) = iter_mirrors(&copy.t()).filter(|&m| m != mirror).next() {
                        sum += mirror.clone();
                        break 'outer;
                    };

                    if let Some(mirror) = iter_mirrors(&copy.view()).next() {
                        sum += 100 * mirror.clone();
                        break 'outer;
                    };
                }
            }
        } else {
            if let Some(mirror) = iter_mirrors(&grid.view()).next() {
                'outer: for i in 0..grid.nrows() {
                    for j in 0..grid.ncols() {
                        let mut copy = grid.clone();
                        if grid[(i, j)] == '#' {
                            copy[(i, j)] = '.';
                        } else {
                            copy[(i, j)] = '#';
                        }

                        if let Some(mirror) =
                            iter_mirrors(&copy.view()).filter(|&m| m != mirror).next()
                        {
                            sum += 100 * mirror.clone();
                            break 'outer;
                        };

                        if let Some(mirror) = iter_mirrors(&copy.t()).next() {
                            sum += mirror.clone();
                            break 'outer;
                        };
                    }
                }
            }
        }
    }

    Ok(sum)
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
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#",
    );

    let input = parse_input(&input_str).unwrap();
    assert_eq!(part1(&input).unwrap(), 405);
    assert_eq!(part2(&input).unwrap(), 400);
}
