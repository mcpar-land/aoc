use crate::Result;

pub fn part1(input: &str) -> Result<i32> {
	let crossword = Crossword::new(input.trim());

	Ok(crossword.find("XMAS"))
}

struct Crossword {
	values: Vec<Vec<char>>,
	width: i32,
	height: i32,
}

impl Crossword {
	pub fn new(input: &str) -> Self {
		let values: Vec<Vec<char>> = input
			.lines()
			.map(|line| line.chars().collect::<Vec<char>>())
			.collect();
		let width = values.iter().map(Vec::len).max().unwrap() as i32;
		let height = values.len() as i32;
		Self {
			values,
			width,
			height,
		}
	}

	pub fn find(&self, target: &str) -> i32 {
		let first_char = target.chars().nth(0).unwrap();
		let second_char = target.chars().nth(1).unwrap();
		let rest = &target[2..];

		let mut potential_matches = Vec::<((i32, i32), (i32, i32))>::new();

		let first_char_locations = self.locations(first_char);
		for l in first_char_locations {
			for adj in self.adjacent_coords(l) {
				if self.get(adj) == second_char {
					potential_matches.push((l, adj));
				}
			}
		}

		let mut n_matches = 0;

		for (a, b) in potential_matches {
			let direction = (b.0 - a.0, b.1 - a.1);
			if self._check_match(b, direction, rest) {
				n_matches += 1;
			}
		}

		n_matches
	}

	pub fn _check_match(
		&self,
		src: (i32, i32),
		direction: (i32, i32),
		rest: &str,
	) -> bool {
		let target = (src.0 + direction.0, src.1 + direction.1);
		let check_for = rest.chars().nth(0).unwrap();

		let (tx, ty) = target;
		if tx < 0 || ty < 0 || tx >= self.width || ty >= self.height {
			return false;
		}

		if self.get(target) != check_for {
			return false;
		}

		if rest.len() == 1 {
			return true;
		}

		self._check_match(target, direction, &rest[1..])
	}

	pub fn locations(&self, target: char) -> Vec<(i32, i32)> {
		let mut res = Vec::new();
		for (y, row) in self.values.iter().enumerate() {
			for (x, c) in row.iter().enumerate() {
				if *c == target {
					res.push((x as i32, y as i32));
				}
			}
		}
		res
	}

	pub fn get(&self, (x, y): (i32, i32)) -> char {
		self.values[y as usize][x as usize]
	}

	pub fn adjacent_coords(&self, (tx, ty): (i32, i32)) -> Vec<(i32, i32)> {
		(-1..=1)
			.into_iter()
			.flat_map(|x| (-1..=1).map(move |y| (x, y)))
			.map(|(x, y)| (x + tx, y + ty))
			.filter(|(x, y)| {
				*x >= 0 && *y >= 0 && *x < self.width && *y < self.height
			})
			.collect()
	}
}

crate::gen_tests!(2024, 4, (18, 2401));
