const KEY_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

enum Op {
	Noop,
	AddX(isize),
}

struct Radio {
	x: isize,
	pc: usize,
	signals: Vec<isize>,
}

impl Radio {
	fn new() -> Self {
		Radio {
			x: 1,
			pc: 0,
			signals: vec![],
		}
	}

	fn from(lines: &Vec<String>) -> Self {
		let mut radio = Radio::new();
		for line in lines {
			let mut words = line.split_whitespace();
			let opcode = words.next().unwrap();
			match opcode {
				"noop" => radio.op(&Op::Noop),
				"addx" => radio.op(&Op::AddX(words.next().unwrap().parse().unwrap())),
				_ => unreachable!(),
			};
		}
		radio
	}

	fn op(&mut self, op: &Op) {
		match op {
			Op::Noop => {
				self.tick();
			}
			Op::AddX(inmm) => {
				self.tick();
				self.tick();
				self.x += inmm;
			}
		}
	}

	fn tick(&mut self) {
		self.pc += 1;
		if KEY_CYCLES.contains(&self.pc) {
			let v: isize = self.pc.try_into().unwrap();
			self.signals.push(v * self.x);
		}
	}
}

pub fn p1(lines: &Vec<String>) -> isize {
	let radio = Radio::from(lines);
	radio.signals.iter().sum()
}

// pub fn p2(lines: &Vec<String>) -> usize {
// 	let radio = Radio::from(lines);
//     radio.signals.iter().sum()
// }

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
		assert_eq!(r, 13140);
	}

	// #[test]
	// fn part1() {
	// 	let fd = std::fs::File::open("input.txt").unwrap();
	// 	let lines: Vec<String> = std::io::BufReader::new(fd)
	// 		.lines()
	// 		.map(|l| l.unwrap())
	// 		.collect();
	// 	let r = p1(&lines);
	// 	assert_eq!(r, 5981);
	// }
	//
	// #[test]
	// fn example2() {
	// 	let fd = std::fs::File::open("example2.txt").unwrap();
	// 	let mut lines: Vec<String> = std::io::BufReader::new(fd)
	// 		.lines()
	// 		.map(|l| l.unwrap())
	// 		.collect();
	// 	let r = p2(&mut lines);
	// 	assert_eq!(r, 36);
	// }
	//
	// #[test]
	// fn part2() {
	// 	let fd = std::fs::File::open("input.txt").unwrap();
	// 	let mut lines: Vec<String> = std::io::BufReader::new(fd)
	// 		.lines()
	// 		.map(|l| l.unwrap())
	// 		.collect();
	// 	let r = p2(&mut lines);
	// 	assert_eq!(r, 2352);
	// }
}
