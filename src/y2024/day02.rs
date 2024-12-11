use std::num::ParseIntError;

use crate::Result;
use anyhow::anyhow;

pub fn part1(input: &str) -> Result<i32> {
	let report = parse_reports(input)?;

	let n_safe = report
		.into_iter()
		.filter(|r| unsafe_index(r).is_none())
		.count();
	Ok(n_safe as i32)
}

fn unsafe_index(report: &Vec<i32>) -> Option<usize> {
	let is_decreasing = report[0] > report[1];
	for i in 1..report.len() {
		let prev = &report[i - 1];
		let current = &report[i];
		match (is_decreasing, prev > current) {
			(true, true) => {}
			(true, false) => return Some(i),
			(false, true) => return Some(i),
			(false, false) => {}
		};
		let diff = (prev - current).abs();
		if diff < 1 || diff > 3 {
			return Some(i);
		}
	}
	return None;
}

fn parse_reports(
	input: &str,
) -> std::result::Result<Vec<Vec<i32>>, ParseIntError> {
	input
		.lines()
		.filter(|l| *l != "")
		.map(|l| {
			l.split_whitespace()
				.map(|s| s.parse())
				.collect::<std::result::Result<Vec<i32>, ParseIntError>>()
		})
		.collect::<std::result::Result<Vec<Vec<i32>>, ParseIntError>>()
}

crate::gen_tests!(2024, 2, (2, 526));
