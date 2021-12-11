use crate::{day, EyreResult};
use arrayvec::ArrayVec;

day! {
    parser,
    part1 => "Score of the first winning board is {}",
    part2 => "Score of the last winning board is {}"
}

#[derive(Debug)]
pub(crate) struct Board {
    array: [[u64; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn is_complete(&self) -> bool {
        self.marked.iter().any(|line| line.iter().all(|&v| v))
            || (0..5).any(|idx| self.marked.iter().all(|line| line[idx]))
    }

    fn draw(&mut self, num: u64) {
        for (num_line, marked_line) in self.array.iter().zip(self.marked.iter_mut()) {
            for (&n, marked) in num_line.iter().zip(marked_line.iter_mut()) {
                if n == num {
                    *marked = true;
                    return;
                }
            }
        }
    }

    fn score(&self) -> u64 {
        self.array
            .iter()
            .zip(self.marked)
            .map(|(line, marked_line)| {
                line.iter()
                    .zip(marked_line)
                    .filter_map(|(&n, marked)| if marked { None } else { Some(n) })
                    .sum::<u64>()
            })
            .sum()
    }
}

type Parsed = (Vec<u64>, Vec<Board>);

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let mut input = input.split("\n\n");
    let draw = input
        .next()
        .ok_or(color_eyre::eyre::eyre!("First line is missing"))?
        .split(",")
        .map(|n| n.parse().map_err(Into::into))
        .collect::<EyreResult<_>>()?;

    let boards = input
        .map(|board| -> EyreResult<Board> {
            Ok(Board {
                array: board
                    .lines()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|n| n.parse().map_err(Into::into))
                            .collect::<EyreResult<ArrayVec<u64, 5>>>()?
                            .into_inner()
                            .map_err(|_| color_eyre::eyre::eyre!("Lines where not 5x5"))
                    })
                    .collect::<EyreResult<ArrayVec<[u64; 5], 5>>>()?
                    .into_inner()
                    .map_err(|_| color_eyre::eyre::eyre!("Board does not have 5 rows"))?,
                marked: [[false; 5]; 5],
            })
        })
        .collect::<EyreResult<_>>()?;

    Ok((draw, boards))
}

pub(crate) fn part1((draws, mut boards): Parsed) -> EyreResult<u64> {
    for draw in draws {
        for board in &mut boards {
            board.draw(draw);
            if board.is_complete() {
                return Ok(board.score() * draw);
            }
        }
    }
    unreachable!("no board wins")
}

pub(crate) fn part2((draws, mut boards): Parsed) -> EyreResult<u64> {
    for draw in draws {
        let mut remaining_win = boards.len();
        for board in &mut boards {
            board.draw(draw);
            if board.is_complete() {
                remaining_win -= 1;
                if remaining_win == 0 {
                    return Ok(board.score() * draw);
                }
            }
        }
        boards.retain(|b| !b.is_complete())
    }
    unreachable!("no board wins")
}
