use crate::EyreResult;

type Parsed = Vec<u64>;

fn median(input: &mut [u64]) -> u64 {
    input.sort();
    if input.len() % 2 == 0 {
        (input[input.len() / 2] + input[(input.len() / 2) + 1]) / 2
    } else {
        input[(input.len() + 1) / 2]
    }
}

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input
        .split(",")
        .map(|x| x.trim().parse().map_err(Into::into))
        .collect()
}

pub(crate) fn part1(mut input: Parsed) -> EyreResult<u64> {
    let min_fuel = median(&mut input) as i64;
    Ok(input
        .iter()
        .map(|&v| (v as i64 - min_fuel).abs() as u64)
        .sum())
}

fn fuel(x: i64, xi: i64) -> i64 {
    let a = (xi - x).abs();
    (a + 1) * (a) / 2
}

fn total_fuel(x: i64, pos: &[i64]) -> i64 {
    pos.iter().map(|&xi| fuel(x, xi)).sum()
}

pub(crate) fn part2(input: Parsed) -> EyreResult<u64> {
    let inp: Vec<_> = input.iter().map(|&v| v as i64).collect();

    let (_, fuelmin) = (0..*inp
        .iter()
        .max()
        .ok_or(color_eyre::eyre::eyre!("Input is empty"))?)
        .map(|x| (x, total_fuel(x, &inp)))
        .min_by(|(_, v), (_, v2)| v.cmp(v2))
        .ok_or(color_eyre::eyre::eyre!("Range was empty"))?;

    Ok(fuelmin as u64)
}

pub(crate) fn fmt1(output: u64) -> String {
    format!("Minimum fuel is {}", output)
}

pub(crate) fn fmt2(output: u64) -> String {
    format!("Minimum fuel is {}", output)
}
