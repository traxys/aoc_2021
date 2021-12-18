use crate::{day, EyreResult};

day! {
    parser,
    part1 => "Sum of all numbers is {}",
    part2 => "Largest sum is {}",
}

#[derive(Clone)]
pub(crate) enum Pair {
    Number(u64),
    Composite(Box<Pair>, Box<Pair>),
}

impl Pair {
    fn as_number(&self) -> Option<u64> {
        match self {
            &Pair::Number(n) => Some(n),
            Pair::Composite(_, _) => None,
        }
    }

    fn as_number_pair(&self) -> Option<(u64, u64)> {
        match self {
            Self::Number(_) => None,
            Self::Composite(l, r) => {
                let l = l.as_number()?;
                let r = r.as_number()?;
                Some((l, r))
            }
        }
    }

    fn exploding_reduce(self, depth: u8) -> (Pair, Option<u64>, Option<u64>, bool) {
        match self {
            na @ Pair::Number(_) => (na, None, None, false),
            p @ Pair::Composite(_, _) if depth >= 4 => match p.as_number_pair() {
                None => todo!("Handle this case"),
                Some((a, b)) => (Self::Number(0), Some(a), Some(b), true),
            },
            Pair::Composite(l, r) => {
                let (l, nl, nr, reducedl) = l.exploding_reduce(depth + 1);
                if reducedl {
                    let r = match nr {
                        None => r,
                        Some(n) => Box::new(r.add_left(n)),
                    };
                    (Pair::Composite(Box::new(l), r), nl, None, true)
                } else {
                    let (r, nl, nr, reduced) = r.exploding_reduce(depth + 1);
                    let l = match nl {
                        None => Box::new(l),
                        Some(n) => Box::new(l.add_right(n)),
                    };
                    (Pair::Composite(l, Box::new(r)), None, nr, reduced)
                }
            }
        }
    }

    fn split(self) -> (Pair, bool) {
        match self {
            Pair::Number(n) if n > 9 => {
                let a = n / 2;
                let b = n / 2 + (n % 2);
                (
                    Pair::Composite(Box::new(Pair::Number(a)), Box::new(Pair::Number(b))),
                    true,
                )
            }
            p @ Pair::Number(_) => (p, false),
            Pair::Composite(l, r) => {
                let (l, rl) = l.split();
                if rl {
                    (Pair::Composite(Box::new(l), r), true)
                } else {
                    let (r, rr) = r.split();
                    (Pair::Composite(Box::new(l), Box::new(r)), rr)
                }
            }
        }
    }

    fn add_left(self, n: u64) -> Pair {
        match self {
            Self::Number(x) => Self::Number(x + n),
            Self::Composite(a, b) => Self::Composite(Box::new(a.add_left(n)), b),
        }
    }

    fn add_right(self, n: u64) -> Pair {
        match self {
            Self::Number(x) => Self::Number(x + n),
            Self::Composite(a, b) => Self::Composite(a, Box::new(b.add_right(n))),
        }
    }

    fn reduce(mut self) -> Pair {
        loop {
            let (new, _, _, reduced_explode) = self.exploding_reduce(0);
            self = new;
            if reduced_explode {
                continue;
            }

            let (new, reduced_split) = self.split();
            self = new;
            if reduced_split {
                continue;
            }

            break self;
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            &Pair::Number(n) => n,
            Pair::Composite(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Number(n) => write!(f, "{}", n),
            Pair::Composite(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

type Parsed = Vec<Pair>;

fn parse_pair(pair: &str) -> Pair {
    let mut stack = Vec::new();
    for b in pair.bytes() {
        match b {
            b'[' | b',' => (),
            b'0'..=b'9' => stack.push(Pair::Number((b - b'0') as u64)),
            b']' => {
                let r = Box::new(stack.pop().unwrap());
                let l = Box::new(stack.pop().unwrap());
                stack.push(Pair::Composite(l, r));
            }
            _ => unreachable!(),
        }
    }
    assert!(stack.len() == 1);
    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::parse_pair;

    #[test]
    fn list_add() {
        let input = r#"[1,1]
[2,2]
[3,3]
[4,4]"#;
        let result = super::list_add(super::parser(input).unwrap());
        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", result.to_string());

        let input = r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#;
        let result = super::list_add(super::parser(input).unwrap());
        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", result.to_string());
    }

    #[test]
    fn complete_reduce() {
        assert_eq!(
            parse_pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
                .reduce()
                .to_string(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn explode() {
        macro_rules! test_explode {
            ($in:expr, $out:expr, $l:expr, $r:expr) => {
                let (pair, left, right, reduced) = parse_pair($in).exploding_reduce(0);
                assert!(reduced);
                assert_eq!(pair.to_string(), $out);
                assert_eq!(left, $l);
                assert_eq!(right, $r);
            };
        }

        test_explode!("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", Some(9), None);
        test_explode!("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]", None, Some(2));
        test_explode!("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", None, None);
    }

    #[test]
    fn parsing() {
        macro_rules! parse_test {
            ($e:expr) => {
                assert_eq!($e, parse_pair($e).to_string())
            };
        }

        parse_test!("[1,2]");
        parse_test!("[[1,2],3]");
        parse_test!("[9,[8,7]]");
        parse_test!("[[1,9],[8,5]]");
        parse_test!("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }
}

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    Ok(input.lines().map(|l| parse_pair(l.trim())).collect())
}

fn list_add(p: Vec<Pair>) -> Pair {
    let mut numbers = p.into_iter();
    let start = numbers.next().unwrap();
    numbers.fold(start, |current, pair| {
        let next = Pair::Composite(Box::new(current), Box::new(pair));
        next.reduce()
    })
}

pub(crate) fn part1(numbers: Parsed) -> EyreResult<u64> {
    let result = list_add(numbers);
    Ok(result.magnitude())
}

pub(crate) fn part2(numbers: Parsed) -> EyreResult<u64> {
    let mut max = 0;
    for (ix, x) in numbers.iter().enumerate() {
        for (iy, y) in numbers.iter().enumerate() {
            if ix == iy {
                continue;
            }
            let mag = Pair::Composite(Box::new(x.clone()), Box::new(y.clone()))
                .reduce()
                .magnitude();
            if mag > max {
                max = mag;
            }
        }
    }
    Ok(max)
}
