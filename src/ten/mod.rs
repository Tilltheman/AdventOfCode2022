use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    AddX(i32),
    Noop(),
}

#[derive(Debug, PartialEq, Eq)]
struct CPU {
    pub register_x: i32,
    instructions: VecDeque<Instruction>,
}

struct CRT {
    pub screen: Vec<char>,
}

impl fmt::Display for CRT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, elem) in self.screen.iter().enumerate() {
            if i % 40 == 0 {
                write!(f, "\n")?
            }
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

impl CRT {
    fn draw_pixels(&mut self, clock: &Clock) {
        for i in 0..240_i32 {
            let row = i / 40;
            let sprite_pos_mid = clock.signal_at_cycle[i as usize];
            if sprite_pos_mid + (row*40) == i || sprite_pos_mid + (row*40) + 1 == i || sprite_pos_mid + (row*40) - 1 == i {
                self.screen[i as usize] = '#';
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Clock {
    pub cycles: i32,
    pub check_points: i32,
    pub signal_strengths: Vec<i32>,
    pub signal_at_cycle: Vec<i32>,
}

impl CPU {
    fn compute(&mut self, clock: &mut Clock) {
        for instr in self.instructions.drain(..) {
            match instr {
                Instruction::AddX(x) => {
                    let current_register = self.register_x;
                    clock.update(2, current_register);
                    self.register_x += x;
                },
                Instruction::Noop() => {
                    let current_register = self.register_x;
                    clock.update(1, current_register)
                },
            }
        }
    }
}

impl Clock {
    fn update(&mut self, cycles: i32, reg: i32) {
        for _i in 0..cycles {
            self.cycles += 1;
            if 20 - self.cycles % 40 == 0 {
                self.signal_strengths.push(self.cycles * reg);
                self.check_points += 1;
            }
            self.signal_at_cycle.push(reg);
        }
    }
}


impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_A: Regex = Regex::new(r"addx ([-]?\d+)").unwrap();
            static ref RE_N: Regex = Regex::new(r"noop").unwrap();
        }

        if let Some(add) = RE_A
            .captures(s)
            .map(|cap| Instruction::AddX(cap[1].parse().unwrap()))
        {
            Ok(add)
        }
        else if let Some(noop) = RE_N
            .captures(s)
            .map(|_cap| Instruction::Noop())
        {
            Ok(noop)
        } else {
            println!("{}", s);
            panic!();
        }
    }
}

fn parse_input(input_file: &str) -> VecDeque<Instruction> {
    let full_string = fs::read_to_string(input_file).unwrap();
    let input = full_string.lines().collect::<Vec<&str>>();
    let instructions: VecDeque<Instruction> = input.iter().map(|line| Instruction::from_str(line).unwrap()).collect::<VecDeque<_>>();
    instructions
}

fn solve_part1(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut cpu = CPU {
        register_x : 1,
        instructions : instructions,
    };
    let mut clock = Clock {
        cycles : 0,
        check_points : 0,
        signal_strengths : vec![],
        signal_at_cycle : vec![],
    };
    cpu.compute(&mut clock);
    clock.signal_strengths.iter().sum()
}

fn solve_part2(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut cpu = CPU {
        register_x : 1,
        instructions : instructions,
    };
    let mut clock = Clock {
        cycles : 0,
        check_points : 0,
        signal_strengths : vec![],
        signal_at_cycle : vec![],
    };
    let mut crt = CRT {
        screen : vec!['.'; 240],
    };
    cpu.compute(&mut clock);
    crt.draw_pixels(&clock);
    println!("{}", crt);

    0
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/ten/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/ten/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let instructions = VecDeque::from([Instruction::Noop(), Instruction::AddX(3), Instruction::AddX(-5)]);
        let mut initial = CPU {
            register_x : 1,
            instructions: instructions
        };
        let mut clock = Clock {
            cycles : 0,
            check_points : 0,
            signal_strengths : vec![],
            signal_at_cycle : vec![],
        };
        let expected = CPU {
            register_x : -1,
            instructions : VecDeque::new(),
        };
        let expected_clock = Clock {
            cycles : 5,
            check_points : 0,
            signal_strengths: vec![],
            signal_at_cycle : vec![1,1,1,4,4],
        };
        initial.compute(&mut clock);
        assert_eq!(expected_clock, clock);
        assert_eq!(expected, initial)
    }

    #[test]
    fn test_sample_part1() {
        let expected: i32= 13140;
        let output = solve_part1("src/ten/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: i32 = 0;
        let output = solve_part2("src/ten/sample-input.txt");
        assert_eq!(expected, output);
    }
}
