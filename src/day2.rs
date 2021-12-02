use crate::EyreResult;
use color_eyre::eyre;
use std::str::FromStr;

pub(crate) enum Action {
    Forward,
    Up,
    Down,
}

impl FromStr for Action {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(eyre::eyre!("No such action: {}", s)),
        }
    }
}

pub(crate) fn parser(input: &str) -> EyreResult<Vec<(Action, u32)>> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            Ok((
                parts.next().ok_or(eyre::eyre!("No action"))?.parse()?,
                parts.next().ok_or(eyre::eyre!("No distance"))?.parse()?,
            ))
        })
        .collect()
}

pub(crate) struct State {
    horizontal: u64,
    vertical: i64,
}

pub(crate) fn part1(instructions: Vec<(Action, u32)>) -> EyreResult<State> {
    Ok(instructions.iter().fold(
        State {
            horizontal: 0,
            vertical: 0,
        },
        |mut state, (action, distance)| {
            match action {
                Action::Forward => state.horizontal += *distance as u64,
                Action::Up => state.vertical -= *distance as i64,
                Action::Down => state.vertical += *distance as i64,
            }
            state
        },
    ))
}

pub(crate) struct AimState {
    horizontal: u64,
    vertical: i64,
    aim: i64,
}

pub(crate) fn part2(instructions: Vec<(Action, u32)>) -> EyreResult<AimState> {
    Ok(instructions.iter().fold(
        AimState {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        },
        |mut state, (action, distance)| {
            match action {
                Action::Forward => {
                    state.horizontal += *distance as u64;
                    state.vertical += *distance as i64 * state.aim;
                }
                Action::Up => state.aim -= *distance as i64,
                Action::Down => state.aim += *distance as i64,
            }
            state
        },
    ))
}

pub(crate) fn fmt1(output: State) -> String {
    format!(
        "horizontal = {}, vertical = {}, result = {}",
        output.horizontal,
        output.vertical,
        output.horizontal as i64 * output.vertical
    )
}

pub(crate) fn fmt2(output: AimState) -> String {
    format!("Result is {}", output.horizontal as i64 * output.vertical)
}
