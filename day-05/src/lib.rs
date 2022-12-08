#[derive(Debug)]
struct Harbour {
	sections: Vec<Vec<char>>,
}

impl Harbour {
	fn new(n: usize) -> Harbour {
		Harbour {
			sections: vec![Vec::new(); n],
		}
	}

	fn push_container(&mut self, section: usize, c: char) {
		self.sections[section].push(c);
	}

	fn rev_stacks(&mut self) {
		for stack in self.sections.iter_mut() {
			stack.reverse();
		}
	}

	fn do_move(&mut self, m: &Move) {
		for _ in 0..m.count {
			let c = self.sections[m.src].pop().unwrap();
			self.sections[m.dst].push(c);
		}
	}

	/// with crane 9001 can move multiple container in one move
	fn do_move_crane(&mut self, m: &Move) {
		let mut v: Vec<char> = vec![];
		for _ in 0..m.count {
			let c = self.sections[m.src].pop().unwrap();
			v.push(c);
		}
		for _ in 0..m.count {
			let c = v.pop().unwrap();
			self.sections[m.dst].push(c);
		}
	}

	fn top_as_str(&self) -> String {
		let mut s = "".to_string();
		for stack in &self.sections {
			if let Some(&c) = &stack.last() {
				s.push(c);
			}
		}
		s
	}

	fn to_string(&self) -> String {
		let n = self.sections.len();
		let mut s = format!("Harbour with {n} sections: ");
		for (i, stack) in self.sections.iter().enumerate() {
			let mut ss = "[".to_string();
			for &c in stack.iter() {
				ss.push(c);
			}
			s.push_str(&format!("\n{i} = {ss}"));
		}
		s
	}
}

struct Move {
	count: usize,
	src: usize,
	dst: usize,
}

impl Move {
	fn from_str(s: &String) -> Self {
		let words: Vec<&str> = s.split_whitespace().collect();
		let count = words.get(1).unwrap().parse::<usize>().unwrap();
		let src = words.get(3).unwrap().parse::<usize>().unwrap() - 1;
		let dst = words.get(5).unwrap().parse::<usize>().unwrap() - 1;
		Move { count, src, dst }
	}
}

pub fn p1(lines: &Vec<String>) -> String {
	let n = (lines[0].len() / 4) + 1; // to get number of sections
	let mut harbor = Harbour::new(n);
	for line in lines {
		if line.is_empty() {
			// moves start here
			harbor.rev_stacks();
			// println!("Before moves: {}", harbor.to_string());
		} else if line.starts_with("move") {
			let m = Move::from_str(line);
			assert!(m.src < n);
			assert!(m.dst < n);
			harbor.do_move(&m); // move normal
		} else if line.contains("[") {
			//"[_] [_] [_] [_] [_] [_] [_] [_] [_]"
			for section in 0..n {
				let container: char = line.chars().nth(1 + section * 4).unwrap();
				if container.is_ascii_uppercase() {
					harbor.push_container(section, container);
				}
			}
		} else { //" 1   2   3   4   5   6   7   8   9 "
		}
	}
	println!("After moves: {}", harbor.to_string());
	harbor.top_as_str()
}

pub fn p2(lines: &Vec<String>) -> String {
	let n = (lines[0].len() / 4) + 1;
	let mut harbor = Harbour::new(n);
	for line in lines {
		if line.is_empty() {
			harbor.rev_stacks();
			// println!("Before moves: {}", harbor.to_string());
		} else if line.starts_with("move") {
			//"move 1 from 8 to 1"
			let m = Move::from_str(line);
			assert!(m.src < n);
			assert!(m.dst < n);
			harbor.do_move_crane(&m);
		} else if line.contains("[") {
			//"[1] [2] [3] [4] [5] [6] [7] [8] [9]"
			for section in 0..n {
				let container: char = line.chars().nth(1 + section * 4).unwrap();
				if container.is_ascii_uppercase() {
					harbor.push_container(section, container);
				}
			}
		} else { //" 1   2   3   4   5   6   7   8   9 "
		}
	}
	println!("After moves: {}", harbor.to_string());
	harbor.top_as_str()
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
		assert_eq!(r, "CMZ");
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, "VGBBJCRMN");
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, "MCD");
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, "LBBVJBRMH");
	}
}
