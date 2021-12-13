use std::collections::HashSet;

use color_eyre::eyre;

use crate::{day, utils::split2, EyreResult};

day! {
    parser,
    part1 => "After one fold there are {} points",
    part2 => "Pattern: \n{}",
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Axis {
    X,
    Y,
}

type Parsed = (HashSet<(u64, u64)>, Vec<(Axis, u64)>);

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let (dots, folds) = split2(input, "\n\n").ok_or(eyre::eyre!("No empty line"))?;
    let dots = dots
        .lines()
        .map(|line| -> EyreResult<_> {
            let (x, y) = split2(line, ",").ok_or(eyre::eyre!("Invalid point: missing ','"))?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect::<EyreResult<_>>()?;

    let folds = folds
        .lines()
        .map(|line| -> EyreResult<_> {
            let p = line
                .strip_prefix("fold along ")
                .ok_or(eyre::eyre!("Lines does not start with 'fold along '"))?;
            let (axis, dist) = split2(p, "=").ok_or(eyre::eyre!("No equal in line"))?;

            let axis = match axis {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => eyre::bail!("Invalid axis"),
            };

            Ok((axis, dist.parse()?))
        })
        .collect::<EyreResult<_>>()?;

    Ok((dots, folds))
}

pub(crate) fn part1((mut points, fold): Parsed) -> EyreResult<usize> {
    let (axis, idx) = fold[0];
    match axis {
        Axis::X => {
            points = points
                .drain()
                .map(|(x, y)| if x > idx { (2 * idx - x, y) } else { (x, y) })
                .collect();
        }
        Axis::Y => {
            points = points
                .drain()
                .map(|(x, y)| if y > idx { (x, 2 * idx - y) } else { (x, y) })
                .collect();
        }
    }
    Ok(points.len())
}

pub(crate) fn part2((mut points, fold): Parsed) -> EyreResult<String> {
    for (axis, idx) in fold {
        match axis {
            Axis::X => {
                points = points
                    .drain()
                    .map(|(x, y)| {
                        if x < idx {
                            (idx - x - 1, y)
                        } else {
                            (x - idx - 1, y)
                        }
                    })
                    .collect();
            }
            Axis::Y => {
                points = points
                    .drain()
                    .map(|(x, y)| if y > idx { (x, 2 * idx - y) } else { (x, y) })
                    .collect();
            }
        }
    }
    let &max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let &max_y = points.iter().map(|(_, y)| y).max().unwrap();

    let mut s = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(max_x - x, y)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }

    Ok(s)
}
