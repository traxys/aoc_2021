use arrayvec::ArrayVec;

use crate::{day, utils::split2, EyreResult};
use std::{cmp, collections::HashSet};

day! {
    parser,
    part1 => "There are {} pixels",
    part2 => "There are {} pixels after 50 rounds",
}

type Parsed = ([bool; 512], State);

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let (mapping, pattern) =
        split2(input, "\n\n").ok_or(color_eyre::eyre::eyre!("No empty line"))?;
    let mapping: ArrayVec<bool, 512> = mapping.trim().bytes().map(|b| b == b'#').collect();
    let pattern = pattern
        .lines()
        .enumerate()
        .map(|(l, line)| {
            line.trim()
                .bytes()
                .enumerate()
                .filter(|(_, i)| *i == b'#')
                .map(move |(c, _)| (c as i64, l as i64))
        })
        .flatten()
        .collect();

    Ok((
        mapping
            .into_inner()
            .map_err(|_| color_eyre::eyre::eyre!("Mapping is not 512 characters"))?,
        State::new(pattern),
    ))
}

fn reduce_bool<I>(iter: I) -> u16
where
    I: Iterator<Item = bool>,
{
    iter.fold(0, |curr, b| (curr << 1) | b as u16)
}

fn neighbours(x: i64, y: i64) -> [(i64, i64); 9] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

pub(crate) struct State {
    inverted: bool,
    coords: HashSet<(i64, i64)>,
}

impl State {
    fn new(coords: HashSet<(i64, i64)>) -> Self {
        Self {
            inverted: false,
            coords,
        }
    }

    fn step(&self, mapping: &[bool; 512]) -> Self {
        let bounding = bounding_box(&self.coords);

        let invert_mapping = if !self.inverted {
            mapping[0]
        } else {
            mapping[512 - 1]
        };

        let mut new_coords = HashSet::new();

        for x in (bounding.min_x - 1)..=(bounding.max_x + 1) {
            for y in (bounding.min_y - 1)..=(bounding.max_y + 1) {
                let num = reduce_bool(
                    neighbours(x, y)
                        .iter()
                        .map(|c| self.coords.contains(&c) ^ self.inverted),
                );
                if mapping[num as usize] ^ invert_mapping {
                    new_coords.insert((x, y));
                }
            }
        }

        Self {
            inverted: invert_mapping,
            coords: new_coords,
        }
    }

    /* fn print(&self) {
        let b = self.bounding();
        let (contained, not_contained) = if self.inverted {
            ('.', '#')
        } else {
            ('#', '.')
        };

        for y in (b.min_y - 2)..=(b.max_y + 2) {
            for x in (b.min_x - 2)..=(b.max_x + 2) {
                if self.coords.contains(&(x, y)) {
                    print!("{}", contained)
                } else {
                    print!("{}", not_contained)
                }
            }
            println!()
        }
    } */

    fn amount_lit(&self) -> Option<usize> {
        if self.inverted {
            None
        } else {
            Some(self.coords.len())
        }
    }
}

#[derive(Debug)]
struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

fn bounding_box(input: &HashSet<(i64, i64)>) -> BoundingBox {
    input.iter().fold(
        BoundingBox {
            min_x: i64::MAX,
            max_x: i64::MIN,
            min_y: i64::MAX,
            max_y: i64::MIN,
        },
        |bounding, &(x, y)| BoundingBox {
            min_x: cmp::min(x, bounding.min_x),
            max_x: cmp::max(x, bounding.max_x),
            min_y: cmp::min(y, bounding.min_y),
            max_y: cmp::max(y, bounding.max_y),
        },
    )
}

pub(crate) fn part1((mapping, state): Parsed) -> EyreResult<usize> {
    // state.print();

    let state = state.step(&mapping);
    // state.print();

    let state = state.step(&mapping);
    // state.print();

    Ok(state.amount_lit().unwrap())
}

pub(crate) fn part2((mapping, mut state): Parsed) -> EyreResult<usize> {
    for _ in 0..50 {
        state = state.step(&mapping);
    }

    Ok(state.amount_lit().unwrap())
}
