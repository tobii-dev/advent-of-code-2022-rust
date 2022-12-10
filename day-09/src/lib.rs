#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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

impl Pos {
	fn follow(&mut self, next: &Pos) -> bool {
		if (next.x - 1..=next.x + 1).contains(&self.x)
			&& (next.y - 1..=next.y + 1).contains(&self.y)
		{
			return false; // no need to move
		}
		let (dx, dy) = (next.x - self.x, next.y - self.y);
		self.x += dx.signum();
		self.y += dy.signum();
		true
	}
}

struct Grid {
	x_min: isize,
	y_min: isize,

	x_max: isize,
	y_max: isize,

	head: Pos,
	tails: Vec<Pos>,
	visits: std::collections::HashSet<Pos>,
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		for y in self.y_min..=self.y_max {
			for x in self.x_min..=self.x_max {
				let pos = Pos { x, y };
				let is_visit = self.visited(&pos);
				let is_tail = self.tails.iter().any(|p| (p.x == x) && (p.y == y));
				let is_head = self.head == pos;
				let mut c = " ";
				if is_visit {
					write!(f, "{{")?;
					c = "}";
				} else {
					write!(f, " ")?;
				}
				if is_head && is_tail {
					write!(f, "{:>2}{c}", "X")?;
				} else if is_head {
					write!(f, "{:>2}{c}", "H")?;
				} else if is_tail {
					let v = self
						.tails
						.iter()
						.position(|p| p.x == x && p.y == y)
						.unwrap();
					write!(f, "{v:>2}{c}")?;
				} else {
					write!(f, "{:>2}{c}", "·")?;
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Grid {
	fn new(c: usize) -> Self {
		let z = Pos { x: 0, y: 0 };
		Grid {
			x_min: 0,
			y_min: 0,
			x_max: 0,
			y_max: 0,
			head: z,
			tails: vec![z; c],
			visits: std::collections::HashSet::from([z]),
		}
	}

	fn from(lines: &Vec<String>, tails: usize) -> Self {
		let mut grid = Grid::new(tails);
		for line in lines {
			let mut words = line.split_whitespace();
			let (dir, count) = (
				words.next().unwrap(),
				words.next().unwrap().parse::<usize>().unwrap(),
			);
			match dir {
				"L" => grid.update_head(count, -1, 0),
				"U" => grid.update_head(count, 0, -1),
				"R" => grid.update_head(count, 1, 0),
				"D" => grid.update_head(count, 0, 1),
				_ => unreachable!(),
			};
		}
		grid
	}

	fn update_head(&mut self, count: usize, dx: isize, dy: isize) {
		for _ in 0..count {
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
			let mut next = self.head;
			for tail in self.tails.iter_mut() {
				tail.follow(&next);
				next = *tail;
			}
			self.update_visits();
		}
	}

	fn update_visits(&mut self) -> bool {
		let tail = self.tails.last().unwrap();
		self.visits.insert(*tail)
	}

	fn visited(&self, p: &Pos) -> bool {
		self.visits.contains(p)
	}
}

pub fn p1(lines: &Vec<String>) -> usize {
	let grid = Grid::from(lines, 1);
	println!(
		"sizes: {}x{}",
		grid.x_max - grid.x_min,
		grid.y_max - grid.y_min
	);
	grid.visits.len()
}

pub fn p2(lines: &Vec<String>) -> usize {
	let grid = Grid::from(lines, 9);
	println!(
		"sizes: {}x{}",
		grid.x_max - grid.x_min,
		grid.y_max - grid.y_min
	);
	grid.visits.len()
}

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

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example2.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 36);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 2352);
	}
}
