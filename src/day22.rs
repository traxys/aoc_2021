use std::ops::RangeInclusive;

use arrayvec::ArrayVec;
use itertools::iproduct;

use crate::{day, utils::split2, EyreResult};

day! {
    parser,
    part1 => "There are {} cubes for startup",
    part2 => "There are {} cubes in total",
}

fn range_overlap(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    std::cmp::max(*a.start(), *b.start())..=std::cmp::min(*a.end(), *b.end())
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CubeRange {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

fn interior(
    large: &RangeInclusive<i64>,
    internal: &RangeInclusive<i64>,
) -> [RangeInclusive<i64>; 3] {
    [
        *large.start()..=*internal.start() - 1,
        internal.clone(),
        *internal.end() + 1..=*large.end(),
    ]
}

impl CubeRange {
    fn intersects(&self, other: &Self) -> bool {
        let inter = self.intersection(other);
        !inter.empty()
    }

    fn split(&self, negative: &Self) -> ArrayVec<CubeRange, { 27 - 1 }> {
        let intersection = self.intersection(negative);

        let x = interior(&self.x, &intersection.x);
        let y = interior(&self.y, &intersection.y);
        let z = interior(&self.z, &intersection.z);

        iproduct!(x, y, z)
            .map(|(x, y, z)| CubeRange { x, y, z })
            .filter(|r| !r.empty() && r != &intersection)
            .collect()
    }

    fn intersection(&self, other: &Self) -> CubeRange {
        Self {
            x: range_overlap(&self.x, &other.x),
            y: range_overlap(&self.y, &other.y),
            z: range_overlap(&self.z, &other.z),
        }
    }

    fn empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn count(&self) -> i64 {
        if self.empty() {
            0
        } else {
            (self.x.end() - self.x.start() + 1).abs()
                * (self.y.end() - self.y.start() + 1).abs()
                * (self.z.end() - self.z.start() + 1).abs()
        }
    }
}

fn append_ranges(ranges: &mut Vec<CubeRange>, range: CubeRange) {
    if ranges.iter().any(|r| r.intersects(&range)) {
        let mut parts = vec![range];

        for r in ranges.iter() {
            let mut new_parts = Vec::with_capacity(parts.len());

            for part in parts {
                if r.intersects(&part) {
                    new_parts.extend(part.split(&r));
                } else {
                    new_parts.push(part);
                }
            }

            parts = new_parts;
        }

        ranges.append(&mut parts);
    } else {
        ranges.push(range);
    }
}

fn sum_range(range: &[CubeRange]) -> i64 {
    range.iter().map(|x| x.count()).sum()
}

fn cut_ranges(ranges: &mut Vec<CubeRange>, negative: CubeRange) {
    if ranges.iter().any(|r| r.intersects(&negative)) {
        let mut new_ranges = Vec::with_capacity(ranges.len());

        for r in ranges.drain(..) {
            if r.intersects(&negative) {
                new_ranges.extend(r.split(&negative));
            } else {
                new_ranges.push(r);
            }
        }

        *ranges = new_ranges;
    }
}

fn accumulate_ranges<I>(ranges: I) -> Vec<CubeRange>
where
    I: Iterator<Item = (bool, CubeRange)>,
{
    let mut total_ranges: Vec<CubeRange> = Vec::new();

    for (on, range) in ranges {
        if on {
            append_ranges(&mut total_ranges, range);
        } else {
            cut_ranges(&mut total_ranges, range);
        }
    }

    total_ranges
}

#[cfg(test)]
mod test {
    use super::CubeRange;

    #[test]
    fn count() {
        assert_eq!(
            CubeRange {
                x: 10..=12,
                y: 10..=12,
                z: 10..=12
            }
            .count(),
            27
        );
    }

    #[test]
    fn create_ranges() {
        use super::{append_ranges, sum_range};

        let mut parts = vec![CubeRange {
            x: 10..=12,
            y: 10..=12,
            z: 10..=12,
        }];

        append_ranges(
            &mut parts,
            CubeRange {
                x: 11..=13,
                y: 11..=13,
                z: 11..=13,
            },
        );

        assert_eq!(sum_range(&parts), 27 + 19)
    }

    #[test]
    fn cut_ranges() {
        use super::{append_ranges, cut_ranges, sum_range};

        let mut parts = vec![CubeRange {
            x: 10..=12,
            y: 10..=12,
            z: 10..=12,
        }];

        append_ranges(
            &mut parts,
            CubeRange {
                x: 11..=13,
                y: 11..=13,
                z: 11..=13,
            },
        );

        assert_eq!(sum_range(&parts), 27 + 19);

        cut_ranges(
            &mut parts,
            CubeRange {
                x: 9..=11,
                y: 9..=11,
                z: 9..=11,
            },
        );

        assert_eq!(sum_range(&parts), 27 + 19 - 8);
    }
}

type Parsed = Vec<(bool, CubeRange)>;

fn parse_range(range: &str) -> EyreResult<RangeInclusive<i64>> {
    let (_, range) = split2(range, "=").ok_or(color_eyre::eyre::eyre!("Malformed range"))?;
    let (start, end) = split2(range, "..").ok_or(color_eyre::eyre::eyre!("Malformed range"))?;
    Ok(start.parse()?..=end.parse()?)
}

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input
        .lines()
        .map(|l| -> EyreResult<_> {
            let (state, ranges) =
                split2(l.trim(), " ").ok_or(color_eyre::eyre::eyre!("Malformed range"))?;
            let (x, yz) = split2(ranges, ",").ok_or(color_eyre::eyre::eyre!("Malformed ranges"))?;
            let (y, z) = split2(yz, ",").ok_or(color_eyre::eyre::eyre!("Malformed ranges"))?;
            Ok((
                state == "on",
                CubeRange {
                    x: parse_range(x)?,
                    y: parse_range(y)?,
                    z: parse_range(z)?,
                },
            ))
        })
        .collect()
}

pub(crate) fn part1(ranges: Parsed) -> EyreResult<i64> {
    let ranges = accumulate_ranges(ranges.into_iter().filter(|(_, r)| {
        r.intersects(&CubeRange {
            x: -50..=50,
            y: -50..=50,
            z: -50..=50,
        })
    }));

    Ok(sum_range(&ranges))
}

pub(crate) fn part2(ranges: Parsed) -> EyreResult<i64> {
    let ranges = accumulate_ranges(ranges.into_iter());

    Ok(sum_range(&ranges))
}
