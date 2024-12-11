use crate::Result;
use regex::{Captures, Regex};

pub fn part1(input: &str) -> Result<i32> {
	let instructions = parse_instructions(input)?;

	let mut res = 0;

	for instr in instructions {
		match instr {
			Instruction::Do => {}
			Instruction::Dont => {}
			Instruction::Mul(a, b) => {
				res += a * b;
			}
		}
	}

	Ok(res)
}

pub fn part2(input: &str) -> Result<i32> {
	let instructions = parse_instructions(input)?;

	let mut res = 0;
	let mut do_mode = true;

	for instr in instructions {
		match instr {
			Instruction::Do => {
				do_mode = true;
			}
			Instruction::Dont => {
				do_mode = false;
			}
			Instruction::Mul(a, b) => {
				if do_mode {
					res += a * b;
				}
			}
		}
	}

	Ok(res)
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>> {
	let re = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)")?;

	let res = re
		.captures_iter(input)
		.map(Instruction::try_from)
		.collect::<Result<Vec<Instruction>>>()?;

	Ok(res)
}

enum Instruction {
	Do,
	Dont,
	Mul(i32, i32),
}

impl<'e> TryFrom<Captures<'e>> for Instruction {
	type Error = anyhow::Error;

	fn try_from(value: Captures<'e>) -> Result<Self> {
		let name = value
			.get(1)
			.ok_or(anyhow::anyhow!("syntax error"))?
			.as_str();
		match (name, value.get(2), value.get(3)) {
			("do", _, _) => Ok(Self::Do),
			("don't", _, _) => Ok(Self::Dont),
			("mul", Some(a), Some(b)) => {
				Ok(Self::Mul(a.as_str().parse()?, b.as_str().parse()?))
			}
			_ => Err(anyhow::anyhow!("syntax error")),
		}
	}
}

crate::gen_tests!(2024, 3, (161, 173785482), (161, 83158140));
