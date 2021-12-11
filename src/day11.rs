use crate::{day, EyreResult};
use arrayvec::ArrayVec;

day! {
    parser,
    part1 => "There where {} flashes",
    part2 => "Synchronizes after {} steps",
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Octopus {
    energy: u8,
    flashed: bool,
}

type Parsed = [[Octopus; 10]; 10];

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| Octopus {
                    energy: b - b'0',
                    flashed: false,
                })
                .collect::<ArrayVec<_, 10>>()
                .into_inner()
                .map_err(|_| color_eyre::eyre::eyre!("Line is not of len 10"))
        })
        .collect::<EyreResult<ArrayVec<_, 10>>>()?
        .into_inner()
        .map_err(|_| color_eyre::eyre::eyre!("There are not 10 lines"))
}

impl Octopus {
    fn should_flash(&self) -> bool {
        self.energy > 9 && !self.flashed
    }
}

fn step(input: &mut Parsed) -> usize {
    input
        .iter_mut()
        .for_each(|l| l.iter_mut().for_each(|o| o.energy += 1));

    let mut flashes = 0;
    loop {
        let flashes_save = flashes;

        for i in 0..10 {
            for j in 0..10 {
                if input[i][j].should_flash() {
                    flashes += 1;
                    input[i][j].flashed = true;

                    let offsets = (-1..=1)
                        .map(|ioffset| (-1..=1).map(move |joffset| (ioffset, joffset)))
                        .flatten()
                        .filter(|&(ioff, joff)| !(ioff == 0 && joff == 0));

                    for (ioff, joff) in offsets {
                        let i = i as i32 + ioff;
                        let j = j as i32 + joff;

                        if i >= 0 && i < 10 && j >= 0 && j < 10 {
                            input[i as usize][j as usize].energy += 1;
                        }
                    }
                }
            }
        }

        if flashes_save == flashes {
            break;
        }
    }

    input.iter_mut().for_each(|l| {
        l.iter_mut().for_each(|o| {
            if o.flashed {
                o.flashed = false;
                o.energy = 0;
            }
        })
    });

    flashes
}

pub(crate) fn part1(mut input: Parsed) -> EyreResult<usize> {
    Ok((0..100).map(|_| step(&mut input)).sum())
}

pub(crate) fn part2(mut input: Parsed) -> EyreResult<usize> {
    let mut i = 0;
    loop {
        i += 1;
        if step(&mut input) == 10 * 10 {
            break Ok(i);
        }
    }
}
