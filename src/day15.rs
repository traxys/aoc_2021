use std::collections::{BinaryHeap, HashMap};

use crate::{day, EyreResult};
use nalgebra::DMatrix;

day! {
    parser,
    part1 => "Total risk is {}",
    part2 => "Total risk is {}",
}

type Parsed = DMatrix<u8>;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let cols = input.lines().count();
    let lines = input
        .find('\n')
        .ok_or(color_eyre::eyre::eyre!("No lines in input"))?;

    let table = DMatrix::from_iterator(
        lines,
        cols,
        input
            .lines()
            .map(|b| b.trim().as_bytes().iter().map(|b| b - b'0'))
            .flatten(),
    );

    Ok(table)
}

fn neighbours<F>(
    i: isize,
    j: isize,
    rows: isize,
    cols: isize,
    cost_eval: &F,
) -> impl Iterator<Item = ((usize, usize), u8)> + '_
where
    F: Fn((usize, usize)) -> u8,
{
    let mut vals = [(-1, -1); 4];

    if i > 0 {
        vals[0] = (i - 1, j);
    }
    if i < rows - 1 {
        vals[1] = (i + 1, j);
    }

    if j > 0 {
        vals[2] = (i, j - 1);
    }
    if j < cols - 1 {
        vals[3] = (i, j + 1);
    }

    vals.into_iter()
        .filter(|&p| p != (-1, -1))
        .map(|(i, j)| (i as usize, j as usize))
        .map(|p| (p, cost_eval(p)))
}

fn search_cost<F>(rows: usize, cols: usize, cost_eval: F) -> u64
where
    F: Fn((usize, usize)) -> u8,
{
    #[derive(PartialEq, Eq)]
    struct Path {
        node: (usize, usize),
        cost: u64,
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let target = (rows - 1, cols - 1);

    let mut access = HashMap::new();
    let mut paths = BinaryHeap::new();
    paths.push(Path {
        node: (0, 0),
        cost: 0,
    });

    loop {
        let path = paths.pop().unwrap();

        if path.node == target {
            return path.cost;
        }

        if let Some(&acc) = access.get(&path.node) {
            if acc < path.cost {
                continue;
            }
        }

        for (p, cost) in neighbours(
            path.node.0 as isize,
            path.node.1 as isize,
            rows as isize,
            cols as isize,
            &cost_eval,
        ) {
            let path_cost = cost as u64 + path.cost;
            let keep = match access.get_mut(&p) {
                None => {
                    access.insert(p, path_cost);
                    true
                }
                Some(oldcost) => {
                    if *oldcost > path_cost {
                        *oldcost = path_cost;
                        true
                    } else {
                        false
                    }
                }
            };
            if keep {
                paths.push(Path {
                    node: p,
                    cost: path_cost,
                })
            }
        }
    }
}

pub(crate) fn part1(matrix: Parsed) -> EyreResult<u64> {
    Ok(search_cost(matrix.nrows(), matrix.ncols(), |p| matrix[p]))
}

pub(crate) fn part2(matrix: Parsed) -> EyreResult<u64> {
    let mut total_matrix = DMatrix::zeros(matrix.nrows() * 5, matrix.ncols() * 5);

    for j in 0..matrix.ncols() {
        for i in 0..matrix.nrows() {
            total_matrix[(i, j)] = matrix[(i, j)];
        }
    }

    for j in matrix.ncols()..total_matrix.ncols() {
        for i in 0..total_matrix.nrows() {
            let tmp = total_matrix[(i, j - matrix.ncols())] + 1;
            total_matrix[(i, j)] = if tmp > 9 { 1 } else { tmp };
        }
    }

    for j in 0..total_matrix.ncols() {
        for i in matrix.nrows()..total_matrix.nrows() {
            let tmp = total_matrix[(i - matrix.nrows(), j)] + 1;
            total_matrix[(i, j)] = if tmp > 9 { 1 } else { tmp };
        }
    }

    Ok(search_cost(
        total_matrix.nrows(),
        total_matrix.ncols(),
        |p| total_matrix[p],
    ))
}
