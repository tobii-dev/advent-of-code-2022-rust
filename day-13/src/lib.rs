#![allow(unused)]

use winnow::{
	ascii::digit1,
	combinator::{alt, delimited, separated},
	prelude::*,
};

#[derive(Debug)]
enum PacketData {
	Int(usize),
	List(Vec<PacketData>),
}

#[derive(Debug)]
struct Packet {
	data: PacketData,
}

#[derive(Debug)]
struct PacketPair {
	l: Packet,
	r: Packet,
}

impl PacketData {
	fn parse_list(input: &mut &str) -> PResult<Self> {
		let list_element = alt([Self::parse_num, Self::parse_list]);
		let list_elements = separated(0.., list_element, ",").map(Self::List);
		delimited("[", list_elements, "]").parse_next(input)
	}

	fn parse_num(input: &mut &str) -> PResult<Self> {
		digit1
			.try_map(|s: &str| s.parse::<usize>().map(Self::Int))
			.parse_next(input)
	}

	///`[[1],[2,3,4]]`
	pub fn from_str(s: &str) -> PResult<Self> {
		let mut s = s;
		Self::parse_list(&mut s)
	}

	pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
		match (&self, &other) {
			(PacketData::Int(a), PacketData::Int(b)) => a.cmp(b),
			(PacketData::Int(i), PacketData::List(_)) => {
				let a = PacketData::List(vec![PacketData::Int(*i)]);
				a.compare(other)
			}
			(PacketData::List(_), PacketData::Int(i)) => {
				let b = PacketData::List(vec![PacketData::Int(*i)]);
				self.compare(&b)
			}
			(PacketData::List(a), PacketData::List(b)) => {
				let (mut a, mut b) = (a.iter(), b.iter());
				loop {
					let (a, b) = (a.next(), b.next());
					return match (a, b) {
						(None, None) => std::cmp::Ordering::Equal,
						(None, Some(_)) => std::cmp::Ordering::Less,
						(Some(_), None) => std::cmp::Ordering::Greater,
						(Some(a), Some(b)) => match a.compare(b) {
							std::cmp::Ordering::Equal => continue, // treat next two pairs
							ord => ord,
						},
					};
				}
			}
		}
	}
}

impl Packet {
	fn from_str(s: &str) -> Self {
		let data = PacketData::from_str(s).unwrap();
		Self { data }
	}

	fn compare(&self, other: &Self) -> std::cmp::Ordering {
		self.data.compare(&other.data)
	}
}

impl PacketPair {
	fn list_from_lines(lines: &Vec<String>) -> Vec<Self> {
		let mut lines = lines.iter();
		let mut v = vec![];
		while let (Some(l), Some(r), _empty_line) = (lines.next(), lines.next(), lines.next()) {
			let (l, r) = (Packet::from_str(l), Packet::from_str(r));
			v.push(Self { l, r });
		}
		v
	}

	fn is_ordered(&self) -> bool {
		let ord = self.l.compare(&self.r);
		match ord {
			std::cmp::Ordering::Less => true, // left side is smaller -> correct order
			std::cmp::Ordering::Equal => {
				dbg!(&self);
				unreachable!("ERROR: no order could be determined for this PacketPair.")
			}
			std::cmp::Ordering::Greater => false, // left side is bigger -> incorrect order
		}
	}
}

pub fn p1(lines: &Vec<String>) -> usize {
	let pairs = PacketPair::list_from_lines(lines);
	let mut sum = 0;
	for (idx, pair) in pairs.iter().enumerate() {
		let idx = idx + 1;
		let is_ordered = pair.is_ordered();
		// println!("[{idx}]: {is_ordered}");
		if is_ordered {
			sum += idx;
		}
	}
	sum
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
		assert_eq!(result, 13);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		dbg!(result);
		assert_eq!(result, 5330);
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
