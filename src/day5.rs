use crate::{day, utils::split2, EyreResult};
use num::integer::gcd;
use std::collections::HashSet;

day! {
    parser,
    part1 => "Overlapping point count: {}",
    part2 => "Overlapping point count: {}"
}

pub(crate) struct Line {
    start: (i64, i64),
    end: (i64, i64),
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn points(&self) -> impl Iterator<Item = (i64, i64)> {
        let mut director = (self.end.0 - self.start.0, self.end.1 - self.start.1);
        let div = gcd(director.0, director.1);
        director.0 /= div;
        director.1 /= div;
        LineIterator {
            current: self.start,
            end: self.end,
            director,
            done: false,
        }
    }
}

#[derive(Debug)]
struct LineIterator {
   current: (i64, i64),
    end: (i64, i64),
    director: (i64, i64),
    done: bool,
}

impl Iterator for LineIterator {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            if self.current == self.end {
                self.done = true;
            }

            let current = self.current;
            self.current.0 += self.director.0;
            self.current.1 += self.director.1;

            Some(current)
        }
    }
}

fn parse_tuple(s: &str) -> EyreResult<(i64, i64)> {
    let (x, y) = split2(s, ",").ok_or(color_eyre::eyre::eyre!("Invalid coords: {}", s))?;
    Ok((x.parse()?, y.parse()?))
}

type Parsed = Vec<Line>;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input
        .lines()
        .map(|line| -> EyreResult<_> {
            let (start, end) = split2(line, " -> ")
                .ok_or(color_eyre::eyre::eyre!("Line format is invalid: {}", line))?;
            Ok(Line {
                start: parse_tuple(start)?,
                end: parse_tuple(end)?,
            })
        })
        .collect()
}

pub(crate) fn part1(lines: Parsed) -> EyreResult<usize> {
    let mut present = HashSet::new();
    let mut overlap = HashSet::new();
    lines
        .iter()
        .filter(|line| line.is_straight())
        .for_each(|line| {
            line.points()
                .filter(|&point| !present.insert(point))
                .for_each(|point| {
                    overlap.insert(point);
                })
        });
    Ok(overlap.len())
}

pub(crate) fn part2(lines: Parsed) -> EyreResult<usize> {
    let mut present = HashSet::new();
    let mut overlap = HashSet::new();
    lines.iter().for_each(|line| {
        line.points()
            .filter(|&point| !present.insert(point))
            .for_each(|point| {
                overlap.insert(point);
            })
    });
    Ok(overlap.len())
}
