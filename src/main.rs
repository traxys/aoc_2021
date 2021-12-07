use chrono::Datelike;
use color_eyre::eyre::{self, Context};
use std::{fs::OpenOptions, io::Write, path::PathBuf, str::FromStr};
use structopt::StructOpt;

mod harness;

use harness::{Restart, RunParams};

pub type EyreResult<T, E = color_eyre::Report> = Result<T, E>;

pub(crate) mod utils;

pub(crate) mod day1;
pub(crate) mod day2;
pub(crate) mod day3;
pub(crate) mod day4;
pub(crate) mod day5;
pub(crate) mod day6;
pub(crate) mod day7;

solutions! {
    day 1 {
        module: day1,
        linking: Restart,
    },
    day 2 {
        module: day2,
        linking: Restart, 
    },
    day 3 {
        module: day3,
        linking: Restart,
    },
    day 4 {
        module: day4,
        linking: Restart,
    },
    day 5 {
        module: day5,
        linking: Restart,
    },
    day 6 {
        module: day6,
        linking: Restart,
    },
    day 7 {
        module: day7,
        linking: Restart,
    }
}

#[derive(Debug)]
enum Day {
    Latest,
    Specific(usize),
}

impl FromStr for Day {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "latest" => Ok(Self::Latest),
            other => other
                .trim_start_matches("day")
                .trim_start_matches(|c| c == '-' || c == '_')
                .parse()
                .map(Self::Specific)
                .map_err(Into::into),
        }
    }
}

#[derive(Debug)]
enum Input {
    Day,
    Path(PathBuf),
}

impl FromStr for Input {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            other => Ok(Self::Path(other.parse()?)),
        }
    }
}

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long, possible_values = POSSIBLE_DAYS,
default_value = "latest")]
    day: Day,
    #[structopt(long, short, possible_values = &["1", "2"],
default_value = "1")]
    part: usize,
    #[structopt(long)]
    no_timings: bool,
    #[structopt(short, long, default_value = "day")]
    input: Input,
    #[structopt(long, short, env = "AOC_SESSION")]
    session: Option<String>,
    #[structopt(long)]
    skip_dl: bool,
    #[structopt(long, default_value = "input")]
    dl_dir: PathBuf,
    #[structopt(long, short)]
    year: Option<usize>,
}

fn main() -> EyreResult<()> {
    color_eyre::install()?;
    let args = Args::from_args();

    let day = resolve_day(args.day);

    let input = match args.input {
        Input::Day => {
            let mut path = args.dl_dir;
            path.push(format!("day{}", day));
            if !path.exists() {
                if args.skip_dl {
                    eyre::bail!("Input for current day is not downloaded and skip-dl = true");
                }

                let session = match args.session {
                    None => eyre::bail!("No session provided while needing to download input"),
                    Some(s) => s,
                };

                let year = args
                    .year
                    .unwrap_or_else(|| chrono::Local::now().year() as usize);

                let client = reqwest::blocking::Client::new();
                let body = client
                    .get(&format!(
                        "https://adventofcode.com/{}/day/{}/input",
                        year, day
                    ))
                    .header("Cookie", format!("session={}", session))
                    .send()
                    .with_context(|| {
                        format!("Could not fetch the input for day {} of AoC {}", day, year)
                    })?
                    .error_for_status()
                    .with_context(|| {
                        format!("Error accessing the input for day {} of AoC {}", day, year)
                    })?
                    .text()
                    .with_context(|| "Error reading the body of the response")?;

                let mut writer = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&path)
                    .with_context(|| format!("Could not open file at {:?}", path))?;

                writer
                    .write_all(body.as_bytes())
                    .with_context(|| format!("Could not write to file {:?}", path))?;
            }
            std::fs::read_to_string(path)?
        }
        Input::Path(p) => std::fs::read_to_string(p)?,
    };

    run_solution(
        day,
        args.part,
        &input,
        RunParams {
            timings: !args.no_timings,
        },
    )?;
    Ok(())
}
