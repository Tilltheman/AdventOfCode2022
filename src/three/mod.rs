use std::fs;

fn parse_input(input_file: &str) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = Vec::<(String, String)>::new();
    let content = fs::read_to_string(input_file).unwrap();
    for line in content.lines() {
        let half_len = line.len()/2;
        let compartment_left = String::from(&line[..half_len]);
        let compartment_right = String::from(&line[half_len..]);
        let tuple = (compartment_left, compartment_right);
        output.push(tuple);
    }
    output
}

fn parse_input_part2(input_file: &str) -> Vec<String> {
    let content = fs::read_to_string(input_file).unwrap();
    let mut output: Vec<String> = Vec::<String>::new();
    for line in content.lines() {
        output.push(line.to_string());
    }
    output
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 64 + 26
    }
}

fn solve_part1(input: &str) -> u32 {
    let backpacks = parse_input(input);
    let mut sum = 0;
    for backpack in backpacks {
        for item in backpack.0.chars() {
            if backpack.1.contains(item) {
                sum += get_priority(item);
                break;
            }
        }
    }
    sum
}

fn solve_part2(input: &str) -> u32 {
    let backpacks = parse_input_part2(input);
    let mut sum = 0;
    for counter in 0..backpacks.len()/3 {
        for item in backpacks[counter*3+0].chars() {
            if backpacks[counter*3+1].contains(item) && backpacks[counter*3+2].contains(item) {
                sum += get_priority(item);
                break;
            }
        }
    }
    sum
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/three/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/three/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let expected: Vec<(String, String)> = vec![
            ("vJrwpWtwJgWr".to_string(),"hcsFMMfFFhFp".to_string()),
            ("jqHRNqRjqzjGDLGL".to_string(),"rsFMfFZSrLrFZsSL".to_string()),
            ("PmmdzqPrV".to_string(),"vPwwTWBwg".to_string()),
            ("wMqvLMZHhHMvwLH".to_string(),"jbvcjnnSBnvTQFn".to_string()),
            ("ttgJtRGJ".to_string(),"QctTZtZT".to_string()),
            ("CrZsJsPPZsGz".to_string(),"wwsLwLmpwMDw".to_string()),
        ];
        let output = parse_input("src/three/test-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_get_priority_lowercase() {
        let expected: u32 = 16;
        let output = get_priority('p');
        println!("{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_get_priority_uppercase() {
        let expected: u32 = 42;
        let output = get_priority('P');
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part1() {
        let expected: u32 = 157;
        let output = solve_part1("src/three/test-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: u32 = 70;
        let output = solve_part2("src/three/test-input-part2.txt");
        assert_eq!(expected, output);
    }
}
