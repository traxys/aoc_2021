use crate::EyreResult;

type Parsed = Vec<Vec<i8>>;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .bytes()
                .map(|v| match v {
                    b'(' => Ok(1),
                    b')' => Ok(-1),
                    b'[' => Ok(2),
                    b']' => Ok(-2),
                    b'{' => Ok(3),
                    b'}' => Ok(-3),
                    b'<' => Ok(4),
                    b'>' => Ok(-4),
                    _ => {
                        color_eyre::eyre::bail!("Invalid character: {:?}", char::from_u32(v.into()))
                    }
                })
                .collect()
        })
        .collect()
}

fn is_corrupted(line: &[i8], stack: &mut Vec<i8>) -> Option<i8> {
    stack.clear();
    for &c in line {
        if c > 0 {
            stack.push(c);
        } else {
            let poped = stack.pop().unwrap();
            if poped != -c {
                return Some(-c);
            }
        }
    }
    None
}

pub(crate) fn part1(input: Parsed) -> EyreResult<usize> {
    let mut stack = Vec::new();
    let mut syntax_error = 0;

    for line in input {
        if let Some(c) = is_corrupted(&line, &mut stack) {
            let score = match c {
                1 => 3,
                2 => 57,
                3 => 1197,
                4 => 25137,
                _ => unreachable!(),
            };
            syntax_error += score;
        }
    }

    Ok(syntax_error)
}

pub(crate) fn part2(input: Parsed) -> EyreResult<u64> {
    let mut stack = Vec::new();
    let mut stack2 = Vec::new();
    let mut scores: Vec<_> = input
        .iter()
        .filter(|line| is_corrupted(line, &mut stack).is_none())
        .map(|line| {
            stack2.clear();
            for &c in line {
                if c > 0 {
                    stack2.push(c);
                } else {
                    stack2.pop();
                }
            }
            stack2
                .iter()
                .rev()
                .fold(0, |score, &missing| score * 5 + (missing as u64))
        })
        .collect();
    scores.sort();
    Ok(scores[scores.len() / 2])
}

pub(crate) fn fmt1(output: usize) -> String {
    format!("Syntax error score is {}", output)
}

pub(crate) fn fmt2(output: u64) -> String {
    format!("Middle auto complete is {}", output)
}
