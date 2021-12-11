use std::time::{Duration, Instant};

pub(crate) struct RunParams {
    pub(crate) timings: bool,
}

pub(crate) struct Timings {
    parse: Duration,
    inter: Option<Duration>,
    part: Duration,
}

pub(crate) struct ExtraInfo {
    timings: Option<Timings>,
}

impl ExtraInfo {
    fn render(&self) -> Option<String> {
        self.timings.as_ref().map(|timings| {
            let mut out = format!(
                "  Timings:\n    - parsing: {}\n",
                humantime::format_duration(timings.parse)
            );
            if let Some(inter) = timings.inter {
                out += &format!(
                    "    - intermediary (part1): {}\n",
                    humantime::format_duration(inter)
                )
            }
            out += &format!(
                "    - solution: {}",
                humantime::format_duration(timings.part)
            );
            out
        })
    }
}

pub(crate) fn render(
    solution: String,
    parsed_time: Duration,
    part_time: Duration,
    timings: bool,
) -> String {
    let info = ExtraInfo {
        timings: if timings {
            Some(Timings {
                parse: parsed_time,
                part: part_time,
                inter: None,
            })
        } else {
            None
        },
    };

    match info.render() {
        None => solution,
        Some(info) => solution + "\n" + &info,
    }
}

pub(crate) fn time_func<F, I, O>(mut f: F, a: I) -> (O, Duration)
where
    F: FnMut(I) -> O,
{
    let start = Instant::now();
    let res = f(a);
    (res, start.elapsed())
}

#[derive(Clone, Copy)]
pub(crate) enum Part {
    Part1,
    Part2,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            Part::Part1 => 1,
            Part::Part2 => 2,
        };
        write!(f, "{}", p)
    }
}

#[macro_export]
macro_rules! day {
    (
        $parser:path,
        $part1:path => $fmt1:literal $(,)?
    ) => {
        pub(crate) fn solver_latest() -> crate::Part {
            crate::Part::Part1
        }

        pub(crate) fn solve<'i>(
            part: crate::Part,
            input: &'i str,
            params: crate::harness::RunParams,
        ) -> crate::EyreResult<String> {
            let (parsed, parsed_time) = crate::harness::time_func($parser, input);
            let (result, part_time) = match part {
                crate::Part::Part1 => {
                    let (part, part_time) = crate::harness::time_func($part1, parsed?);
                    (format!($fmt1, part?), part_time)
                }
                crate::Part::Part2 => color_eyre::eyre::bail!("part 2 is not implemented"),
            };

            Ok(crate::harness::render(
                result,
                parsed_time,
                part_time,
                params.timings,
            ))
        }
    };
    (
        $parser:path,
        $part1:path => $fmt1:literal,
        $part2:path => $fmt2:literal $(,)?
    ) => {
        pub(crate) fn solver_latest() -> crate::Part {
            crate::Part::Part2
        }

        pub(crate) fn solve<'i>(
            part: crate::Part,
            input: &'i str,
            params: crate::harness::RunParams,
        ) -> crate::EyreResult<String> {
            let (parsed, parsed_time) = crate::harness::time_func($parser, input);
            let (result, part_time) = match part {
                crate::Part::Part1 => {
                    let (part, part_time) = crate::harness::time_func($part1, parsed?);
                    (format!($fmt1, part?), part_time)
                }
                crate::Part::Part2 => {
                    let (part, part_time) = crate::harness::time_func($part2, parsed?);
                    (format!($fmt2, part?), part_time)
                }
            };

            Ok(crate::harness::render(
                result,
                parsed_time,
                part_time,
                params.timings,
            ))
        }
    };
}

#[macro_export]
macro_rules! days {
    ($($day:literal = $mod:ident,)*) => {
        const IMPLEMENTED_DAYS: &[usize] = &[$($day,)*];
        const POSSIBLE_DAYS: &[&str] = &["latest", $(stringify!($day),)*];

        $(
            pub(crate) mod $mod;
        )*

        fn resolve_day(day: Day) -> usize {
            match day {
                Day::Latest => *IMPLEMENTED_DAYS.last().unwrap(),
                Day::Specific(x) => x,
            }
        }

        fn run_solution(
            day: usize,
            part: Option<crate::Part>,
            input: &str,
            params: crate::harness::RunParams
        ) -> EyreResult<()> {
            match day {
                $(
                    $day => {
                        let part = part.unwrap_or($mod::solver_latest());
                        let result = $mod::solve(part, input, params)?;
                        println!("Day {} Part {}:\n  {}", day, part, result);
                        Ok(())
                    }
                )*
                _ => eyre::bail!("Day {} was not implemented", day),
            }
        }
    };
}

#[macro_export]
macro_rules! solutions {
     ($(
         day $day:literal {
             module: $mod:path,
             linking: $link:ty $(,)?
         }
     ),* $(,)?) => {
         const IMPLEMENTED_DAYS: &[usize] = &[$($day,)*];
         const POSSIBLE_DAYS: &[&str] = &["latest", $(stringify!($day),)*];

         fn resolve_day(day: Day) -> usize {
             match day {
                 Day::Latest => *IMPLEMENTED_DAYS.last().unwrap(),
                 Day::Specific(x) => x,
             }
         }

         fn run_solution(day: usize, part: usize, input: &str, params: crate::harness::RunParams) -> EyreResult<()> {
             match day {
                 $(
                     $day => {
                         use $mod as module;

                         let mut problem: crate::harness::Problem<_, _, _, _, _, $link, _, _, _> = crate::harness::Problem {
                             parser: module::parser,
                             part1: module::part1,
                             part2: module::part2,
                             fmt1: module::fmt1,
                             fmt2: module::fmt2,
                             _cont: Default::default(),
                             _inp: Default::default(),
                             _outp1: Default::default(),
                             _outp2: Default::default(),
                         };
                         let result = match part {
                             1 => problem.part1(input, params)?,
                             2 => problem.part2(input, params)?,
                             _ => eyre::bail!("Part {} is not possible", part),
                         };
                         println!("Day {} Part {}:\n  {}", day, part, result);
                     },
                 )*
                 _ => eyre::bail!("Day {} was not implemented", day),
             };
             Ok(())
         }
     };
}
