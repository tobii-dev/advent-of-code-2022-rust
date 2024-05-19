#![allow(unused)]
//#![feature(ascii_char)]
//#![feature(ascii_char_variants)]

use std::{ops::RangeInclusive, usize};

#[derive(Debug)]
enum Op {
	Open { dst: u16 },
	Tunnel { dst: u16 },
}

#[derive(Debug)]
struct Valve {
	flow: usize,
	label: u16,
	cnx: Vec<u16>,
}

#[derive(Debug)]
struct Cave {
	valves: Vec<Option<Valve>>,
}

impl Valve {
	// "Valve HH has flow rate=22; tunnel leads to valve GG"
	// "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
	fn from_line(line: &str) -> Self {
		let line = line.trim_start_matches("Valve ");
		let mut s = line.split(" has flow rate=");
		let label = s.next().unwrap(); // "HH"
		let label: u16 = label_to_u16(label);

		let mut s = s.next().unwrap().split(';');
		let flow = s.next().unwrap(); // "22";
		let flow: usize = flow.parse().unwrap();

		let mut s = s
			.next()
			.unwrap()
			.trim_start_matches(|c: char| !c.is_uppercase())
			.split(", ");
		let mut cnx = vec![];
		for cnx_label in s {
			let id = label_to_u16(cnx_label);
			cnx.push(id);
		}
		cnx.sort();

		Self { flow, label, cnx }
	}
}

impl Cave {
	fn from_lines(lines: &[String]) -> Self {
		let mut valves = {
			let max_index = label_to_u16("ZZ") as usize + 1;
			let mut v: Vec<Option<Valve>> = vec![];
			v.resize_with(max_index, Default::default);
			v
		};
		for line in lines {
			let valve = Valve::from_line(line);
			let id: usize = valve.label.into();
			valves.get_mut(id).unwrap().replace(valve);
		}
		assert!(valves[0].is_some()); // we start at valve "AA"
		Self { valves }
	}

	/// Make sure all valves cnx exist, and that there are no "dead end" valves
	fn verify(&self) -> bool {
		for valve in self.valves.iter().filter_map(|v| v.as_ref()) {
			if valve.cnx.is_empty() {
				// dead end¿¿??
				return false;
			}
			if valve
				.cnx
				.iter()
				.any(|cnx| self.valves.get(*cnx as usize).is_none())
			{
				// tunnel leads to valvet that doesn't exist
				return false;
			}
		}
		true
	}

	fn get_valve(&self, src: u16) -> &Valve {
		self.valves.get(src as usize).unwrap().as_ref().unwrap()
	}

	fn search(&self, start: u16) -> usize {
		const MAX_T: usize = 30;
		let mut t = MAX_T;
		let mut best_path: Vec<&Op> = vec![];
		let start_valve = self.get_valve(start);
		let start_valve_label = u16_to_label(start);
		print!("[{start_valve_label}] ->");
		for cnx in &start_valve.cnx {
			let cnx_label = u16_to_label(*cnx);
			print!(" {cnx_label}");
		}
		println!();
		let v: Vec<Op> = vec![];
		self.get_ops_at(start, &v);
		0
	}

	fn get_ops_at(&self, src: u16, ops: &Vec<Op>) -> Vec<Op> {
		let mut r = vec![];
		fn was_opened(src: u16, ops: &Vec<Op>) -> bool {
			for op in ops {
				if let Op::Open { dst } = op {
					if *dst == src {
						return true;
					}
				}
			}
			false
		}
		let v = self.get_valve(src);
		if v.flow > 0 && !was_opened(src, ops) {
			r.push(Op::Open { dst: src });
		}
		for cnx in &v.cnx {
			r.push(Op::Tunnel { dst: *cnx });
		}
		{
			let src_label = u16_to_label(src);
			print!("[{src_label}] ->");
			for op in &r {
				match op {
					Op::Open { dst } => {
						let dst = u16_to_label(*dst);
						print!(" ·{dst}·");
					}
					Op::Tunnel { dst } => {
						let dst = u16_to_label(*dst);
						print!(" ({dst})");
					}
				}
			}
			println!()
		}
		r
	}
}

/// Convert labels like "AA" to a unique [u16], used for indexing stuff later
fn label_to_u16(s: &str) -> u16 {
	assert_eq!(s.len(), 2);
	const A: u16 = 'A' as u16;
	const Z: u16 = 'Z' as u16;
	const A_Z: RangeInclusive<u16> = A..=Z;
	const RADIX: u16 = 1 + (Z - A); // A_Z.len() is not const...
	let mut s = s.chars().map(|c| {
		let c = u16::try_from(c).unwrap();
		assert!(A_Z.contains(&c));
		c - A
	});
	let (c1, c0) = (s.next().unwrap(), s.next().unwrap()); // "AB" -> 'A', 'B' -> ('A' * RADIX) + B
	(c1 * RADIX) + c0
}

/// Convert [u16] back to label, used for printing maybe...
fn u16_to_label(n: u16) -> String {
	const A: u16 = 'A' as u16;
	const Z: u16 = 'Z' as u16;
	const RADIX: u16 = 1 + (Z - A); // A_Z.len() is not const...
	const ZZ: u16 = Z * RADIX + Z;
	assert!(n <= ZZ);
	let (c1, c0) = (n / RADIX, n % RADIX);
	let s = [c1, c0].map(|c| char::from_u32((c + A).into()).unwrap());
	String::from_iter(s)
}

pub fn p1(lines: &[String]) -> usize {
	let cave = Cave::from_lines(lines);
	assert!(cave.verify());
	dbg!(cave.valves.len());
	cave.search(label_to_u16("AA"));
	0
}

pub fn p2(lines: &[String]) -> usize {
	p1(lines)
}

#[cfg(test)]
mod tests {
	use std::io::BufRead;

	use super::*;

	#[ignore]
	#[test]
	fn example1() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		assert_eq!(result, 0);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		assert_eq!(result, 0);
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

	#[ignore]
	#[test]
	fn test_label_to_u16() {
		let mut ids = vec![];
		for c1 in 'A'..='Z' {
			for c0 in 'A'..='Z' {
				let s: String = String::from_iter([c1, c0]);
				let id = label_to_u16(&s);
				assert_eq!(u16_to_label(id), s);
				ids.push(id);
			}
		}
		let len_before = ids.len();
		ids.dedup();
		let len_after = ids.len();
		assert_eq!(len_before, len_after);
		let mut n = 0;
		for id in ids {
			assert_eq!(id, n);
			n = n + 1;
		}
	}
}
