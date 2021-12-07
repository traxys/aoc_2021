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

fn fuel(x: u64, xi: u64) -> u64 {
    let a = (xi as i64 - x as i64).abs() as u64;
    (a + 1) * (a) / 2
}

fn total_fuel(x: u64, pos: &[u64]) -> u64 {
    pos.iter().map(|&xi| fuel(x, xi)).sum()
}

pub(crate) fn part2(input: Parsed) -> EyreResult<u64> {
    let average = (input.iter().sum::<u64>() as f64 / input.len() as f64).round() as u64;
    // We can prove that avg(x) - 0.5 < distance < avg(x) + 0.5
    // We just have to try 2 values in order to find the correct distance, but use 3 just to be
    // safe
    Ok((-1..=1)
            .map(|off| total_fuel((average as i64 + off) as u64, &input))
            .min()
            .unwrap())
}

pub(crate) fn fmt1(output: u64) -> String {
    format!("Minimum fuel is {}", output)
}

pub(crate) fn fmt2(output: u64) -> String {
    format!("Minimum fuel is {}", output)
}
