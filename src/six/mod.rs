use std::fs;
use itertools::Itertools;

fn parse_input(input_file: &str) -> String {
    let content = fs::read_to_string(input_file).unwrap();
    content
}

fn solve_part1(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let parsed_vec: Vec<char> = parsed_input.chars().collect();
    let length = parsed_vec.len();
    for i in 0..length-4 {
        if parsed_vec[i..i+4] == parsed_vec[i..i+4].into_iter().unique().copied().collect::<Vec<char>>() {
            println!("{}", i);
            return (i+4) as u32;
        }
    }
    length as u32
}

fn solve_part2(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let parsed_vec: Vec<char> = parsed_input.chars().collect();
    let length = parsed_vec.len();
    for i in 0..length-14 {
        if parsed_vec[i..i+14] == parsed_vec[i..i+14].into_iter().unique().copied().collect::<Vec<char>>() {
            println!("{}", i);
            return (i+14) as u32;
        }
    }
    length as u32
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/six/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/six/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let expected: u32= 5;
        let output = solve_part1("src/six/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: u32 = 23;
        let output = solve_part2("src/six/sample-input.txt");
        assert_eq!(expected, output);
    }
}
