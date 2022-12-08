pub fn section_as_bits(s: &str) -> u128 {
	let (left, right) = s.split_once("-").unwrap();
	let (mut left, mut right) = (
		left.parse::<u128>().unwrap(),
		right.parse::<u128>().unwrap(),
	);
	if left > right {
		(left, right) = (right, left);
	}
	assert!((1 <= left) && (right <= 99));
	let mut bits: u128 = 0;
	for sh in left..=right {
		bits |= 1 << (sh - 1);
	}
	bits
}

pub fn p1(lines: &Vec<String>) -> usize {
	let mut total = 0;
	for line in lines {
		let (left, right) = line.split_once(",").unwrap();
		let (left_bits, right_bits) = (section_as_bits(&left), section_as_bits(&right));
		let overlap = left_bits & right_bits;
		if overlap == left_bits {
			// right fully contained left
			total += 1;
			println!("{left}\t{left_bits:#0128b}");
			println!("{right}\t{right_bits:#0128b}");
			println!("\t{overlap:#0128b}\n");
		} else if overlap == right_bits {
			// left fully contained right
			total += 1;
			println!("{left}\t{left_bits:#0128b}");
			println!("{right}\t{right_bits:#0128b}");
			println!("\t{overlap:#0128b}\n");
		}
	}
	total
}

pub fn p2(lines: &Vec<String>) -> usize {
	let mut total = 0;
	for line in lines {
		let (left, right) = line.split_once(",").unwrap();
		let (left_bits, right_bits) = (section_as_bits(&left), section_as_bits(&right));
		if (left_bits & right_bits) != 0 {
			total += 1;
		}
	}
	total
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
		assert_eq!(r, 2);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 494);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 4);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 833);
	}
}
