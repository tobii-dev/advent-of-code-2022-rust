use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum DeviceError {
	DirAlreadyExists(DirError),
	VarAlreadyExists,
	DirNotFound,
	DirAboveRoot,
}

struct Device {
	root: Rc<RefCell<Dir>>,
	ptr: Rc<RefCell<Dir>>,
}

impl Device {
	fn new() -> Self {
		let root = Rc::new(RefCell::new(Dir::new()));
		let ptr = Rc::clone(&root);
		Self { root, ptr }
	}

	fn add_dir(&mut self, name: &str) -> Result<(), DeviceError> {
		let dir = self
			.ptr
			.borrow_mut()
			.add_dir(name)
			.map_err(DeviceError::DirAlreadyExists)?;
		dir.borrow_mut().root = Some(self.ptr.clone());
		Ok(())
	}

	fn add_var(&mut self, name: &str, size: usize) -> Result<(), DeviceError> {
		self.ptr
			.borrow_mut()
			.add_var(name, size)
			.map_err(|_| DeviceError::VarAlreadyExists)?;
		Ok(())
	}

	fn cd(&mut self, name: Option<&str>) -> Result<(), DeviceError> {
		let dir: Rc<RefCell<Dir>> = match name {
			Some(name) => {
				match self
					.ptr
					.borrow()
					.dirs
					.iter()
					.find(|v| v.borrow().name == name)
				{
					Some(r) => r.clone(),
					None => return Err(DeviceError::DirNotFound),
				}
			}
			None => match &self.ptr.borrow().root {
				Some(r) => r.clone(),
				None => return Err(DeviceError::DirAboveRoot),
			},
		};
		self.ptr = dir;
		Ok(())
	}

	fn get_all_dirs(&self) -> Vec<Rc<RefCell<Dir>>> {
		return self.root.borrow().get_sub_dirs();
	}

	fn get_total_size(&self) -> usize {
		self.root.borrow_mut().calc_size();
		self.root.borrow().size
	}
}

#[derive(Debug)]
enum DirError {
	DirAlreadyExists,
	VarAlreadyExists,
}

struct Dir {
	name: String,
	size: usize,
	root: Option<Rc<RefCell<Dir>>>,
	dirs: Vec<Rc<RefCell<Dir>>>,
	vars: Vec<(String, usize)>,
}

impl Dir {
	fn new() -> Self {
		Dir {
			name: "/".to_string(),
			size: 0,
			root: None,
			dirs: vec![],
			vars: vec![],
		}
	}

	fn add_dir(&mut self, name: &str) -> Result<Rc<RefCell<Self>>, DirError> {
		if self.dirs.iter().any(|v| v.borrow().name == name) {
			return Err(DirError::DirAlreadyExists);
		}
		let dir = Dir {
			name: name.to_string(),
			size: 0,
			root: None, //TODO!
			dirs: vec![],
			vars: vec![],
		};
		let dir = Rc::new(RefCell::new(dir));
		self.dirs.push(dir.clone());
		Ok(dir)
	}

	fn add_var(&mut self, name: &str, size: usize) -> Result<(), DirError> {
		if self
			.vars
			.iter()
			.any(|(var_name, _var_size)| var_name == name)
		{
			return Err(DirError::VarAlreadyExists);
		}
		self.vars.push((name.to_string(), size));
		Ok(())
	}

	fn get_sub_dirs(&self) -> Vec<Rc<RefCell<Dir>>> {
		let mut v: Vec<Rc<RefCell<Dir>>> = vec![];
		for dir in self.dirs.iter() {
			for sub_dir in dir.borrow().get_sub_dirs() {
				v.push(sub_dir.clone());
			}
			v.push(dir.clone());
		}
		v
	}

	fn calc_size(&mut self) {
		self.size = 0;
		for (_name, size) in self.vars.iter() {
			self.size += size;
		}
		for dir in self.dirs.iter() {
			dir.borrow_mut().calc_size();
			self.size += dir.borrow().size;
		}
	}
}

pub fn p1(lines: &Vec<String>) -> usize {
	const MAX_SIZE: usize = 100_000;
	let mut dev = Device::new();

	for line in lines {
		if line.starts_with("$ cd /") {
			dev.cd(Some("/")).unwrap();
		} else if line.starts_with("$ ls") {
			// TODO: dev.ls(); ?
		} else if line.starts_with("dir ") {
			let name = line.split_whitespace().last().unwrap();
			dev.add_dir(name).unwrap();
		} else if line.starts_with("$ cd ..") {
			dev.cd(None).unwrap();
		} else if line.starts_with("$ cd ") {
			let name: String = line.split_whitespace().last().unwrap().to_string();
			dev.cd(Some(&name)).unwrap();
		} else if line.chars().next().unwrap().is_numeric() {
			let mut words = line.split_whitespace();
			let size: usize = words.next().unwrap().parse().unwrap();
			let name: String = words.next().unwrap().to_string();
			dev.add_var(&name, size).unwrap();
		}
	}
	dev.get_total_size();
	dev.get_all_dirs()
		.iter()
		.filter_map(|v| {
			let size = v.borrow().size;
			if size <= MAX_SIZE {
				Some(size)
			} else {
				None
			}
		})
		.sum()
}

pub fn p2(lines: &Vec<String>) -> usize {
	const MAX_SIZE: usize = 70_000_000;
	const REQ_SIZE: usize = 30_000_000;
	const TOP_SIZE: usize = MAX_SIZE - REQ_SIZE;

	let mut dev = Device::new();

	for line in lines {
		if line.starts_with("$ cd /") {
			dev.cd(Some("/")).unwrap();
		} else if line.starts_with("$ ls") {
			//TODO: dev.ls()?;
		} else if line.starts_with("dir ") {
			let name = line.split_whitespace().last().unwrap();
			dev.add_dir(name).unwrap();
		} else if line.starts_with("$ cd ..") {
			dev.cd(None).unwrap();
		} else if line.starts_with("$ cd ") {
			let name: String = line.split_whitespace().last().unwrap().to_string();
			dev.cd(Some(&name)).unwrap();
		} else if line.chars().next().unwrap().is_numeric() {
			let mut words = line.split_whitespace();
			let size: usize = words.next().unwrap().parse().unwrap();
			let name: String = words.next().unwrap().to_string();
			dev.add_var(&name, size).unwrap();
		}
	}
	let total = dev.get_total_size();
	assert!(total < MAX_SIZE);
	assert!(total > TOP_SIZE);

	let target = total - TOP_SIZE;

	// let mut canditates = std::collections::BTreeSet::new();
	// for dir in dev.get_all_dirs() {
	//	 canditates.insert(dir.borrow().size);
	// }
	//return (&canditates).range(target..).next().unwrap();

	let mut smallest = total;
	for dir in dev.get_all_dirs() {
		let size = dir.borrow().size;
		if (size >= target) && (size < smallest) {
			smallest = size;
		}
	}
	smallest
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
		assert_eq!(r, 95437);
	}

	#[test]
	fn part1() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p1(&lines);
		assert_eq!(r, 1449447);
	}

	#[test]
	fn example2() {
		let fd = std::fs::File::open("example.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 24933642);
	}

	#[test]
	fn part2() {
		let fd = std::fs::File::open("input.txt").unwrap();
		let mut lines: Vec<String> = std::io::BufReader::new(fd)
			.lines()
			.map(|l| l.unwrap())
			.collect();
		let r = p2(&mut lines);
		assert_eq!(r, 8679207);
	}
}
