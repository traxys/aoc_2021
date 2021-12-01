use crate::EyreResult;

pub(crate) fn parser(input: &str) -> EyreResult<Vec<u64>> {
    input
        .lines()
        .map(|l| l.trim().parse().map_err(Into::into))
        .collect()
}

pub(crate) fn part1(depths: Vec<u64>) -> EyreResult<usize> {
    Ok(depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(f, s)| f < s)
        .count())
}

pub(crate) fn part2(depths: Vec<u64>) -> EyreResult<usize> {
    let sums = || {
        depths
            .iter()
            .zip(depths.iter().skip(1))
            .zip(depths.iter().skip(2))
            .map(|((a, b), c)| a + b + c)
    };
    Ok(sums().zip(sums().skip(1)).filter(|(f, s)| f < s).count())
}

pub(crate) fn fmt1(output: usize) -> String {
    format!("The depth increased {} times", output)
}

pub(crate) fn fmt2(output: usize) -> String {
    format!("The depth increased {} times with windows", output)
}
