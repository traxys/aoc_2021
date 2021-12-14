use color_eyre::eyre;

use crate::{day, utils::split2, EyreResult};
use std::collections::HashMap;

day! {
    parser,
    part1 => "Most common minus least common is {}",
    part2 => "Most common minus least common is {}",
}

type Parsed = (Vec<u8>, HashMap<(u8, u8), u8>);

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let (template, rules) = split2(input, "\n\n").ok_or(eyre::eyre!("No empty line"))?;
    let rules = rules
        .lines()
        .try_fold(HashMap::new(), |mut map, rule| -> EyreResult<_> {
            let (pair, produce) = split2(rule, " -> ").ok_or(eyre::eyre!("rule has no ->"))?;
            let pair = pair.as_bytes();
            map.insert((pair[0], pair[1]), produce.as_bytes()[0]);
            Ok(map)
        })?;
    Ok((template.as_bytes().into(), rules))
}

fn min_max_diff(amounts: &HashMap<u8, u64>) -> u64 {
    let ((_, minamount), (_, maxamount)) = amounts.iter().fold(
        ((-1, 0), (-1, 0)),
        |((minkey, minamount), (maxkey, maxamount)), (&key, &amount)| {
            (
                if minkey == -1 || minamount > amount {
                    (key as i16, amount as u64)
                } else {
                    (minkey, minamount)
                },
                if maxkey == -1 || maxamount < amount {
                    (key as i16, amount as u64)
                } else {
                    (maxkey, maxamount)
                },
            )
        },
    );

    maxamount - minamount
}

fn polymerize_pairs(
    chain: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
    steps: usize,
) -> HashMap<u8, u64> {
    let mut pairs = {
        let chain: &[u8] = &chain;
        chain
            .iter()
            .zip(chain.iter().skip(1))
            .fold(HashMap::new(), |mut pairs, (&a, &b)| {
                *pairs.entry((a, b)).or_insert(0) += 1;
                pairs
            })
    };

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        for (pair, amount) in pairs {
            let product = *rules.get(&pair).unwrap();
            *new_pairs.entry((pair.0, product)).or_insert(0) += amount;
            *new_pairs.entry((product, pair.1)).or_insert(0) += amount;
        }
        pairs = new_pairs;
    }

    let mut amounts = HashMap::new();

    for ((_, a), amount) in pairs {
        *amounts.entry(a).or_insert(0) += amount;
    }
    *amounts.entry(chain[0]).or_insert(0) += 1;

    amounts
}

pub(crate) fn part1((template, rules): Parsed) -> EyreResult<u64> {
    Ok(min_max_diff(&polymerize_pairs(template, rules, 10)))
}

pub(crate) fn part2((template, rules): Parsed) -> EyreResult<u64> {
    Ok(min_max_diff(&polymerize_pairs(template, rules, 40)))
}
