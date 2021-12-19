use crate::{day, utils::split2, EyreResult};
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

day! {
    parser,
    part1 => "There are {} beacons",
    part2 => "The largest distance is {}",
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

type Rotation = [[i64; 3]; 3];

const ROTATION_GROUP: [Rotation; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
];

fn compose_rotation(a: Rotation, b: Rotation) -> Rotation {
    let mut c = [[0; 3]; 3];
    for (l, line) in c.iter_mut().enumerate() {
        for (c, cell) in line.iter_mut().enumerate() {
            *cell = a[l][0] * b[0][c] + a[l][1] * b[1][c] + a[l][2] * b[2][c];
        }
    }
    c
}

fn possible_dists(reference: &[Vec3], other: &[Vec3], rotation: Rotation) -> HashMap<Vec3, usize> {
    let mut dists = HashMap::new();

    for ref_point in reference {
        for other_point in other {
            let rotated_other = other_point.matmul(rotation);
            *dists.entry(*ref_point - rotated_other).or_insert(0) += 1;
        }
    }

    dists
}

fn try_map<'i>(
    reference: &'i [Vec3],
    other: &'i [Vec3],
) -> impl Iterator<Item = (Vec3, Rotation)> + 'i {
    ROTATION_GROUP.into_iter().filter_map(|rotation| {
        let mapping = possible_dists(reference, other, rotation);
        let mut mapping = mapping.iter().filter(|(_, v)| **v >= 12);
        if let Some((off, _)) = mapping.next() {
            assert!(mapping.next().is_none());
            Some((*off, rotation))
        } else {
            None
        }
    })
}

fn recover_position(
    idx: usize,
    scanners: &[Vec<Vec3>],
    current: &mut HashMap<(usize, usize), (Vec3, Rotation)>,
    evaluating: &mut HashSet<(usize, usize)>,
) -> Option<(Vec3, Rotation)> {
    if idx == 0 {
        Some((Vec3 { x: 0, y: 0, z: 0 }, ROTATION_GROUP[0]))
    } else {
        'other: for (refidx, reference) in scanners.iter().enumerate() {
            if refidx == idx {
                continue;
            }

            match current.get(&(idx, refidx)) {
                Some(p) => return Some(*p),
                None => (),
            }

            for (mappos, maprot) in try_map(reference, &scanners[idx]) {
                let key = (idx, refidx);
                let revkey = (refidx, idx);
                if !evaluating.contains(&key) && !evaluating.contains(&revkey) {
                    evaluating.insert(key);
                    evaluating.insert(revkey);

                    let (refpos, refrot) =
                        match recover_position(refidx, scanners, current, evaluating) {
                            None => {
                                evaluating.remove(&key);
                                evaluating.remove(&revkey);
                                continue 'other;
                            }
                            Some(v) => v,
                        };

                    evaluating.remove(&key);
                    evaluating.remove(&revkey);

                    let rotatated_pos = mappos.matmul(refrot);
                    let position = rotatated_pos + refpos;
                    let total_rot = compose_rotation(refrot, maprot);

                    current.insert((idx, refidx), (position, total_rot));

                    return Some((position, total_rot));
                }
            }
        }

        None
    }
}

fn recover_positions(scanners: &[Vec<Vec3>]) -> Vec<(Vec3, Rotation)> {
    let mut coords = Vec::with_capacity(scanners.len());
    coords.push((Vec3 { x: 0, y: 0, z: 0 }, ROTATION_GROUP[0]));
    let mut mappings = HashMap::new();
    let mut evaluating = HashSet::new();

    for idx in 1..scanners.len() {
        coords.push(
            recover_position(idx, scanners, &mut mappings, &mut evaluating)
                .expect("Should always find 0"),
        );
        assert!(evaluating.is_empty());
    }

    coords
}

impl Vec3 {
    fn dot(&self, line: [i64; 3]) -> i64 {
        self.x * line[0] + self.y * line[1] + self.z * line[2]
    }

    fn matmul(&self, mat: [[i64; 3]; 3]) -> Self {
        Self {
            x: self.dot(mat[0]),
            y: self.dot(mat[1]),
            z: self.dot(mat[2]),
        }
    }

    fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day19::Vec3;

    use super::{parser, ROTATION_GROUP};

    #[test]
    fn same_rotation() {
        let input = include_str!("../examples/day19-samerot");
        let scanners = parser(input).unwrap();

        'outer: for scanner in scanners.iter().skip(1) {
            for rot in ROTATION_GROUP {
                let rotated: Vec<_> = scanner.iter().map(|r| r.matmul(rot)).collect();
                if &rotated == &scanners[0] {
                    continue 'outer;
                }
            }
            panic!("Could not find a matching rotation")
        }
    }

    #[test]
    fn mapping() {
        use super::try_map;

        let input = include_str!("../examples/day19");
        let scanners = parser(input).unwrap();

        let (mapping, _) = try_map(&scanners[0], &scanners[1]).next().unwrap();
        assert_eq!(
            mapping,
            Vec3 {
                x: 68,
                y: -1246,
                z: -43
            }
        );
    }

    #[test]
    fn all_mapping() {
        use super::recover_positions;

        let input = include_str!("../examples/day19");
        let scanners = parser(input).unwrap();

        macro_rules! vec3 {
            ($x:expr, $y:expr, $z:expr $(,)?) => {
                Vec3 {
                    x: $x,
                    y: $y,
                    z: $z,
                }
            };
        }

        assert_eq!(
            recover_positions(&scanners)
                .into_iter()
                .map(|(pos, _)| pos)
                .collect::<Vec<_>>(),
            &[
                vec3!(0, 0, 0),
                vec3!(68, -1246, -43),
                vec3!(1105, -1205, 1229),
                vec3!(-92, -2380, -20),
                vec3!(-20, -1133, 1061),
            ]
        );
    }
}

type Parsed = Vec<Vec<Vec3>>;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|coords| {
                    let (x, yz) =
                        split2(coords, ",").ok_or(color_eyre::eyre::eyre!("malformed coords"))?;
                    let (y, z) =
                        split2(yz, ",").ok_or(color_eyre::eyre::eyre!("malformed coords"))?;
                    Ok(Vec3 {
                        x: x.parse()?,
                        y: y.parse()?,
                        z: z.trim().parse()?,
                    })
                })
                .collect()
        })
        .collect()
}

pub(crate) fn part1(scanners: Parsed) -> EyreResult<usize> {
    let positions = recover_positions(&scanners);

    let mut beacons = HashSet::new();
    for (scanner, (refpos, refrot)) in scanners.iter().zip(&positions) {
        for beacon in scanner {
            let pos = beacon.matmul(*refrot) + *refpos;
            beacons.insert(pos);
        }
    }

    Ok(beacons.len())
}

pub(crate) fn part2(scanners: Parsed) -> EyreResult<i64> {
    let positions = recover_positions(&scanners);

    Ok(positions
        .iter()
        .map(|&(x, _)| positions.iter().map(move |&(y, _)| (x, y)))
        .flatten()
        .map(|(x, y)| (x - y).norm())
        .max()
        .unwrap())
}
