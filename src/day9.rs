use crate::{EyreResult, day};
use nalgebra::DMatrix;
use std::collections::HashSet;

day! {
    parser,
    part1 => "Risk level is {}",
    part2 => "Multiplication of size is {}",
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
            .as_bytes()
            .iter()
            .filter(|&&b| b != b'\n')
            .map(|b| b - b'0'),
    );

    Ok(table)
}

fn flows_into(matrix: &Parsed, i: usize, j: usize) -> Option<(usize, usize)> {
    let mut flow = None;
    let mut n = 10;

    if i + 1 < matrix.nrows() {
        n = matrix[(i + 1, j)];
        flow = Some((i + 1, j));
    }
    if i > 0 {
        let p = (i - 1, j);
        if matrix[p] < n {
            n = matrix[p];
            flow = Some(p);
        }
    }
    if j + 1 < matrix.ncols() {
        let p = (i, j + 1);
        if matrix[p] < n {
            n = matrix[p];
            flow = Some(p);
        }
    }
    if j > 0 {
        let p = (i, j - 1);
        if matrix[p] < n {
            n = matrix[p];
            flow = Some(p);
        }
    }

    if matrix[(i, j)] < n {
        None
    } else {
        flow
    }
}

fn low_points(matrix: &Parsed) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    (0..matrix.ncols())
        .map(move |j| {
            (0..matrix.nrows()).filter_map(move |i| match flows_into(matrix, i, j) {
                None => Some(((i, j), matrix[(i, j)])),
                Some(_) => None,
            })
        })
        .flatten()
}

pub(crate) fn part1(matrix: Parsed) -> EyreResult<usize> {
    Ok(low_points(&matrix).map(|(_, p)| 1 + p as usize).sum())
}

pub(crate) fn part2(matrix: Parsed) -> EyreResult<usize> {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct Basin {
        parent: (usize, usize),
        size: usize,
    }

    fn find(basins: &mut DMatrix<Basin>, mut x: (usize, usize)) -> (usize, usize) {
        while x != basins[x].parent {
            let tmp = basins[x].parent;
            basins[x].parent = basins[basins[x].parent].parent;
            x = tmp;
        }
        x
    }

    fn union(
        basins: &mut DMatrix<Basin>,
        roots: &mut HashSet<(usize, usize)>,
        x: (usize, usize),
        y: (usize, usize),
    ) {
        let mut x = find(basins, x);
        let mut y = find(basins, y);

        if x == y {
            return;
        }

        if basins[x].size < basins[y].size {
            std::mem::swap(&mut x, &mut y);
        }
        roots.remove(&y);

        basins[y].parent = x;
        basins[x].size += basins[y].size;
    }

    let mut basins: DMatrix<_> = DMatrix::from_fn(matrix.nrows(), matrix.ncols(), |i, j| Basin {
        parent: (i, j),
        size: 1,
    });

    let m = &matrix;
    let mut roots: HashSet<_> = (0..matrix.ncols())
        .map(|j| {
            (0..m.nrows())
                .filter(move |&i| m[(i, j)] != 9)
                .map(move |i| (i, j))
        })
        .flatten()
        .collect();

    for j in 0..matrix.ncols() {
        for i in 0..matrix.nrows() {
            if let Some(flow) = flows_into(&matrix, i, j) {
                if matrix[(i, j)] != 9 {
                    union(&mut basins, &mut roots, (i, j), flow);
                }
            }
        }
    }

    /*
    for (basin, size) in basins_list.iter() {
        println!("Basin {:?} (size = {})", basin, size);
        for j in 0..matrix.ncols() {
            print!(" ");
            for i in 0..matrix.nrows() {
                if find(&mut basins, (i, j)) == *basin {
                    print!(
                        "{}",
                        ansi_term::Colour::Red
                            .underline()
                            .bold()
                            .paint(format!("{}", matrix[(i, j)]))
                    )
                } else {
                    print!("{}", matrix[(i, j)]);
                }
            }
            println!();
        }
    } */

    let mut basins: Vec<_> = roots.iter().map(|&k| basins[k].size).collect();
    basins.sort_by(|a, b| b.cmp(a));

    Ok(basins[0] * basins[1] * basins[2])
}
