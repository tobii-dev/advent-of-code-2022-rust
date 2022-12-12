use std::collections::VecDeque;

#[derive(Debug)]
struct Test {
	div: isize,
	pass_throw: usize,
	fail_throw: usize,
}

#[derive(Debug)]
enum Op {
	Mul(isize),
	Sum(isize),
	Ssq,
}

#[derive(Debug)]
struct Monkey {
	op: Op,
	test: Test,
	items: VecDeque<isize>,
	counter: usize,
}

impl Monkey {
	fn from(lines: &[String]) -> Self {
		let mut op = None;
		let mut test_div = None;
		let mut test_pass = None;
		let mut test_fail = None;
		let mut items = None;
		for line in lines {
			let mut words = line.split_whitespace();
			if let Some(s) = words.next() {
				match s {
					"Monkey" => {
						let _id: usize = words
							.next()
							.unwrap()
							.split_once(':')
							.unwrap()
							.0
							.parse()
							.unwrap();
					}
					"Starting" => {
						let list = line.split_once(':').unwrap().1.split(',');
						let mut q = VecDeque::new();
						for item in list {
							let item: isize = item.trim().parse().unwrap();
							q.push_back(item);
						}
						items = Some(q);
					}
					"Operation:" => {
						let inmm = words.last().unwrap();
						match inmm {
							"old" => {
								op = Some(Op::Ssq);
							}
							inmm => {
								if line.contains('+') {
									op = Some(Op::Sum(inmm.parse().unwrap()));
								} else if line.contains('*') {
									op = Some(Op::Mul(inmm.parse().unwrap()));
								} else {
									unreachable!("Bad monkey parsing: Operation @  \"{inmm}\"?");
								}
							}
						}
					}
					"Test:" => {
						let val = words.last().unwrap().parse().unwrap();
						test_div = Some(val);
					}
					"If" => match words.next().unwrap() {
						"true:" => {
							test_pass = Some(words.last().unwrap().parse().unwrap());
						}
						"false:" => {
							test_fail = Some(words.last().unwrap().parse().unwrap());
						}
						s => {
							unreachable!("Bad monkey parsing: If \"{s}\"?");
						}
					},
					_ => {
						unreachable!("Bad monkey parsing!");
					}
				}
			}
		}
		Monkey {
			op: op.unwrap(),
			test: Test {
				div: test_div.unwrap(),
				pass_throw: test_pass.unwrap(),
				fail_throw: test_fail.unwrap(),
			},
			items: items.unwrap(),
			counter: 0,
		}
	}

	fn inspect(&mut self, chill_inv: isize, modu: Option<isize>) -> (usize, isize) {
		self.counter += 1;
		let mut item = self.items.pop_front().unwrap();
		if let Some(modu) = modu {
			item %= modu;
		};
		println!("  Monkey inspects an item with a worry level of {item}.");
		item = match self.op {
			Op::Sum(inmm) => {
				let r = item + inmm;
				println!("    Worry level increases by {inmm} to {r}.");
				r
			}
			Op::Mul(inmm) => {
				let r = item * inmm;
				println!("    Worry level is multiplied by {inmm} to {r}.");
				r
			}
			Op::Ssq => {
				let r = item * item;
				println!("    Worry level is multiplied by itself to {r}.");
				r
			}
		};
		item /= chill_inv;
		println!(
			"    Monkey gets bored with item. Worry level is divided by {chill_inv} to {item}."
		);
		let div = self.test.div;
		let test = item % div == 0;
		let next = if test {
			println!("    Current worry level is divisible by {div}.");
			self.test.pass_throw
		} else {
			println!("    Current worry level is not divisible by {div}.");
			self.test.fail_throw
		};
		println!("    Item with worry level {item} is thrown to monkey {next}.");
		(next, item)
	}
}
struct Jungle {
	chill_inv: isize,
	modu: Option<isize>,
	monkeys: Vec<Monkey>,
}

impl Jungle {
	fn from(lines: &[String], chill_inv: isize) -> Self {
		let mut monkeys = vec![];
		let mut modu = 1;
		for def in lines.chunks(7) {
			let monkey = Monkey::from(def);
			modu *= monkey.test.div;
			monkeys.push(monkey);
		}
		let modu = if chill_inv == 1 { Some(modu) } else { None };
		Jungle {
			chill_inv,
			modu,
			monkeys,
		}
	}

	fn run(&mut self, rounds: usize) {
		for _ in 0..rounds {
			self.do_round();
		}
	}

	fn do_round(&mut self) {
		for i in 0..self.monkeys.len() {
			let monkey = self.monkeys.get_mut(i).unwrap();
			let mut throws = vec![];
			println!("Monkey {i}:");
			while !monkey.items.is_empty() {
				throws.push(monkey.inspect(self.chill_inv, self.modu));
			}
			for (id, item) in throws {
				self.monkeys.get_mut(id).unwrap().items.push_back(item);
			}
		}
	}

	fn get_monkey_business(&self) -> usize {
		let mut max = vec![0, 0];
		for monkey in &self.monkeys {
			let c = monkey.counter;
			max.push(c);
			max.sort_by(|a, b| b.cmp(a));
			max.pop();
		}
		max.iter().product()
	}
}

pub fn p1(lines: &[String]) -> usize {
	let mut jungle = Jungle::from(lines, 3);
	jungle.run(20);
	jungle.get_monkey_business()
}

pub fn p2(lines: &[String]) -> usize {
	let mut jungle = Jungle::from(lines, 1);
	jungle.run(10_000);
	jungle.get_monkey_business()
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
		assert_eq!(r, 10605);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 58786);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&lines);
		assert_eq!(r, 2713310158);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&lines);
		assert_eq!(r, 14952185856);
	}
}
