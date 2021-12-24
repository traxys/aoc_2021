use crate::{day, utils::split2, EyreResult};
use dlopen::wrapper::{Container, WrapperApi};
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{io::Write, str::FromStr};

day! {
    parser,
    part1 => "{}",
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Variable {
    W,
    X,
    Y,
    Z,
}

impl Variable {
    fn str(&self) -> &'static str {
        match self {
            Variable::W => "w",
            Variable::X => "x",
            Variable::Y => "y",
            Variable::Z => "z",
        }
    }
}

impl FromStr for Variable {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(color_eyre::eyre::eyre!("Invalid variable")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Val {
    Var(Variable),
    Num(i64),
}

impl FromStr for Val {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::Var(Variable::W)),
            "x" => Ok(Self::Var(Variable::X)),
            "y" => Ok(Self::Var(Variable::Y)),
            "z" => Ok(Self::Var(Variable::Z)),
            _ => Ok(Self::Num(s.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Instr {
    Inp(Variable),
    Op(Operation, Variable, Val),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Operation {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operation::Add => "+",
            Operation::Mul => "*",
            Operation::Div => "/",
            Operation::Mod => "%",
            Operation::Eql => "=",
        };
        write!(f, "{}", op)
    }
}

impl FromStr for Instr {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, operands) =
            split2(s, " ").ok_or(color_eyre::eyre::eyre!("Malformed instruction"))?;

        let operation = match instr {
            "inp" => return Ok(Self::Inp(operands.parse()?)),
            "add" => Operation::Add,
            "mul" => Operation::Mul,
            "mod" => Operation::Mod,
            "div" => Operation::Div,
            "eql" => Operation::Eql,
            _ => return Err(color_eyre::eyre::eyre!("Invalid instruction")),
        };

        let (a, b) = split2(operands, " ").ok_or(color_eyre::eyre::eyre!("Malformed operands"))?;

        Ok(Self::Op(operation, a.parse()?, b.parse()?))
    }
}

#[derive(Debug)]
#[repr(C)]
struct Registers {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(dlopen_derive::WrapperApi)]
struct LibMONAD {
    calculate: unsafe extern "C" fn(input: SendPtr<i64>) -> Registers,
}

fn generate_program(instrs: &[Instr]) -> String {
    let mut func = r#"
        #include <stdint.h>
        struct registers {
            int64_t w;
            int64_t x;
            int64_t y;
            int64_t z;
        };

        struct registers calculate(int64_t *input) {
            struct registers regs = { 0 };
"#
    .to_string();

    for instr in instrs {
        match instr {
            Instr::Inp(_) => func += "\t\tregs.w = *input;\n\t\tinput++;\n",
            Instr::Op(op, a, b) => {
                let op = match op {
                    Operation::Add => "+",
                    Operation::Mul => "*",
                    Operation::Div => "/",
                    Operation::Mod => "%",
                    Operation::Eql => "==",
                };

                let b = match b {
                    Val::Var(v) => format!("regs.{}", v.str()),
                    Val::Num(n) => n.to_string(),
                };

                func += &format!("\t\tregs.{} = regs.{} {} {};\n", a.str(), a.str(), op, b);
            }
        }
    }

    func += "\t\treturn regs;\n}";

    func
}

type Parsed = Vec<Instr>;

#[repr(transparent)]
struct SendPtr<T>(*const T);

unsafe impl<T> Send for SendPtr<T> {}

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    input.lines().map(|l| l.trim().parse()).collect()
}

pub(crate) fn part1(instrs: Parsed) -> EyreResult<i64> {
    let program = generate_program(&instrs);
    let mut c_source = tempfile::NamedTempFile::new()?;
    c_source.write_all(program.as_bytes())?;

    let c_libdir = tempfile::tempdir()?;

    std::process::Command::new("gcc")
        .arg("--language=c")
        .arg("-o")
        .arg(c_libdir.path().join("libmonad.so"))
        .arg("-shared")
        .arg("-O2")
        .arg(c_source.path())
        .output()?;

    let cont: Container<LibMONAD> =
        unsafe { Container::load(c_libdir.path().join("libmonad.so"))? };

    let input =
        (1..10)
            .into_par_iter()
            .map(|a0| {
                (1..10)
                    .into_par_iter()
                    .map(move |a1| {
                        (1..10)
                            .into_par_iter()
                            .map(move |a2| {
                                (1..10)
                                    .into_par_iter()
                                    .map(move |a3| {
                                        (1..10)
                                            .into_par_iter()
                                            .map(move |a4| {
                                                (1..10)
                                                    .into_par_iter()
                                                    .map(move |a5| {
                                                        (1..10).into_par_iter().map(move |a6| {
                                (1..10).into_par_iter().map(move |a7| {
                                    (1..10).into_par_iter().map(move |a8| {
                                        (1..10).into_par_iter().map(move |a9| {
                                            (1..10).into_par_iter().map(move |a10| {
                                                (1..10).into_par_iter().map(move |a11| {
                                                    (1..10).into_par_iter().map(move |a12| {
                                                        (1..10).into_par_iter().map(move |a13| {
                                                            [
                                                                a0, a1, a2, a3, a4, a5, a6, a7, a8,
                                                                a9, a10, a11, a12, a13,
                                                            ]
                                                        })
                                                    }).flatten()
                                                }).flatten()
                                            }).flatten()
                                        }).flatten()
                                    }).flatten()
                                }).flatten()
                            }).flatten()
                                                    })
                                                    .flatten()
                                            })
                                            .flatten()
                                    })
                                    .flatten()
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten().progress_count(9u64.pow(14))
            .map(|input| {
                let input_ptr = SendPtr(input.as_ptr());
                (input, unsafe { cont.calculate(input_ptr).z })
            })
            .find_any(|(_, z)| *z == 0);

    dbg!(input);

    todo!()
}

pub(crate) fn part2(_: Parsed) -> EyreResult<()> {
    todo!()
}
