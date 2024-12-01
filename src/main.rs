pub mod y2024;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
pub type DayFn = fn(&str) -> Result<i32>;
pub type DayFnPair = (DayFn, Option<DayFn>);

fn exec_day_fn(
	f: DayFn,
	year: u16,
	day: u8,
	is_example: bool,
	known_solution: Option<i32>,
) -> Result<i32> {
	let input_path = match is_example {
		true => format!("inputs/{}/day{:02}_example.txt", year, day),
		false => format!("inputs/{}/day{:02}.txt", year, day),
	};
	let input_body = std::fs::read_to_string(&input_path)
		.expect(&format!("failed to read {}", input_path));
	exec_single_fn(f, &input_body, known_solution)
}

fn exec_single_fn(
	f: DayFn,
	input: &str,
	known_solution: Option<i32>,
) -> Result<i32> {
	let res = f(input)?;
	match known_solution {
		Some(known_solution) if known_solution != res => Err(anyhow::anyhow!(
			"{} does not match known solution {}",
			res,
			known_solution
		)),
		Some(_) => Ok(res),
		None => Err(anyhow::anyhow!("got {} (no known solution yet)", res)),
	}
}

#[macro_export]
macro_rules! gen_tests {
	($year:expr, $day:expr, ($example_solution1:expr, $known_solution1:expr), ($example_solution2:expr, $known_solution2:expr)) => {
		#[cfg(test)]
		mod tests {
			use super::*;
			crate::gen_tests!(_test_both $year, $day, 1, $example_solution1, Some($known_solution1));
			crate::gen_tests!(_test_both $year, $day, 2, $example_solution2, Some($known_solution2));
		}
	};
	($year:expr, $day:expr, ($example_solution1:expr, $known_solution1:expr), ($example_solution2:expr, ?)) => {
		#[cfg(test)]
		mod tests {
			use super::*;
			crate::gen_tests!(_test_both $year, $day, 1, $example_solution1, Some($known_solution1));
			crate::gen_tests!(_test_both $year, $day, 2, $example_solution2, None);
		}
	};
	($year:expr, $day:expr, ($example_solution1:expr, $known_solution1:expr)) => {
		#[cfg(test)]
		mod tests {
			use super::*;
			crate::gen_tests!(_test_both $year, $day, 1, $example_solution1, Some($known_solution1));
		}
	};
	($year:expr, $day:expr, ($example_solution1:expr, ?)) => {
		#[cfg(test)]
		mod tests {
			use super::*;
			crate::gen_tests!(_test_both $year, $day, 1, $example_solution1, None);
		}
	};
	(_test $fxn:ident, $test_name:ident; $year:expr, $day:expr, $is_example:expr, $known_solution:expr) => {
		#[test]
		fn $test_name() {
			let _ = env_logger::builder().is_test(true).try_init();
			let res = crate::exec_day_fn(
				$fxn,
				stringify!($year).parse().unwrap(),
				stringify!($day).parse().unwrap(),
				match stringify!($is_example) {
					"true" => true,
					"false" => false,
					_ => unreachable!(),
				},
				$known_solution
			);
			match res {
				Ok(res) => {
					log::info!("y {} / d {} / p {} ::: solution =  {}", stringify!($year), stringify!($day), stringify!($part), res);
				},
				Err(err) => {
					log::error!("failed: {:?}", &err);
					panic!();
				}
			}
		}
	};
	(_test_both $year:expr, $day:expr, $part:tt, $example_solution:expr, $known_solution:expr) => {
		paste::paste! {
			crate::gen_tests!(_test [<part $part>], [<part_ $part _example>]; $year, $day, true, Some($example_solution));
			crate::gen_tests!(_test [<part $part>], [<part_ $part>]; $year, $day, false, $known_solution);
		}
	};
}
