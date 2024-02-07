//#![allow(unused)]

#[derive(Debug)]
struct Point {
	x: usize,
	y: usize,
}

#[derive(Debug)]
struct Path {
	points: Vec<Point>,
	max: Point,
	min: Point,
}

#[derive(Debug)]
enum Cell {
	/// Rock
	R,
	/// Air
	A,
	/// Sand
	S,
	/// Pour source
	P,
}

#[derive(Debug)]
enum Landing {
	/// Sand falls into Abyss
	Abyss,
	/// Sand falls into air
	Airborne(Point),
	/// Sand is at rest
	Rest(Point),
}

impl std::fmt::Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let c = match self {
			Cell::R => "[#]",
			Cell::A => " · ",
			Cell::S => " O ",
			Cell::P => "|+|",
		};
		write!(f, "{c}")
	}
}

#[derive(Debug)]
struct Grid {
	max: Point,
	min: Point,
	rows: Vec<Vec<Cell>>,
	src: Point,
	rest: usize,
}

impl Point {
	/// `498,4` -> Point { x: 498, y: 4 }
	fn from_str(s: &str) -> Self {
		let mut s = s.trim().split(",");
		let (x, y) = (s.next().unwrap(), s.next().unwrap());
		let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
		Self { x, y }
	}

	fn new(coords: (usize, usize)) -> Self {
		Self {
			x: coords.0,
			y: coords.1,
		}
	}
	fn new_max() -> Self {
		Self::new((usize::MAX, usize::MAX))
	}

	fn new_min() -> Self {
		Self::new((usize::MIN, usize::MIN))
	}

	fn grow(&mut self, other: &Self) {
		(self.x, self.y) = (self.x.max(other.x), self.y.max(other.y));
	}

	fn shrink(&mut self, other: &Self) {
		(self.x, self.y) = (self.x.min(other.x), self.y.min(other.y));
	}
}

impl Path {
	fn from_line(line: &str) -> Self {
		let mut points: Vec<Point> = vec![];
		let (mut max, mut min) = (Point::new_min(), Point::new_max());
		for s in line.split(" -> ") {
			let point = Point::from_str(s);
			max.grow(&point);
			min.shrink(&point);
			dbg!(&point);
			points.push(point);
		}
		Self { points, max, min }
	}
}

impl Grid {
	fn get(&self, x: usize, y: usize) -> Option<&Cell> {
		self.rows.get(y)?.get(x)
	}

	fn from_lines(lines: &[String], source_coords: (usize, usize)) -> Self {
		let mut paths: Vec<Path> = vec![];
		let (mut max, mut min) = (Point::new_min(), Point::new_max());
		for line in lines {
			let path = Path::from_line(line);
			max.grow(&path.max);
			min.shrink(&path.min);
			paths.push(path);
		}
		let (w, h) = (max.x + 1, max.y + 1);
		let mut rows = Vec::with_capacity(h);
		for _y in 0..=h {
			let mut row: Vec<Cell> = Vec::with_capacity(w);
			for _x in 0..=w {
				row.push(Cell::A);
			}
			rows.push(row);
		}
		for path in paths {
			for pc in path.points.windows(2) {
				let (p0, p1) = (&pc[0], &pc[1]);
				if p0.x == p1.x {
					let x = p0.x;
					let (y0, y1) = (p0.y.min(p1.y), p0.y.max(p1.y));
					for y in y0..=y1 {
						rows[y][x] = Cell::R;
					}
				} else if p0.y == p1.y {
					let y = p0.y;
					let (x0, x1) = (p0.x.min(p1.x), p0.x.max(p1.x));
					for x in x0..=x1 {
						rows[y][x] = Cell::R;
					}
				} else {
					unreachable!("Path movement must be strictly vertical/horitzontal");
				}
			}
		}
		let src = Point::new(source_coords);
		rows[src.y][src.x] = Cell::P;

		let rest = 0;

		Self {
			max,
			min,
			rows,
			src,
			rest,
		}
	}

	fn pour(&mut self) -> bool {
		let mut current_pos = Point::new((self.src.x, self.src.y));
		loop {
			let fall = self.fall(&current_pos);
			match fall {
				Landing::Abyss => break true,
				Landing::Airborne(pos) => {
					current_pos = pos;
				}
				Landing::Rest(pos) => {
					let (x, y) = (pos.x, pos.y);
					self.rows[y][x] = Cell::S;
					println!("{self}");
					self.rest += 1;
					break false;
				}
			}
		}
	}

	fn fall(&self, current_pos: &Point) -> Landing {
		// · + ·
		// · o ·
		let pos_down = Point::new((current_pos.x, current_pos.y + 1));
		// · + ·
		// o · ·
		let pos_down_l = Point::new((pos_down.x - 1, pos_down.y));
		// · + ·
		// · · o
		let pos_down_r = Point::new((pos_down.x + 1, pos_down.y));

		let candidates = &[pos_down, pos_down_l, pos_down_r];
		for candidate in candidates {
			let Some(cell) = self.get(candidate.x, candidate.y) else {
				return Landing::Abyss;
			};
			if matches!(*cell, Cell::A) {
				return Landing::Airborne(Point::new((candidate.x, candidate.y)));
			}
		}
		return Landing::Rest(Point::new((current_pos.x, current_pos.y)));
	}
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let (max_w, max_h) = (self.max.x + 1, self.max.y + 1);
		let (min_w, min_h) = (self.min.x - 1, 0);
		for y in min_h..=max_h {
			for x in min_w..=max_w {
				if let Some(cell) = &self.get(x, y) {
					write!(f, "{cell}")?;
				} else {
					write!(f, "(?)")?;
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

pub fn p1(lines: &Vec<String>) -> usize {
	const SOURCE_COORDS: (usize, usize) = (500, 0);
	let mut grid = Grid::from_lines(lines, SOURCE_COORDS);
	println!("{grid}");
	while !grid.pour() {}
	grid.rest
}

pub fn p2(lines: &Vec<String>) -> usize {
	p1(lines)
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
		assert_eq!(result, 24);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		assert_eq!(result, 901);
	}

	#[ignore]
	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines);
		assert_eq!(result, 0);
	}

	#[ignore]
	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines);
		assert_eq!(result, 0);
	}
}
