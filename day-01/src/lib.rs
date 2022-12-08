pub fn p1(lines: &Vec<String>) -> usize {
	let mut c = 0;
	let mut max = 0;
	for line in lines {
		if line.is_empty() {
			if c > max {
				max = c;
			}
			c = 0;
		} else {
			let x = line.parse::<usize>().unwrap();
			c += x;
		}
	}
	if c > max {
		// don't forget last line
		max = c;
	}
	max
}

pub fn p2(lines: &Vec<String>) -> usize {
	let mut top3 = vec![0, 0, 0];
	let mut c = 0;
	for line in lines {
		if line.is_empty() {
			println!("c = {c}");
			top3.push(c);
			top3.sort_by(|a, b| b.cmp(a));
			top3.pop();
			println!("top3 = {top3:#?}");
			c = 0;
		} else {
			let x = line.parse::<usize>().unwrap();
			c += x;
		}
		println!("{line:#?}");
	}
	top3.push(c); // don't forget last line
	top3.sort_by(|a, b| b.cmp(a));
	top3.pop();
	println!("top3 = {top3:#?}");
	top3.iter().sum()
}

// general case for sum of top N
pub fn g(lines: &Vec<String>, n: usize) -> usize {
	let mut top = vec![0; n];
	let mut c = 0;
	for line in lines {
		if line.is_empty() {
			println!("c = {c}");
			top.push(c);
			top.sort_by(|a, b| b.cmp(a));
			top.pop();
			println!("top = {top:#?}");
			c = 0;
		} else {
			let x = line.parse::<usize>().unwrap();
			c += x;
		}
		println!("{line:#?}");
	}
	top.push(c); // don't forget last line
	top.sort_by(|a, b| b.cmp(a));
	top.pop();
	println!("top = {top:#?}");
	top.iter().sum()
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
		assert_eq!(result, 24000);
	}

	#[test]
	fn t1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p1(&lines);
		assert_eq!(result, 68292);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines);
		assert_eq!(result, 45000);
	}

	#[test]
	fn t2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result = p2(&lines);
		assert_eq!(result, 203203);
	}

	#[test]
	fn general() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result_top1 = g(&lines.clone(), 1);
		assert_eq!(result_top1, 24000);

		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let result_top3 = g(&lines, 3);
		assert_eq!(result_top3, 203203);
	}
}
