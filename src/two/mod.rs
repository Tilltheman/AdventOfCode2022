fn parse_input(input_str: &str) -> Vec<(&str, &str)> {
    let input: &str;
    let mut output: Vec<(&str, &str)> = Vec::<(&str, &str)>::new();
    if input_str.is_empty() {
        input = include_str!("input.txt");
    } else {
        input = input_str;
    }
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let tuple = (parts[0], parts[1]);
        output.push(tuple);
    }
    output
}

fn solve_part1() -> u32 {
    let data = parse_input("");
    let mut sum: u32 = 0;

    for tuple in data {
        sum += match tuple {
            ("A", "X") => 1 + 3, // draw
            ("A", "Y") => 2 + 6, // win
            ("A", "Z") => 3 + 0, // loss
            ("B", "X") => 1 + 0, // loss
            ("B", "Y") => 2 + 3, // draw
            ("B", "Z") => 3 + 6, // win
            ("C", "X") => 1 + 6, // win
            ("C", "Y") => 2 + 0, // loss
            ("C", "Z") => 3 + 3, // draw
            (&_, _) => todo!()
        }
    }
    sum
}

fn solve_part2() -> u32 {
    let data = parse_input("");
    let mut sum: u32 = 0;

    for tuple in data {
        sum += match tuple {
            ("A", "X") => 0 + 3, // lose -> pick Scissors
            ("A", "Y") => 3 + 1, // draw -> pick Rock
            ("A", "Z") => 6 + 2, // win -> pick Paper
            ("B", "X") => 0 + 1, // lose -> pick Rock
            ("B", "Y") => 3 + 2, // draw -> pick Paper
            ("B", "Z") => 6 + 3, // win -> pick Scissors
            ("C", "X") => 0 + 2, // lose -> pick Paper
            ("C", "Y") => 3 + 3, // draw -> pick Scissors
            ("C", "Z") => 6 + 1, // win -> pick Rock
            (&_, _) => todo!()
        }
    }
    sum
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1());
    println!("Solution for part 2 {}", solve_part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = "A Z\nB X\n";
        let expected = vec![("A", "Z"), ("B", "X")];
        let output = parse_input(input);
        assert_eq!(expected, output)
    }
}
