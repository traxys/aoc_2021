use std::ops::RangeInclusive;

use crate::{day, utils::split2, EyreResult};

day! {
    parser,
    part1 => "May y value: {}",
    part2 => "Number of probe launch configurations: {}"
}

#[derive(Debug)]
pub(crate) struct Target {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

type Parsed = Target;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let input = input.trim().trim_start_matches("target area: ");
    let (x, y) = split2(input, ", ").ok_or(color_eyre::eyre::eyre!("Malformed input"))?;

    fn parse_range(range: &str) -> EyreResult<RangeInclusive<i64>> {
        let (_, range) =
            split2(range, "=").ok_or(color_eyre::eyre::eyre!("Malformed range definition"))?;
        let (min, max) = split2(range, "..").ok_or(color_eyre::eyre::eyre!("Malformed range"))?;
        Ok(min.parse()?..=max.parse()?)
    }

    Ok(Target {
        x: parse_range(x)?,
        y: parse_range(y)?,
    })
}

fn yn(vy0: i64, n: i64) -> i64 {
    n * vy0 - (n * (n - 1) / 2)
}

fn xn(vx0: i64, n: i64) -> i64 {
    if n > vx0 {
        xmax(vx0)
    } else {
        yn(vx0, n)
    }
}

fn xmax(vx0: i64) -> i64 {
    vx0 * (vx0 + 1) / 2
}

fn vx0_for_xmax(xmax: i64) -> f64 {
    (-1. + (1. + 8. * xmax as f64).sqrt()) / 2.
}

fn sqrt_delta(v: f64, c: f64) -> f64 {
    ((1. + 2. * v).powi(2) - 8. * c).sqrt()
}

fn step_y(v0: f64, c: f64) -> f64 {
    ((1. + 2. * v0) + sqrt_delta(v0, c)) / 2.
}

fn step_x(v0: f64, c: f64) -> f64 {
    ((1. + 2. * v0) - sqrt_delta(v0, c)) / 2.
}

fn step_range_first_branch(v: f64, i: f64, o: f64) -> (f64, f64) {
    (step_x(v, i), step_x(v, o))
}

fn step_range_second_branch(v: f64, i: f64, o: f64) -> (f64, f64) {
    (step_y(v, i), step_y(v, o))
}

fn int_in_range((step_in, step_out): (f64, f64)) -> bool {
    if step_out.is_nan() {
        !step_in.is_nan()
    } else {
        //dbg!(step_in, step_out);
        step_out.floor() >= step_in.ceil()
    }
}

fn steps_y_into(v: f64, i: f64, o: f64) -> bool {
    let step_in = step_y(v as f64, i);
    let step_out = step_y(v as f64, o);
    step_out.floor() >= step_in.ceil()
}

#[cfg(test)]
mod test {
    use super::{xmax, xn, yn};

    #[test]
    fn without_drag_limit() {
        let vx0 = 7;
        let vy0 = 2;
        assert_eq!((xn(vx0, 0), yn(vy0, 0)), (0, 0));
        assert_eq!((xn(vx0, 1), yn(vy0, 1)), (7, 2));
        assert_eq!((xn(vx0, 2), yn(vy0, 2)), (13, 3));
        assert_eq!((xn(vx0, 3), yn(vy0, 3)), (18, 3));
        assert_eq!((xn(vx0, 4), yn(vy0, 4)), (22, 2));
        assert_eq!((xn(vx0, 5), yn(vy0, 5)), (25, 0));
        assert_eq!((xn(vx0, 6), yn(vy0, 6)), (27, -3));
        assert_eq!((xn(vx0, 7), yn(vy0, 7)), (28, -7));
    }

    #[test]
    fn with_drag_limit() {
        let vx0 = 6;
        let vy0 = 3;
        assert_eq!((xn(vx0, 0), yn(vy0, 0)), (0, 0));
        assert_eq!((xn(vx0, 1), yn(vy0, 1)), (6, 3));
        assert_eq!((xn(vx0, 2), yn(vy0, 2)), (11, 5));
        assert_eq!((xn(vx0, 3), yn(vy0, 3)), (15, 6));
        assert_eq!((xn(vx0, 4), yn(vy0, 4)), (18, 6));
        assert_eq!((xn(vx0, 5), yn(vy0, 5)), (20, 5));
        assert_eq!((xn(vx0, 6), yn(vy0, 6)), (21, 3));
        assert_eq!((xn(vx0, 7), yn(vy0, 7)), (21, 0));
        assert_eq!((xn(vx0, 8), yn(vy0, 8)), (21, -4));
        assert_eq!((xn(vx0, 9), yn(vy0, 9)), (21, -9));
        assert_eq!(xmax(vx0), 21)
    }
}

// 
// The main idea is that y(n) = n*vy0 - n(n-1)/2
// x(n) is the same uptil x(vx0), and constant after
// ymax/xmax = C(vC0)
//

pub(crate) fn part1(target: Parsed) -> EyreResult<i64> {
    Ok((0..1000)
        .filter(|&vy0| steps_y_into(vy0 as f64, *target.y.end() as f64, *target.y.start() as f64))
        // for n = vy0 we have the maximum y value
        .map(|vy0| yn(vy0, vy0))
        .max()
        .unwrap())
}

fn overlap_range(r1: (f64, f64), r2: (f64, f64)) -> Option<(f64, f64)> {
    let start = if r1.0.is_nan() {
        r2.0
    } else if r2.0.is_nan() {
        r1.0
    } else if r1.0 < r2.0 {
        r2.0
    } else {
        r1.0
    };

    let end = if r1.1.is_nan() {
        r2.1
    } else if r2.1.is_nan() {
        r1.1
    } else if r1.1 < r2.1 {
        r1.1
    } else {
        r2.1
    };

    if end < start {
        None
    } else {
        Some((start, end))
    }
}

pub(crate) fn part2(target: Parsed) -> EyreResult<usize> {
    let min_vx = vx0_for_xmax(*target.x.start()).ceil() as i64;
    let max_vx = *target.x.end();
    let count = (min_vx..=max_vx)
        .map(|vx| {
            (
                vx,
                step_range_first_branch(
                    vx as f64,
                    *target.x.start() as f64,
                    *target.x.end() as f64,
                ),
            )
        })
        .filter(|(_, r)| int_in_range(*r))
        .map(|(vx, xrange)| {
            // TODO: find a better bound on y ?
            (-1000..1000)
                .map(|vy| {
                    (
                        vy,
                        step_range_second_branch(
                            vy as f64,
                            *target.y.end() as f64,
                            *target.y.start() as f64,
                        ),
                    )
                })
                .filter(|(_, yrange)| int_in_range(*yrange))
                .filter(move |(_, yrange)| match overlap_range(xrange, *yrange) {
                    None => false,
                    Some(ov) => int_in_range(ov),
                })
                .map(move |(vy, _)| (vx, vy))
        })
        .flatten()
        .count();
    Ok(count)
}
