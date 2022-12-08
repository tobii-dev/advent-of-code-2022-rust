pub fn item_as_bit(item: &char) -> u64 {
	let sh = if item.is_ascii_lowercase() {
		// a..z => 1..26
		(*item as u64) - ('a' as u64) + 1
	} else if item.is_ascii_uppercase() {
		//A..Z => 27..52
		(*item as u64) - ('A' as u64) + 27
	} else {
		unreachable!("Can't item_as_bit({item})");
	};
	assert!((1 <= sh) && (sh <= 52));
	(1 as u64) << sh
}

pub fn p1(lines: &Vec<String>) -> usize {
	let mut total = 0;
	for line in lines {
		let (left, right) = line.split_at(line.len() / 2);
		let (mut left_bits, mut right_bits) = (0u64, 0u64);
		for (left, right) in left.chars().zip(right.chars()) {
			left_bits |= item_as_bit(&left);
			right_bits |= item_as_bit(&right);
		}
		let overlap: u64 = left_bits & right_bits;
		assert!(overlap.count_ones() == 1);
		let p = overlap.trailing_zeros();
		total += p;
	}
	total.try_into().unwrap()
}

pub fn p2(lines: &mut Vec<String>) -> usize {
	let mut total = 0;
	assert!(lines.len() % 3 == 0);
	for group in lines.chunks(3) {
		let mut overlap = !0u64; // fill with 1s
		assert!(overlap.count_ones() == 64);
		for line in group {
			let mut bits = 0u64;
			for item in line.chars() {
				bits |= item_as_bit(&item);
			}
			overlap &= bits;
		}
		assert!(overlap.count_ones() == 1);
		let p = overlap.trailing_zeros();
		total += p;
	}
	total.try_into().unwrap()
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
		assert_eq!(r, 157);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 7746);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 70);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 2604);
	}
}
