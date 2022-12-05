use std::fmt;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct CrateStack {
    crates: Vec<char>,
}

impl fmt::Display for CrateStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for element in &self.crates {
            write!(f, "[{}] ", element)?;
        }
        Ok(())
    }
}

impl CrateStack {
    fn new() -> CrateStack {
        CrateStack { crates : vec![], }
    }
    fn pop(&mut self) -> char {
        return self.crates.pop().unwrap();
    }
    fn push(&mut self, c: char) {
        self.crates.push(c);
    }
    fn last(&self) -> char {
        return *self.crates.last().unwrap();
    }
}

#[derive(Debug)]
struct CrateStorage {
    amount_stacks: usize,
    crate_stacks: Vec<CrateStack>,
}

impl fmt::Display for CrateStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Amount Stacks: {}\n", self.amount_stacks)?;
        for (i, stack) in self.crate_stacks.iter().enumerate() {
            write!(f, "{}: {}\n", i, stack)?;
        }
        Ok(())
    }
}

impl CrateStorage {
    fn new(size: usize) -> CrateStorage {
        let mut crate_stacks = Vec::<CrateStack>::new();
        for _i in 0..size {
            crate_stacks.push(CrateStack::new());
        }
        CrateStorage {
            amount_stacks: size,
            crate_stacks: crate_stacks,
        }
    }
    #[allow(dead_code)]
    fn get_amount(self) -> usize {
        return self.amount_stacks;
    }
    #[allow(dead_code)]
    fn pop(&mut self, i: usize) -> char {
        return self.crate_stacks[i-1].pop();
    }

    fn push(&mut self, c: char, i: usize) {
        self.crate_stacks[i].push(c);
    }
    fn move_amount_from_to(&mut self, amount: usize, from: usize, to: usize) {
        for _i in 0..amount {
            let element: char = self.crate_stacks[from-1].pop();
            self.crate_stacks[to-1].push(element);
        }
    }
    fn move_amount_from_to_keeping_order(&mut self, amount: usize, from: usize, to: usize) {
        let mut temporary_stack: Vec<char> = Vec::new();
        for _i in 0..amount {
            temporary_stack.push(self.crate_stacks[from-1].pop());
        }
        for _i in 0..amount {
            self.crate_stacks[to-1].push(temporary_stack.pop().unwrap());
        }
    }
    fn part1(&self) -> String {
        let mut result: String = String::new();
        for element in &self.crate_stacks {
            result.push(element.last());
        }
        result
    }
}


fn parse_input(input_file: &str) -> Vec<String> {
    let content = fs::read_to_string(input_file).unwrap();
    let binding = content.clone();
    let parts: Vec<&str> = binding.split("\n\n").collect::<Vec<&str>>();
    let first_part = String::from(parts[0]);
    let second_part = String::from(parts[1]);
    vec![first_part, second_part]
}

fn solve_part1(input: &str) -> String {
    let parsed_input = parse_input(input);
    let part1 = &parsed_input[0];
    let part2 = &parsed_input[1];

    let initial_configuration: Vec<&str> = part1.split("\n").collect();
    let instructions: Vec<&str> = part2.split("\n").collect();

    let amount_lines = initial_configuration.len() as usize;
    let amount_characters_in_line = initial_configuration[0].len() as usize;
    let amount_stacks = ( amount_characters_in_line + 1 ) / 4 as usize;
    let mut crate_storage: CrateStorage = CrateStorage::new(amount_stacks);
    //println!("crate_storage: \n {}", crate_storage);
    for element in initial_configuration[0..amount_lines-1].iter().rev() {
        let result: Vec<_> = element.chars().skip(1).step_by(4).collect::<Vec<char>>();
        for (i, e) in result.iter().enumerate() {
            if *e != ' ' {
                crate_storage.push(*e, i);
            }
        }
    }
    //println!("Filled Crate Storage: {}", crate_storage);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    for line in instructions {
        //println!("Line: {}", line);
        // move around
        for cap in RE.captures_iter(line) {
            let amount: usize = cap[1].parse::<usize>().unwrap();
            let from: usize = cap[2].parse::<usize>().unwrap();
            let to: usize = cap[3].parse::<usize>().unwrap();
            crate_storage.move_amount_from_to(amount, from, to);
        }
        //println!("{}", crate_storage);
    }
    let res = String::from(crate_storage.part1());
    res
}

fn solve_part2(input: &str) -> String {
    let parsed_input = parse_input(input);
    let part1 = &parsed_input[0];
    let part2 = &parsed_input[1];

    let initial_configuration: Vec<&str> = part1.split("\n").collect();
    let instructions: Vec<&str> = part2.split("\n").collect();

    let amount_lines = initial_configuration.len() as usize;
    let amount_characters_in_line = initial_configuration[0].len() as usize;
    let amount_stacks = ( amount_characters_in_line + 1 ) / 4 as usize;
    let mut crate_storage: CrateStorage = CrateStorage::new(amount_stacks);
    //println!("crate_storage: \n {}", crate_storage);
    for element in initial_configuration[0..amount_lines-1].iter().rev() {
        let result: Vec<_> = element.chars().skip(1).step_by(4).collect::<Vec<char>>();
        for (i, e) in result.iter().enumerate() {
            if *e != ' ' {
                crate_storage.push(*e, i);
            }
        }
    }
    //println!("Filled Crate Storage: {}", crate_storage);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    for line in instructions {
        //println!("Line: {}", line);
        // move around
        for cap in RE.captures_iter(line) {
            let amount: usize = cap[1].parse::<usize>().unwrap();
            let from: usize = cap[2].parse::<usize>().unwrap();
            let to: usize = cap[3].parse::<usize>().unwrap();
            crate_storage.move_amount_from_to_keeping_order(amount, from, to);
        }
        //println!("{}", crate_storage);
    }
    let res = String::from(crate_storage.part1());
    res
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/five/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/five/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let expected: &str= "CMZ";
        let output = solve_part1("src/five/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: &str = "MCD";
        let output = solve_part2("src/five/sample-input.txt");
        assert_eq!(expected, output);
    }
}
