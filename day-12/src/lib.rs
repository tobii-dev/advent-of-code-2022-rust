use std::fmt::Display;

const DIRS: [char; 4] = ['<', '^', '>', 'v'];

#[derive(Debug)]
struct HeightMap {
	grid: Vec<Vec<isize>>,
	start: (isize, isize),
	end: (isize, isize),
	dimensions: (isize, isize),
}

impl HeightMap {
	fn from(lines: &[String]) -> Self {
		let mut start = None;
		let mut end = None;
		let mut n = None;
		let mut grid = vec![];
		for (row, y) in lines.iter().zip(0isize..) {
			let mut new_row = vec![];
			for (mut c, x) in row.chars().zip(0isize..) {
				assert!(c.is_ascii_alphabetic());
				if c == 'S' {
					if let Some(s) = start {
						let bad = (x, y);
						unreachable!("Start defined twice: at {s:#?} and at {bad:#?}");
					} else {
						c = 'a';
						start = Some((x, y));
					}
				} else if c == 'E' {
					if let Some(e) = end {
						let bad = (x, y);
						unreachable!("End defined twice: at {e:#?} and at {bad:#?}");
					} else {
						c = 'z';
						end = Some((x, y));
					}
				}
				new_row.push(c as isize - 'a' as isize);
			}
			if let Some(n) = n {
				assert!(n == new_row.len());
			} else {
				n = Some(new_row.len());
			}
			grid.push(new_row);
		}
		let x = n.unwrap().try_into().unwrap();
		let y = grid.len().try_into().unwrap();
		HeightMap {
			grid,
			start: start.unwrap(),
			end: end.unwrap(),
			dimensions: (x, y),
		}
	}

	fn is_valid(&self, x: isize, y: isize) -> bool {
		let (dim_x, dim_y) = self.dimensions;
		(0..dim_x).contains(&x) && (0..dim_y).contains(&y)
	}

	fn get(&self, x: isize, y: isize) -> Option<isize> {
		if self.is_valid(x, y) {
			Some(self.grid[y as usize][x as usize])
		} else {
			None
		}
	}

	fn get_by_tuple(&self, pos: (isize, isize)) -> Option<isize> {
		let (x, y) = pos;
		self.get(x, y)
	}

	fn moves(&self, cur_pos: (isize, isize)) -> [Option<(isize, isize)>; 4] {
		let mut movs = [None; 4];
		for (mov, dir) in movs.iter_mut().zip(DIRS) {
			*mov = self.do_move(cur_pos, dir);
		}
		movs
	}

	fn do_move(&self, cur_pos: (isize, isize), dir: char) -> Option<(isize, isize)> {
		let (x, y) = cur_pos;
		let cur_h = self.get(x, y).unwrap();
		let nxt = match dir {
			'<' => Some((x - 1, y)),
			'^' => Some((x, y - 1)),
			'>' => Some((x + 1, y)),
			'v' => Some((x, y + 1)),
			dir => {
				unreachable!("Can't move to dir '{dir}'! I can only move in: [<] [^] [>] [v]")
			}
		};
		if let Some(nxt_h) = self.get_by_tuple(nxt.unwrap()) {
			if nxt_h <= (cur_h + 1) {
				return nxt;
			}
		}
		None
	}

	//TODO: DRY ..
	fn moves_rev(&self, cur_pos: (isize, isize)) -> [Option<(isize, isize)>; 4] {
		let mut movs = [None; 4];
		for (mov, dir) in movs.iter_mut().zip(DIRS) {
			*mov = self.do_move_rev(cur_pos, dir);
		}
		movs
	}

	//TODO: DRY ..
	fn do_move_rev(&self, cur_pos: (isize, isize), dir: char) -> Option<(isize, isize)> {
		let (x, y) = cur_pos;
		let cur_h = self.get(x, y).unwrap();
		let nxt = match dir {
			'<' => Some((x - 1, y)),
			'^' => Some((x, y - 1)),
			'>' => Some((x + 1, y)),
			'v' => Some((x, y + 1)),
			dir => {
				unreachable!("Can't move to dir '{dir}'! I can only move in: [<] [^] [>] [v]")
			}
		};
		if let Some(nxt_h) = self.get_by_tuple(nxt.unwrap()) {
			if nxt_h >= (cur_h - 1) {
				return nxt;
			}
		}
		None
	}

	/// Manhattan dist (is consistent) BOOOOOOOOOOOOOOOOOOOOOOOOOOOORIIIIIIIIIIIIIIIIIIIIIIIIING
	///"The estimate is always less than or equal to
	/// the estimated distance from any neighbouring
	/// vertex to the goal,
	/// plus the cost of reaching that neighbour."
	fn h(&self, cur_pos: (isize, isize)) -> f64 {
		let (x, y) = cur_pos;
		let (fx, fy) = self.end;
		fx.abs_diff(x) as f64 + fy.abs_diff(y) as f64
	}

	/// A*
	//TODO: refactor, and try a new h() for fun
	fn search(&self) -> Vec<(isize, isize)> {
		let mut open_set: std::collections::BTreeSet<(isize, isize)> =
			std::collections::BTreeSet::new();
		let mut g_score: std::collections::HashMap<(isize, isize), f64> =
			std::collections::HashMap::new();
		let mut f_score: std::collections::HashMap<(isize, isize), f64> =
			std::collections::HashMap::new();
		let mut came_from: std::collections::HashMap<(isize, isize), (isize, isize)> =
			std::collections::HashMap::new();

		let mut cur_pos = self.start;
		open_set.insert(cur_pos);
		g_score.insert(cur_pos, 0f64);
		f_score.insert(cur_pos, self.h(cur_pos));

		let mut r = vec![];
		while !open_set.is_empty() {
			let mut min = f64::INFINITY;
			for (&pos, &f) in f_score.iter() {
				if open_set.contains(&pos) && (f < min) {
					cur_pos = pos;
					min = f;
				}
			}
			if cur_pos == self.end {
				while cur_pos != self.start {
					r.push(cur_pos);
					cur_pos = *came_from.get(&cur_pos).unwrap();
				}
				//r.push(cur_pos); // Start pos included in path?
				r.reverse();
				return r;
			}

			open_set.remove(&cur_pos);

			for mov in self.moves(cur_pos).iter().filter_map(|v| *v) {
				let score = g_score.get(&cur_pos).unwrap_or(&f64::INFINITY) + 1f64;
				if score < *g_score.get(&mov).unwrap_or(&f64::INFINITY) {
					came_from.insert(mov, cur_pos);
					g_score.insert(mov, score);
					f_score.insert(mov, score + self.h(mov));
					open_set.insert(mov);
				}
			}
		}
		r
	}

	//TODO: refactor (this is very slow)
	fn search_rev(&self) -> Vec<(isize, isize)> {
		let mut open_set: std::collections::BTreeSet<(isize, isize)> =
			std::collections::BTreeSet::new();
		let mut g_score: std::collections::HashMap<(isize, isize), f64> =
			std::collections::HashMap::new();
		let mut f_score: std::collections::HashMap<(isize, isize), f64> =
			std::collections::HashMap::new();
		let mut came_from: std::collections::HashMap<(isize, isize), (isize, isize)> =
			std::collections::HashMap::new();

		let mut cur_pos = self.end;
		open_set.insert(cur_pos);
		g_score.insert(cur_pos, 0f64);
		//f_score.insert(cur_pos, 0f64);

		let mut r = vec![];
		while !open_set.is_empty() {
			let mut min = f64::INFINITY;
			for (&pos, &f) in f_score.iter() {
				if open_set.contains(&pos) && (f < min) {
					cur_pos = pos;
					min = f;
				}
			}
			if self.get_by_tuple(cur_pos).unwrap() == 0 {
				while cur_pos != self.end {
					r.push(cur_pos);
					cur_pos = *came_from.get(&cur_pos).unwrap();
				}
				//r.push(cur_pos);
				r.reverse();
				return r;
			}

			open_set.remove(&cur_pos);

			for mov in self.moves_rev(cur_pos).iter().filter_map(|v| *v) {
				let score = g_score.get(&cur_pos).unwrap_or(&f64::INFINITY) + 1f64;
				if score < *g_score.get(&mov).unwrap_or(&f64::INFINITY) {
					came_from.insert(mov, cur_pos);
					g_score.insert(mov, score);
					f_score.insert(mov, score + 0.0);
					open_set.insert(mov);
				}
			}
		}
		r
	}
}

impl Display for HeightMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f)?;
		for (row, y) in self.grid.iter().zip(0isize..) {
			for (h, x) in row.iter().zip(0isize..) {
				if (x, y) == self.start {
					write!(f, "{:>3}", 'S')?;
				} else if (x, y) == self.end {
					write!(f, "{:>3}", 'E')?;
				} else {
					write!(f, "{h:>3}")?;
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

pub fn p1(lines: &[String]) -> usize {
	let hm: HeightMap = HeightMap::from(lines);
	println!("{hm}");
	println!("Start pos: {:#?}", hm.start);
	println!("End pos: {:#?}", hm.end);
	let path = hm.search();
	dbg!(&path);
	path.len()
}

pub fn p2(lines: &[String]) -> usize {
	let hm: HeightMap = HeightMap::from(lines);
	println!("{hm}");
	println!("Start pos: {:#?}", hm.start);
	println!("End pos: {:#?}", hm.end);
	let path = hm.search_rev();
	dbg!(&path);
	path.len()
}

#[cfg(test)]
mod tests {
	use std::io::BufRead;

	use super::*;

	#[test]
	fn example1() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		assert_eq!(result, 31);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		assert_eq!(result, 504);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines);
		assert_eq!(result, 29);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines);
		assert_eq!(result, 500);
	}
}
