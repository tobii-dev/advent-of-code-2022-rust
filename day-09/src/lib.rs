#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
	x: isize,
	y: isize,
}

impl std::fmt::Display for Pos {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let (x, y) = (self.x, self.y);
		write!(f, "[{x}, {y}]")?;
		Ok(())
	}
}

struct Grid {
	x_min: isize,
	y_min: isize,

	x_max: isize,
	y_max: isize,

	head: Pos,
	tail: Pos,
	tail_visits: Vec<Pos>,
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		for y in self.y_min..=self.y_max {
			for x in self.x_min..=self.x_max {
				let is_visit = self.tail_visits.iter().any(|p| (p.x == x) && (p.y == y));
				let is_tail = (self.tail.x == x) && (self.tail.y == y);
				let is_head = (self.head.x == x) && (self.head.y == y);
				if is_visit {
					write!(f, "{{")?;
				} else {
					write!(f, "[")?;
				}
				if is_head && is_tail {
					write!(f, "{:>2}", "X")?;
				} else if is_head {
					write!(f, "{:>2}", "H")?;
				} else if is_tail {
					write!(f, "{:>2}", "T")?;
				} else {
					write!(f, "{:>2}", "")?;
				}
				if is_visit {
					write!(f, "}}")?;
				} else {
					write!(f, "]")?;
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Grid {
	fn new() -> Self {
		Grid {
			x_min: 0,
			y_min: 0,
			x_max: 0,
			y_max: 0,
			head: Pos { x: 0, y: 0 },
			tail: Pos { x: 0, y: 0 },
			tail_visits: vec![Pos { x: 0, y: 0 }],
		}
	}

	fn from(lines: &Vec<String>) -> Self {
		let mut grid = Grid::new();
		for line in lines {
			let mut words = line.split_whitespace();
			let (dir, count) = (
				words.next().unwrap(),
				words.next().unwrap().parse::<usize>().unwrap(),
			);
			match dir {
				"L" => {
					grid.update_head(count, -1, 0);
				}
				"U" => {
					grid.update_head(count, 0, -1);
				}
				"R" => {
					grid.update_head(count, 1, 0);
				}
				"D" => {
					grid.update_head(count, 0, 1);
				}
				_ => unreachable!(),
			}
		}
		grid
	}

	fn update_head(&mut self, count: usize, dx: isize, dy: isize) -> bool {
		let mut update = false;
		for _ in 0..count {
			let prev = self.head;
			self.head.x += dx;
			self.head.y += dy;
			if self.head.x < self.x_min {
				self.x_min = self.head.x;
			}
			if self.head.y < self.y_min {
				self.y_min = self.head.y;
			}
			if self.head.x > self.x_max {
				self.x_max = self.head.x;
			}
			if self.head.y > self.y_max {
				self.y_max = self.head.y;
			}
			if self.update_tail(&prev) {
				update = self.update_visits();
			}
		}
		update
	}

	fn update_tail(&mut self, prev: &Pos) -> bool {
		if (self.head.x - 1..=self.head.x + 1).contains(&self.tail.x)
			&& (self.head.y - 1..=self.head.y + 1).contains(&self.tail.y)
		{
			return false;
		}
		self.tail.x = prev.x;
		self.tail.y = prev.y;
		true
	}

	fn update_visits(&mut self) -> bool {
		if !self.tail_visits.iter().any(|p| p == &self.tail) {
			self.tail_visits.push(Pos {
				x: self.tail.x,
				y: self.tail.y,
			});
		}
		false
	}
}

pub fn p1(lines: &Vec<String>) -> usize {
	let grid = Grid::from(lines);
	grid.tail_visits.len()
}

// pub fn p2(lines: &Vec<String>) -> usize {
// 	todo!()
// }

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::BufRead;

	#[test]
	fn example1() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 13);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 5981);
	}

	// #[test]
	// fn example2() {
	// 	let fd = std::fs::File::open("example.txt").unwrap();
	// 	let mut lines: Vec<String> = std::io::BufReader::new(fd)
	// 		.lines()
	// 		.map(|l| l.unwrap())
	// 		.collect();
	// 	let r = p2(&mut lines);
	// 	assert_eq!(r, 8);
	// }

	// #[test]
	// fn part2() {
	// 	let fd = std::fs::File::open("input.txt").unwrap();
	// 	let mut lines: Vec<String> = std::io::BufReader::new(fd)
	// 		.lines()
	// 		.map(|l| l.unwrap())
	// 		.collect();
	// 	let r = p2(&mut lines);
	// 	assert_eq!(r, 235200);
	// }
}
