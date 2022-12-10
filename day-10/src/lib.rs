const KEY_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

const SCREEN_X: usize = 40;
const SCREEN_Y: usize = 6;

enum Op {
	Noop,
	AddX(isize),
}

struct Radio {
	x: isize,
	pc: usize,
	signals: Vec<isize>,
	screen: [bool; SCREEN_X * SCREEN_Y],
}

impl Radio {
	fn new() -> Self {
		Radio {
			x: 1,
			pc: 0,
			signals: vec![],
			screen: [false; SCREEN_X * SCREEN_Y],
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
		self.crt();
		self.pc += 1;
		if KEY_CYCLES.contains(&self.pc) {
			self.signals.push(self.pc as isize * self.x);
		}
	}

	fn crt(&mut self) {
		let pos: isize = (self.pc % SCREEN_X).try_into().unwrap();
		if (self.x - 1..=self.x + 1).contains(&pos) {
			self.screen[self.pc] = true;
		};
	}

	fn display(&self) -> String {
		let mut display = "".to_string();
		for row in self.screen.chunks(SCREEN_X) {
			for px in row {
				if *px {
					display.push('#');
				} else {
					display.push('.');
				}
			}
			display.push('\n');
		}
		display
	}
}

pub fn p1(lines: &Vec<String>) -> isize {
	let radio = Radio::from(lines);
	radio.signals.iter().sum()
}

pub fn p2(lines: &Vec<String>) -> String {
	let radio = Radio::from(lines);
	radio.display()
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
		assert_eq!(r, 13140);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 14060);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		let mut ans = "".to_string();
		ans.push_str("##..##..##..##..##..##..##..##..##..##..\n");
		ans.push_str("###...###...###...###...###...###...###.\n");
		ans.push_str("####....####....####....####....####....\n");
		ans.push_str("#####.....#####.....#####.....#####.....\n");
		ans.push_str("######......######......######......####\n");
		ans.push_str("#######.......#######.......#######.....\n");
		assert_eq!(r, ans);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		let mut ans = "".to_string();
		ans.push_str("###...##..###..#..#.####.#..#.####...##.\n");
		ans.push_str("#..#.#..#.#..#.#.#..#....#.#..#.......#.\n");
		ans.push_str("#..#.#..#.#..#.##...###..##...###.....#.\n");
		ans.push_str("###..####.###..#.#..#....#.#..#.......#.\n");
		ans.push_str("#....#..#.#....#.#..#....#.#..#....#..#.\n");
		ans.push_str("#....#..#.#....#..#.#....#..#.####..##..\n");
		assert_eq!(r, ans);
	}
}
