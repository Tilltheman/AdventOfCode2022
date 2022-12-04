use std::fs;

fn parse_input(input_file: &str) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = Vec::<(String, String)>::new();
    let content = fs::read_to_string(input_file).unwrap();
    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let first_elf = String::from(parts[0]);
        let second_elf = String::from(parts[1]);
        let tuple = (first_elf, second_elf);
        output.push(tuple);
    }
    output
}

fn contains(first_elf: Vec<&str>, second_elf: Vec<&str>) -> bool{
    let fe_end: i32 = first_elf[1].parse::<i32>().unwrap();
    let fe_begin: i32 = first_elf[0].parse::<i32>().unwrap();
    let se_end: i32 = second_elf[1].parse::<i32>().unwrap();
    let se_begin: i32 = second_elf[0].parse::<i32>().unwrap();

    if fe_end <= se_end && fe_end >= se_begin && fe_begin >= se_begin && fe_begin <= se_end {
        // end of first elf is smaller than end of second elf, but bigger than begin of second elf
        return true;
    } else if se_end <= fe_end  && se_end >= fe_begin && se_begin >= fe_begin && se_begin <= fe_end {
        return true;
    }
    false
}

fn overlaps(first_elf: Vec<&str>, second_elf: Vec<&str>) -> bool{
    let fe_end: i32 = first_elf[1].parse::<i32>().unwrap();
    let fe_begin: i32 = first_elf[0].parse::<i32>().unwrap();
    let se_end: i32 = second_elf[1].parse::<i32>().unwrap();
    let se_begin: i32 = second_elf[0].parse::<i32>().unwrap();

    if fe_end <= se_end && fe_end >= se_begin {
        return true;
    } else if fe_begin >= se_begin && fe_begin <= se_end {
        return true;
    } else if se_end <= fe_end && se_end >= fe_begin {
        return true;
    } else if se_begin >= fe_begin && se_begin <= fe_end {
        return true;
    }
    false
}

fn solve_part1(input: &str) -> u32 {
    let assignments = parse_input(input);
    let mut sum = 0;
    for line in assignments {
        let first_elf: Vec<&str> = line.0.split("-").collect();
        let second_elf: Vec<&str> = line.1.split("-").collect();
        if contains(first_elf, second_elf) {
            sum += 1;
        }
    }
    sum
}

fn solve_part2(input: &str) -> u32 {
    let assignments = parse_input(input);
    let mut sum = 0;
    for line in assignments {
        let first_elf: Vec<&str> = line.0.split("-").collect();
        let second_elf: Vec<&str> = line.1.split("-").collect();
        if overlaps(first_elf, second_elf) {
            sum += 1;
        }
    }
    sum
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/four/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/four/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let expected: u32 = 2;
        let output = solve_part1("src/four/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: u32 = 4;
        let output = solve_part2("src/four/sample-input.txt");
        assert_eq!(expected, output);
    }
}
