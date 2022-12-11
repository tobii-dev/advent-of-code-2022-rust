fn has_repeating(s: &[char]) -> bool {
	for (i, c) in s.iter().enumerate() {
		for h in &s[i + 1..] {
			if c == h {
				return true;
			}
		}
	}
	false
}

pub fn p1(lines: &Vec<String>) -> Vec<usize> {
	let mut v = vec![];
	for line in lines {
		'w: for (pos, window) in line.chars().collect::<Vec<_>>().windows(4).enumerate() {
			println!("[{pos}]: {}", window.to_vec().iter().collect::<String>());
			if !has_repeating(window) {
				v.push(pos + window.len());
				break 'w;
			}
		}
	}
	v
}

pub fn p2(lines: &Vec<String>) -> Vec<usize> {
	let mut v = vec![];
	for line in lines {
		'w: for (pos, window) in line.chars().collect::<Vec<_>>().windows(14).enumerate() {
			println!("[{pos}]: {}", window.to_vec().iter().collect::<String>());
			if !has_repeating(window) {
				v.push(pos + window.len());
				break 'w;
			}
		}
	}
	v
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
		assert_eq!(r, [7, 5, 6, 10, 11]);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, [1965]);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, [19, 23, 23, 29, 26]);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, [2773]);
	}
}
