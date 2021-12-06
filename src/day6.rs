use crate::EyreResult;

type Parsed = [u64; 9];

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input.split(",").try_fold([0; 9], |mut current, fish| {
        let idx = match fish.trim() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            _ => color_eyre::eyre::bail!("No such fish: {}", fish),
        };
        current[idx] += 1;
        Ok(current)
    })
}

fn step(current: [u64; 9]) -> [u64; 9] {
    let mut next = [0; 9];
    for i in 1..9 {
        next[i - 1] = current[i];
    }
    next[6] += current[0];
    next[8] += current[0];
    next
}

pub(crate) fn part1(mut fishes: Parsed) -> EyreResult<u64> {
    for _ in 0..80 {
        fishes = step(fishes)
    }
    Ok(fishes.iter().sum())
}

pub(crate) fn part2(mut fishes: Parsed) -> EyreResult<u64> {
    for _ in 0..256 {
        fishes = step(fishes)
    }
    Ok(fishes.iter().sum())
}

pub(crate) fn fmt1(output: u64) -> String {
    format!("After 80 days there are {} fishes", output)
}

pub(crate) fn fmt2(output: u64) -> String {
    format!("After 256 days there are {} fishes", output)
}

#[cfg(test)]
mod test {
    use super::{parser, step};

    #[test]
    fn step_one() {
        assert_eq!(
            step(parser("1,2,1,6,0,8").unwrap()),
            parser("0,1,0,5,6,7,8").unwrap()
        );
    }

    #[test]
    fn example() {
        let mut state = parser("3,4,3,1,2").unwrap();
        for _ in 0..18 {
            state = step(state);
        }
        assert_eq!(
            state,
            parser("6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8").unwrap()
        );
    }
}
