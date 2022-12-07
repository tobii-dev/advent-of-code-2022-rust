use std::cell::RefCell;
use std::rc::Rc;

use std::collections::BTreeSet;



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
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lvl = f.width().unwrap_or(0);
        let prefix = "\t".repeat(lvl);
        let name = &self.name;
        let size = &self.size;
        writeln!(f, "{prefix}\"{name}\" # ({size}) [")?;

        let len_dirs = self.dirs.len();
        writeln!(f, "{prefix}\tdirs: ~ {len_dirs} (")?;
        for dir in &self.dirs {
            let dir = dir.borrow();
            let l = lvl + 1;
            writeln!(f, "{dir:l$}")?;
        }
        writeln!(f, "\t{prefix})")?;

        let len_vars = self.vars.len();
        writeln!(f, "{prefix}\tvars: ~ {len_vars} {{")?;
        for var in &self.vars {
            let (var_name, var_size) = var;
            writeln!(f, "{prefix}\t\t{var_name}: ({var_size})")?;
        }
        writeln!(f, "{prefix}\t}}")?;

        writeln!(f, "{prefix}]")?;

        Ok(())
    }
}

pub fn p1(lines: &Vec<String>) -> usize {
    let mut total = 0;
    const MAX_SIZE: usize = 100_000;
    let root = Rc::new(RefCell::new(Dir::new()));
    let mut ptr = Rc::clone(&root);

    for line in lines {
        if line.starts_with("$ cd /") {
            //?
        } else if line.starts_with("$ ls") {
            //?
        } else if line.starts_with("dir ") {
            let name = line.split_whitespace().last().unwrap();
            assert!(&ptr
                .borrow()
                .dirs
                .iter()
                .find(|v| v.borrow().name == name)
                .is_none());
            let dir = Dir {
                name: name.to_string(),
                size: 0,
                root: None,
                dirs: vec![],
                vars: vec![],
            };
            let dir = Rc::new(RefCell::new(dir));
            ptr.borrow_mut().dirs.push(Rc::clone(&dir));
            dir.borrow_mut().root = Some(Rc::clone(&ptr));
        } else if line.starts_with("$ cd ..") {
            let size = ptr.borrow().size;
            if size <= MAX_SIZE {
                total += size;
            }
            let ptr_clone = Rc::clone(&ptr);
            ptr = Rc::clone(ptr_clone.borrow().root.as_ref().unwrap());
            ptr.borrow_mut().size += size;
        } else if line.starts_with("$ cd ") {
            let name: String = line.split_whitespace().last().unwrap().to_string();

            let ptr_clone = Rc::clone(&ptr);
            let dir = Rc::clone(
                ptr_clone
                    .borrow()
                    .dirs
                    .iter()
                    .find(|v| v.borrow().name == name)
                    .unwrap(),
            );
            assert!(dir.borrow().size == 0);
            ptr = Rc::clone(&dir);
        } else if line.chars().next().unwrap().is_numeric() {
            let mut words = line.split_whitespace();
            let size: usize = words.next().unwrap().parse().unwrap();
            let name: String = words.next().unwrap().to_string();
            assert!(&ptr
                .borrow()
                .vars
                .iter()
                .find(|(v, _size)| v == &name)
                .is_none());
            {
                let mut ptr = ptr.borrow_mut();
                ptr.size += size;
                ptr.vars.push((name, size));
            }
        }
    }
    let rt = root.borrow();
    println!("{rt}");
    total
}

pub fn p2(lines: &Vec<String>) -> usize {
    const MAX_SIZE: usize = 70_000_000;
    const REQ_SIZE: usize = 30_000_000;
    const TOP_SIZE: usize = MAX_SIZE - REQ_SIZE;
    let mut canditates = BTreeSet::new();

    let root = Rc::new(RefCell::new(Dir::new()));
    let mut ptr = Rc::clone(&root);

    for line in lines {
        if line.starts_with("$ cd /") {
        } else if line.starts_with("$ ls") {

        } else if line.starts_with("dir ") {
            let name = line.split_whitespace().last().unwrap();
            assert!(&ptr
                .borrow()
                .dirs
                .iter()
                .find(|v| v.borrow().name == name)
                .is_none());
            let dir = Dir {
                name: name.to_string(),
                size: 0,
                root: None,
                dirs: vec![],
                vars: vec![],
            };
            let dir = Rc::new(RefCell::new(dir));
            ptr.borrow_mut().dirs.push(Rc::clone(&dir));
            dir.borrow_mut().root = Some(Rc::clone(&ptr));

        } else if line.starts_with("$ cd ..") {
            let size = ptr.borrow().size;
            let ptr_clone = Rc::clone(&ptr);
            ptr = Rc::clone(ptr_clone.borrow().root.as_ref().unwrap());
            ptr.borrow_mut().size += size;
            canditates.insert(size); // TODO: check if insert can be ommited based on prev info.

        } else if line.starts_with("$ cd ") {
            let name: String = line.split_whitespace().last().unwrap().to_string();

            let ptr_clone = Rc::clone(&ptr); //TODO: Is this needed?
            let dir = Rc::clone(
                ptr_clone
                    .borrow()
                    .dirs
                    .iter()
                    .find(|v| v.borrow().name == name)
                    .unwrap(),
            );
            assert!(dir.borrow().size == 0);
            ptr = Rc::clone(&dir);
        } else if line.chars().next().unwrap().is_numeric() {
            let mut words = line.split_whitespace();
            let size: usize = words.next().unwrap().parse().unwrap();
            let name: String = words.next().unwrap().to_string();
            assert!(&ptr
                .borrow()
                .vars
                .iter()
                .find(|(v, _size)| v == &name)
                .is_none());
            {
                let mut ptr = ptr.borrow_mut();
                ptr.size += size;
                ptr.vars.push((name, size));
            }
        }
    }

    while ptr.borrow().root.is_some() { // crawl back to root
        let ptr_clone = Rc::clone(&ptr);
        let size = ptr.borrow().size;
        ptr = Rc::clone(ptr_clone.borrow().root.as_ref().unwrap());
        ptr.borrow_mut().size += size;
        canditates.insert(size);
    }

    assert!(ptr.as_ptr() == root.as_ptr()); //FIXME: sure?
    let total_used = ptr.borrow().size;
    dbg!(&total_used);
    dbg!(&canditates);

    assert!(total_used > TOP_SIZE);
    let target = total_used - TOP_SIZE;
    dbg!(&target);

    let f = (&canditates).range(target..).next().unwrap();
    dbg!(&f);
    *f
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
