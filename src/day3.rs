use crate::{day, EyreResult};

day! {
    parser,
    part1 => "{}",
    part2 => "{}",
}

pub(crate) fn parser(input: &str) -> EyreResult<(usize, Vec<u16>)> {
    let len = input
        .find('\n')
        .ok_or(color_eyre::eyre::eyre!("number list is empty"))?;
    Ok((
        len,
        input
            .lines()
            .map(|v| u16::from_str_radix(v.trim(), 2).map_err(Into::into))
            .collect::<EyreResult<_>>()?,
    ))
}

pub(crate) fn part1((num_len, numbers): (usize, Vec<u16>)) -> EyreResult<String> {
    let (len, occurences) =
        numbers
            .iter()
            .fold((0, vec![0; num_len]), |(count, mut occurences), num| {
                for b in 0..num_len {
                    occurences[b] += ((num & (1 << b)) != 0) as usize;
                }
                (count + 1, occurences)
            });
    let mut gamma = 0;
    for (pos, &bit) in occurences.iter().enumerate() {
        gamma += ((bit > (len / 2)) as u16) << pos;
    }
    let epsilon = !gamma & !(!0 << num_len);
    Ok(format!(
        "Gamma is {:b} and epsilon is {:b}. Power consumption is {}",
        gamma,
        epsilon,
        gamma as u64 * epsilon as u64
    ))
}

fn bit_criteria_filter(mut numbers: Vec<u16>, num_len: usize, most: bool) -> u16 {
    let mut position = num_len - 1;
    while numbers.len() > 1 {
        let ones = numbers.iter().fold(0, |occurence, number| {
            occurence + ((number & (1 << position)) != 0) as usize
        });

        let retain_ones = if numbers.len() / 2 == ones && numbers.len() % 2 == 0 {
            most
        } else {
            if most {
                ones > numbers.len() / 2
            } else {
                ones < numbers.len() / 2
            }
        };

        if retain_ones {
            numbers.retain(|num| num & (1 << position) != 0)
        } else {
            numbers.retain(|num| num & (1 << position) == 0)
        }

        if position == 0 {
            break;
        }
        position -= 1;
    }

    assert!(numbers.len() == 1);

    numbers[0]
}

pub(crate) fn part2((num_len, numbers): (usize, Vec<u16>)) -> EyreResult<String> {
    let oxygen_rating = bit_criteria_filter(numbers.clone(), num_len, true);
    let co2_rating = bit_criteria_filter(numbers, num_len, false);
    Ok(format!(
        "oxygen_rating is {:b} and co2_rating is {:b}. Life support is {}",
        oxygen_rating,
        co2_rating,
        oxygen_rating as u64 * co2_rating as u64
    ))
}
