use std::collections::HashMap;

use anyhow::anyhow;

use crate::Result;

crate::gen_tests!(2024, 5, (143, 5955), (123, ?));

pub fn part1(input: &str) -> Result<i32> {
	let dataset = Dataset::parse(input)?;

	let mut total: i32 = 0;
	for update in &dataset.updates {
		if update.is_valid(&dataset.pairs) {
			total += update.middle_number();
		}
	}

	Ok(total)
}

pub fn part2(input: &str) -> Result<i32> {
	let dataset = Dataset::parse(input)?;

	let mut total: i32 = 0;
	for update in &dataset.updates {
		if !update.is_valid(&dataset.pairs) {
			total += update.make_valid(&dataset.pairs).middle_number();
		}
	}
	Ok(total)
}

struct Dataset {
	pub pairs: Vec<Pair>,
	pub updates: Vec<Update>,
}

impl Dataset {
	pub fn parse(input: &str) -> Result<Self> {
		let (pairs, updates) = input
			.split_once("\n\n")
			.ok_or_else(|| anyhow!("missing double newline"))?;

		let pairs = pairs
			.trim()
			.split('\n')
			.map(Pair::parse)
			.collect::<Result<Vec<Pair>>>()?;

		let updates = updates
			.trim()
			.split('\n')
			.map(Update::parse)
			.collect::<Result<Vec<Update>>>()?;

		Ok(Self { pairs, updates })
	}
}

struct Pair(pub i32, pub i32);

impl Pair {
	fn parse(input: &str) -> Result<Pair> {
		let (a, b) = input
			.split_once('|')
			.ok_or_else(|| anyhow!("invalid pair {}", input))?;
		let a: i32 = a.parse()?;
		let b: i32 = b.parse()?;
		Ok(Pair(a, b))
	}
}

#[derive(Clone)]
struct Update(Vec<i32>);

impl Update {
	fn parse(input: &str) -> Result<Self> {
		input
			.split(',')
			.map(|v| v.parse().map_err(|e| anyhow!("{}", e)))
			.collect::<Result<Vec<i32>>>()
			.map(|l| Self(l))
	}

	fn middle_number(&self) -> i32 {
		let middle_i = self.0.len() / 2;
		self.0[middle_i]
	}

	fn is_valid(&self, rules: &[Pair]) -> bool {
		let index_map = self
			.0
			.iter()
			.enumerate()
			.map(|(i, val)| (*val, i))
			.collect::<HashMap<i32, usize>>();

		for Pair(a, b) in rules {
			match (index_map.get(a), index_map.get(b)) {
				(Some(a), Some(b)) if a > b => {
					return false;
				}
				_ => {}
			}
		}

		return true;
	}

	fn make_valid(&self, rules: &[Pair]) -> Self {
		if rules.len() == 0 {
			return self.clone();
		}

		let mut res = self.0.clone();
		let index_map = self
			.0
			.iter()
			.enumerate()
			.map(|(i, val)| (*val, i))
			.collect::<HashMap<i32, usize>>();

		for Pair(a, b) in rules {
			match (index_map.get(a), index_map.get(b)) {
				(Some(a), Some(b)) if a >= b => {
					res.swap(*a, *b);
				}
				_ => {}
			}
		}

		Self(res).make_valid(&rules[1..])
	}
}
