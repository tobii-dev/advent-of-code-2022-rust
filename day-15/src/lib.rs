use std::{isize, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
	pub x: isize,
	pub y: isize,
}

#[derive(Debug)]
struct Beacon {
	pos: Point,
}

#[derive(Debug)]
struct Sensor {
	pos: Point,
	beacon: Beacon,
	radius: usize,
}

#[derive(Debug)]
struct Cave {
	min: Point, // left up
	max: Point, // right down
	sensors: Vec<Sensor>,
}

impl Point {
	fn from_str(s: &str) -> Self {
		let s = s.trim_start_matches("x=");
		let mut s = s.split(',');
		let (x, y) = (s.next().unwrap(), s.next().unwrap());
		let x = x.parse().unwrap();
		let y = y.trim_start_matches(" y=").parse().unwrap();
		Self { x, y }
	}
	/// Manhattan dist to other
	fn dist(&self, other: &Self) -> usize {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}

	fn new_max() -> Self {
		Self {
			x: isize::MAX,
			y: isize::MAX,
		}
	}

	fn new_min() -> Self {
		Self {
			x: isize::MIN,
			y: isize::MIN,
		}
	}

	fn grow(&mut self, other: &Self) {
		(self.x, self.y) = (self.x.max(other.x), self.y.max(other.y));
	}

	fn shrink(&mut self, other: &Self) {
		(self.x, self.y) = (self.x.min(other.x), self.y.min(other.y));
	}

	fn is_in_bounds(&self, min_bound: &Point, max_bound: &Point) -> bool {
		let min_ok = (min_bound.x <= self.x) && (min_bound.y <= self.y);
		let max_ok = (self.x <= max_bound.x) && (self.y <= max_bound.y);
		min_ok && max_ok
	}
}

impl Beacon {
	fn from_str(s: &str) -> Self {
		let pos = Point::from_str(s);
		Self { pos }
	}
}

impl Sensor {
	fn from_str(s: &str) -> Self {
		let s = s.trim_start_matches("Sensor at ");
		let mut s = s.split(": closest beacon is at ");
		let (pos, beacon_pos) = (s.next().unwrap(), s.next().unwrap());
		let pos = Point::from_str(pos);
		let beacon = Beacon::from_str(beacon_pos);
		let radius = pos.dist(&beacon.pos);
		Self {
			pos,
			beacon,
			radius,
		}
	}

	fn bounds(&self) -> (Point, Point) {
		let (mut min, mut max) = (self.pos, self.pos);
		let radius: isize = self.radius.try_into().unwrap();
		min.x = min.x.saturating_sub(radius);
		min.y = min.y.saturating_sub(radius);
		max.x = max.x.saturating_add(radius);
		max.y = max.y.saturating_add(radius);
		(min, max)
	}

	fn get_border_path(&self) -> Vec<Point> {
		let mut border = vec![];
		let radius: isize = self.radius.try_into().unwrap();
		let (x, y) = (self.pos.x, self.pos.y);
		let dx_iter = 1..=radius;
		let dy_iter = dx_iter.clone().rev();
		for (dx, dy) in dx_iter.zip(dy_iter) {
			border.push(Point {
				x: x - dx,
				y: y - dy,
			});
			border.push(Point {
				x: x + dx,
				y: y - dy,
			});
			border.push(Point {
				x: x - dx,
				y: y + dy,
			});
			border.push(Point {
				x: x + dx,
				y: y + dy,
			});
		}
		let radius = radius + 1;
		border.push(Point { x, y: y + radius });
		border.push(Point { x, y: y - radius });
		border.push(Point { x: x - radius, y });
		border.push(Point { x: x + radius, y });
		border
	}
}

impl Cave {
	fn from_lines(lines: &[String]) -> Self {
		let (mut min, mut max) = (Point::new_max(), Point::new_min());
		let mut sensors = vec![];
		for line in lines {
			let sensor = Sensor::from_str(line);
			let (bounds_min, bounds_max) = sensor.bounds();
			min.shrink(&bounds_min);
			max.grow(&bounds_max);
			sensors.push(sensor);
		}
		Self { min, max, sensors }
	}

	fn get_clears_on_row(&self, y: isize) -> usize {
		let x0 = self.min.x.saturating_sub(1);
		let x1 = self.max.x.saturating_add(1);
		let mut count = 0;
		for x in x0..=x1 {
			for sensor in &self.sensors {
				let p = Point { x, y };
				let is_beacon = p == sensor.beacon.pos;
				let is_in_sensor_range = p.dist(&sensor.pos) <= sensor.radius;
				if !is_beacon && is_in_sensor_range {
					count += 1;
					break;
				}
			}
		}
		count
	}

	fn get_possible_beacons_in_bounds(&self, min_bound: Point, max_bound: Point) -> Vec<Point> {
		let mut candidates = vec![];
		for sensor in &self.sensors {
			for point in sensor.get_border_path() {
				if point.is_in_bounds(&min_bound, &max_bound) {
					candidates.push(point);
				}
			}
		}
		candidates
			.iter()
			.filter(|candidate| {
				for sensor in &self.sensors {
					if candidate.dist(&sensor.pos) <= sensor.radius {
						return false;
					}
				}
				true
			})
			.copied()
			.collect()
	}
}

pub fn p1(lines: &[String], y: isize) -> usize {
	let cave = Cave::from_lines(lines);
	cave.get_clears_on_row(y)
}

pub fn p2(lines: &[String], min: isize, max: isize, freq: isize) -> usize {
	let cave = Cave::from_lines(lines);
	let (min_bound, max_bound) = (Point { x: min, y: min }, Point { x: max, y: max });
	let mut candidates = cave.get_possible_beacons_in_bounds(min_bound, max_bound);
	candidates.dedup();
	assert!(candidates.len() == 1);
	let the_chosen_wan_kenobi = candidates[0];
	(the_chosen_wan_kenobi.x * freq + the_chosen_wan_kenobi.y)
		.try_into()
		.unwrap()
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
		let result = p1(&lines, 10);
		assert_eq!(result, 26);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines, 2_000_000);
		assert_eq!(result, 4861076);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines, 0, 20, 4_000_000);
		assert_eq!(result, 56000011);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines, 0, 4_000_000, 4_000_000);
		assert_eq!(result, 10649103160102);
	}
}
