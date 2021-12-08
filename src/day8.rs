use crate::{utils::split2, EyreResult};
use arrayvec::ArrayVec;
use color_eyre::eyre;

type Parsed = Vec<([ArrayVec<u8, 7>; 10], [ArrayVec<u8, 7>; 4])>;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    fn parse_segments<const N: usize>(segments: &str) -> EyreResult<[ArrayVec<u8, 7>; N]> {
        segments
            .split_whitespace()
            .map(|segment| segment.as_bytes().iter().map(|c| c - b'a').collect())
            .collect::<ArrayVec<_, N>>()
            .into_inner()
            .map_err(|s| eyre::eyre!("Not enough segments: got {} needed {}", s.len(), N))
    }

    input
        .lines()
        .map(|line| {
            let (input, output) =
                split2(line, " | ").ok_or(eyre::eyre!("no | in line: {:?}", line))?;
            Ok((parse_segments(input)?, parse_segments(output)?))
        })
        .collect()
}

pub(crate) fn part1(logs: Parsed) -> EyreResult<usize> {
    Ok(logs
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|seg| match seg.len() {
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum())
}

fn only_in_one(a: &ArrayVec<u8, 7>, b: &ArrayVec<u8, 7>) -> impl Iterator<Item = u8> {
    let mut count = [0; 7];
    for x in a.iter().chain(b) {
        count[*x as usize] += 1;
    }

    (0..7).filter(move |&idx| count[idx as usize] == 1)
}

fn is_included_in(large: &ArrayVec<u8, 7>, small: &ArrayVec<u8, 7>) -> bool {
    small.iter().all(|i| large.contains(i))
}

fn single_difference(a: &ArrayVec<u8, 7>, b: &ArrayVec<u8, 7>) -> EyreResult<u8> {
    single_iter(only_in_one(a, b))
}

fn single_iter<T, I>(mut it: I) -> EyreResult<T>
where
    I: Iterator<Item = T>,
{
    let res = it.next().ok_or(eyre::eyre!("Iterator is empty"))?;
    if it.next().is_some() {
        eyre::bail!("More than one element in iterator")
    } else {
        Ok(res)
    }
}

fn pick_single<F: Fn(&ArrayVec<u8, 7>) -> bool>(
    inputs: &mut Vec<ArrayVec<u8, 7>>,
    select: F,
) -> EyreResult<ArrayVec<u8, 7>> {
    let (idx, _) = single_iter(inputs.iter().enumerate().filter(|(_, v)| select(v)))?;
    let mut res = inputs.remove(idx);
    res.sort();
    Ok(res)
}

// `n`'s pattern is return[n]
fn recover_mapping(inputs: &[ArrayVec<u8, 7>; 10]) -> EyreResult<[ArrayVec<u8, 7>; 10]> {
    let mut numbers: Vec<_> = inputs.clone().into_iter().collect();

    let one = pick_single(&mut numbers, |s| s.len() == 2)?;
    let seven = pick_single(&mut numbers, |s| s.len() == 3)?;
    let three = pick_single(&mut numbers, |s| s.len() == 5 && is_included_in(s, &one))?;
    let nine = pick_single(&mut numbers, |s| s.len() == 6 && is_included_in(s, &three))?;
    let eight = pick_single(&mut numbers, |s| s.len() == 7)?;

    let e = single_difference(&nine, &eight)?;

    let five = pick_single(&mut numbers, |s| s.len() == 5 && !s.contains(&e))?;
    let six = pick_single(&mut numbers, |s| s.len() == 6 && is_included_in(s, &five))?;
    let zero = pick_single(&mut numbers, |s| s.len() == 6)?;
    let two = pick_single(&mut numbers, |s| s.len() == 5)?;
    let four = pick_single(&mut numbers, |_| true)?;

    Ok([zero, one, two, three, four, five, six, seven, eight, nine])
}

fn translate_output(reversed: &[ArrayVec<u8, 7>], output: &mut [ArrayVec<u8, 7>]) -> usize {
    output
        .iter_mut()
        .map(|segment| {
            segment.sort();
            reversed
                .iter()
                .enumerate()
                .find(|(_, s)| s == &segment)
                .unwrap()
                .0
        })
        .fold(0, |x, n| 10 * x + n)
}

pub(crate) fn part2(mut logs: Parsed) -> EyreResult<usize> {
    logs.iter_mut()
        .map(|(input, ref mut output)| -> EyreResult<_> {
            let reversed = recover_mapping(&input)?;
            Ok(translate_output(&reversed, output))
        })
        .try_fold(0, |s, n| Ok(s + n?))
}

pub(crate) fn fmt1(output: usize) -> String {
    format!("There are {} 1,4,7 and 8", output)
}

pub(crate) fn fmt2(output: usize) -> String {
    format!("Sum of displays is {}", output)
}

#[cfg(test)]
mod test {
    use super::{parser, recover_mapping, translate_output};

    #[test]
    fn recover_example_mapping() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let (input, mut output) = parser(input).unwrap().into_iter().next().unwrap();

        let reversed_mapping = recover_mapping(&input).unwrap();

        let output = translate_output(&reversed_mapping, &mut output);

        assert_eq!(output, 5353)
    }
}
