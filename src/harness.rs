use crate::EyreResult;
use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

#[allow(unused)]
pub(crate) struct Continue;
pub(crate) struct Restart;

pub(crate) struct RunParams {
    pub(crate) timings: bool,
}

struct Timings {
    parse: Duration,
    inter: Option<Duration>,
    part: Duration,
}

struct ExtraInfo {
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

pub(crate) struct Problem<P, S1, F1, S2, F2, C, Inp, O1, O2> {
    pub(crate) parser: P,
    pub(crate) part1: S1,
    pub(crate) fmt1: F1,
    pub(crate) part2: S2,
    pub(crate) fmt2: F2,
    pub(crate) _cont: PhantomData<C>,
    pub(crate) _inp: PhantomData<Inp>,
    pub(crate) _outp1: PhantomData<O1>,
    pub(crate) _outp2: PhantomData<O2>,
}

impl<P, S1, F1, S2, F2, C, Inp, O1, O2> Problem<P, S1, F1, S2, F2, C, Inp, O1, O2>
where
    F1: FnMut(O1) -> String,
{
    fn fmt1(&mut self, input: O1, extra: ExtraInfo) -> String {
        let res = (self.fmt1)(input);
        match extra.render() {
            None => res,
            Some(info) => res + "\n" + &info,
        }
    }
}

impl<P, S1, F1, S2, F2, C, Inp, O1, O2> Problem<P, S1, F1, S2, F2, C, Inp, O1, O2>
where
    F2: FnMut(O2) -> String,
{
    fn fmt2(&mut self, input: O2, extra: ExtraInfo) -> String {
        let res = (self.fmt2)(input);
        match extra.render() {
            None => res,
            Some(info) => res + "\n" + &info,
        }
    }
}

fn time_func<F, I, O>(mut f: F, a: I) -> (O, Duration)
where
    F: FnMut(I) -> O,
{
    let start = Instant::now();
    let res = f(a);
    (res, start.elapsed())
}

impl<'i, P, S1, F1, S2, F2, C, Inp, O1, O2> Problem<P, S1, F1, S2, F2, C, Inp, O1, O2>
where
    P: FnMut(&'i str) -> EyreResult<Inp>,
    S1: FnMut(Inp) -> EyreResult<O1>,
    F1: FnMut(O1) -> String,
{
    pub(crate) fn part1(&mut self, input: &'i str, params: RunParams) -> EyreResult<String> {
        let (parsed, parsed_time) = time_func(&mut self.parser, input);
        let (part1, part1_time) = time_func(&mut self.part1, parsed?);

        let timings = if params.timings {
            Some(Timings {
                parse: parsed_time,
                inter: None,
                part: part1_time,
            })
        } else {
            None
        };

        Ok(self.fmt1(part1?, ExtraInfo { timings }))
    }
}

impl<'i, P, S1, F1, S2, F2, Inp, O1, O2> Problem<P, S1, F1, S2, F2, Continue, Inp, O1, O2>
where
    P: FnMut(&'i str) -> EyreResult<Inp>,
    S1: FnMut(Inp) -> EyreResult<O1>,
    S2: FnMut(O1) -> EyreResult<O2>,
    F2: FnMut(O2) -> String,
{
    #[allow(unused)]
    pub(crate) fn part2(&mut self, input: &'i str, params: RunParams) -> EyreResult<String> {
        let (parsed, parsed_time) = time_func(&mut self.parser, input);
        let (part1, part1_time) = time_func(&mut self.part1, parsed?);
        let (part2, part2_time) = time_func(&mut self.part2, part1?);

        let timings = if params.timings {
            Some(Timings {
                parse: parsed_time,
                inter: Some(part1_time),
                part: part2_time,
            })
        } else {
            None
        };
        Ok(self.fmt2(part2?, ExtraInfo { timings }))
    }
}

impl<'i, P, S1, F1, F2, S2, Inp, O1, O2> Problem<P, S1, F1, S2, F2, Restart, Inp, O1, O2>
where
    P: FnMut(&'i str) -> EyreResult<Inp>,
    S2: FnMut(Inp) -> EyreResult<O2>,
    F2: FnMut(O2) -> String,
{
    pub(crate) fn part2(&mut self, input: &'i str, params: RunParams) -> EyreResult<String> {
        let (parsed, parsed_time) = time_func(&mut self.parser, input);
        let (part2, part_time) = time_func(&mut self.part2, parsed?);

        let timings = if params.timings {
            Some(Timings {
                parse: parsed_time,
                inter: None,
                part: part_time,
            })
        } else {
            None
        };

        Ok(self.fmt2(part2?, ExtraInfo { timings }))
    }
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
