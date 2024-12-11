use crate::Result;
use regex::{Captures, Regex};

pub fn part1(input: &str) -> Result<i32> {
	let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

	let res = re
		.captures_iter(input)
		.map(Mul::try_from)
		.collect::<Result<Vec<Mul>>>()?
		.into_iter()
		.map(Mul::run)
		.sum::<i32>();

	Ok(res)
}

struct Mul(i32, i32);

impl Mul {
	pub fn run(self) -> i32 {
		self.0 * self.1
	}
}

impl<'e> TryFrom<Captures<'e>> for Mul {
	type Error = anyhow::Error;

	fn try_from(
		value: Captures<'e>,
	) -> std::prelude::v1::Result<Self, Self::Error> {
		let (_, [a, b]) = value.extract();
		Ok(Mul(a.parse()?, b.parse()?))
	}
}

crate::gen_tests!(2024, 3, (161, 173785482));
