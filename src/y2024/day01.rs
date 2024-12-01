use std::collections::HashMap;

use crate::Result;
use anyhow::anyhow;

pub fn part2(input: &str) -> Result<i32> {
	let (mut left, mut right) = parse_list(input)?;
	left.sort();
	right.sort();
	let mut counts = HashMap::<i32, i32>::new();
	for r in &right {
		if let Some(count) = counts.get_mut(r) {
			*count += 1;
		} else {
			counts.insert(*r, 1);
		}
	}
	let res: i32 = left.iter().map(|l| *l * *counts.get(l).unwrap_or(&0)).sum();
	Ok(res)
}

pub fn part1(input: &str) -> Result<i32> {
	let (mut left, mut right) = parse_list(input)?;
	left.sort();
	right.sort();
	let res: i32 = left
		.iter()
		.zip(right.iter())
		.map(|(l, r)| (*l - *r).abs())
		.sum();
	Ok(res)
}

pub fn parse_list(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
	let mut left = Vec::<i32>::new();
	let mut right = Vec::<i32>::new();
	for line in input.lines() {
		if line == "" {
			continue;
		}
		let (l, r) = line
			.split_once("   ")
			.ok_or(anyhow!("malformed line: {}", line))?;
		left.push(l.parse()?);
		right.push(r.parse()?);
	}
	Ok((left, right))
}

crate::gen_tests!(2024, 1, (11, 1530215), (31, 26800609));
