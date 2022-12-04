#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Copy, Clone)]
enum Round {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Hand {
    fn beats(&self) -> Hand {
        match &self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
    fn loses(&self) -> Hand {
        match &self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn vs(&self, op: &Hand) -> Round {
        if self.beats() == *op { //own hand beats opp
            return Round::Win
        } else if op.beats() == *self { // opp beats own hand
            return Round::Loss;
        }
        return Round::Draw;
    }

    fn score(&self, op: &Hand) -> usize { // total score for self after playing round of self vs op
        let shape_score = *self as usize;
        let round_score = *&(self.vs(op)) as usize;
        shape_score + round_score
    }

    fn from_str(x: &str) -> Self {
        match x {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => unreachable!("Can't create Hand::{{}} from_str: {x}"),
        }
    }
}


impl Round {
    fn from_str(x: &str) -> Self {
        match x {
            "X" => Round::Loss,
            "Y" => Round::Draw,
            "Z" => Round::Win,
            _ => unreachable!("Can't create Hand::{{}} from_str: {x}"),
        }
    }
}


pub fn p1(lines: &Vec<String>) -> usize {
    let mut total = 0;
    for line in lines {
        let mut words = line.split_whitespace();
        let hand_opp = Hand::from_str(&words.next().unwrap());
        let hand_own = Hand::from_str(&words.next().unwrap());
        let s = hand_own.score(&hand_opp);
        total += s;
    }
    total
}


pub fn p2(lines: &Vec<String>) -> usize {
    let mut total = 0;
    for line in lines {
        let mut words = line.split_whitespace();
        let hand_opp = Hand::from_str(&words.next().unwrap());
        let desired_round = Round::from_str(&words.next().unwrap());
        let hand_own = match desired_round {
            Round::Draw => hand_opp.clone(),
            Round::Loss => hand_opp.beats(),
            Round::Win => hand_opp.loses(),
        };
        let s = hand_own.score(&hand_opp);
        total += s;
    }
    total
}


#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use super::*;
    #[test]
    fn ex1() {
        let fd = std::fs::File::open("example.txt").unwrap();
        let lines: Vec<String> = std::io::BufReader::new(fd).lines().map(|l| l.unwrap()).collect();
        let r = p1(&lines);
        assert_eq!(r, 15);
    }


    #[test]
    fn part1() {
        let fd = std::fs::File::open("input.txt").unwrap();
        let lines: Vec<String> = std::io::BufReader::new(fd).lines().map(|l| l.unwrap()).collect();
        let r = p1(&lines);
        assert_eq!(r, 14375);
    }

    #[test]
    fn ex2() {
        let fd = std::fs::File::open("example.txt").unwrap();
        let lines: Vec<String> = std::io::BufReader::new(fd).lines().map(|l| l.unwrap()).collect();
        let r = p2(&lines);
        assert_eq!(r, 12);
    }


    #[test]
    fn part2() {
        let fd = std::fs::File::open("input.txt").unwrap();
        let lines: Vec<String> = std::io::BufReader::new(fd).lines().map(|l| l.unwrap()).collect();
        let r = p2(&lines);
        assert_eq!(r, 10274);
    }
}
