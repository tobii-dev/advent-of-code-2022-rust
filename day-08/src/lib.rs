struct Pine {
	height: usize,
	visible: bool,
	score: usize,
}

impl std::fmt::Display for Pine {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let c = self.height;
		if self.visible {
			write!(f, "[{c}]")?;
		} else {
			write!(f, "{{{c}}}")?;
		}
		Ok(())
	}
}

struct Grid {
	n: usize,
	rows: Vec<Vec<Pine>>,
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		for y in 0..self.n {
			for x in 0..self.n {
				let c = self.get(x, y).unwrap();
				write!(f, "{c}")?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Grid {
	fn from(lines: &Vec<String>) -> Self {
		let (x, y) = (lines[0].len(), lines.len());
		assert!(x == y);
		let n = x;
		let mut rows = Vec::with_capacity(n);
		for line in lines {
			assert!(line.len() == n);
			let mut row = vec![];
			for c in line.chars() {
				let c: usize = c as usize - '0' as usize;
				row.push(Pine {
					height: c,
					visible: false,
					score: 0,
				});
			}
			rows.push(row);
		}
		Grid { n, rows }
	}

	fn get(&self, x: usize, y: usize) -> Option<&Pine> {
		let v = self.rows.get(y)?;
		let c = v.get(x)?;
		Some(c)
	}

	fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pine> {
		let v = self.rows.get_mut(y)?;
		let c = v.get_mut(x)?;
		Some(c)
	}

	fn calc_visible(&mut self) -> usize {
		let mut total = 0;
		for y in 0..self.n {
			for x in 0..self.n {
				if !self.is_hidden(x, y) {
					self.get_mut(x, y).unwrap().visible = true;
					total += 1;
				}
			}
		}
		total
	}

	fn is_hidden(&self, x: usize, y: usize) -> bool {
		if self.is_edge(x, y) {
			return false;
		};
		let height = self.get(x, y).unwrap().height;
		if (0..x)
			.find(|&x| self.get(x, y).unwrap().height >= height)
			.is_none()
		{
			return false;
		}
		if (0..y)
			.find(|&y| self.get(x, y).unwrap().height >= height)
			.is_none()
		{
			return false;
		}
		if (x + 1..self.n)
			.find(|&x| self.get(x, y).unwrap().height >= height)
			.is_none()
		{
			return false;
		}
		if (y + 1..self.n)
			.find(|&y| self.get(x, y).unwrap().height >= height)
			.is_none()
		{
			return false;
		}
		true
	}

	fn is_edge(&self, x: usize, y: usize) -> bool {
		(x == 0) || (y == 0) || (x == self.n - 1) || (y == self.n - 1)
	}

	fn calc_top_scenic_score(&mut self) -> usize {
		let mut best = 0;
		for y in 1..self.n - 1 {
			for x in 1..self.n - 1 {
				let s = self.score(x, y);
				self.get_mut(x, y).unwrap().score = s;
				if s > best {
					best = s;
				}
			}
		}
		best
	}

	fn score(&self, x: usize, y: usize) -> usize {
		let height = self.get(x, y).unwrap().height;
		let xn = (0..x)
			.rev()
			.find(|&x| self.is_edge(x, y) || self.get(x, y).unwrap().height >= height)
			.unwrap();
		let yn = (0..y)
			.rev()
			.find(|&y| self.is_edge(x, y) || self.get(x, y).unwrap().height >= height)
			.unwrap();
		let xp = (x + 1..self.n)
			.find(|&x| self.is_edge(x, y) || self.get(x, y).unwrap().height >= height)
			.unwrap();
		let yp = (y + 1..self.n)
			.find(|&y| self.is_edge(x, y) || self.get(x, y).unwrap().height >= height)
			.unwrap();
		(x - xn) * (y - yn) * (xp - x) * (yp - y)
	}
}

pub fn p1(lines: &Vec<String>) -> usize {
	let mut grid = Grid::from(&lines);
	let vis_count = grid.calc_visible();
	println!("{grid}");
	vis_count
}

pub fn p2(lines: &Vec<String>) -> usize {
	let mut grid = Grid::from(&lines);
	let top = grid.calc_top_scenic_score();
	println!("{grid}");
	top
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
		assert_eq!(r, 21);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 1825);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 8);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 235200);
	}
}
