use itertools::Itertools;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Direction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_U: Regex = Regex::new(r"U (\d+)").unwrap();
            static ref RE_D: Regex = Regex::new(r"D (\d+)").unwrap();
            static ref RE_L: Regex = Regex::new(r"L (\d+)").unwrap();
            static ref RE_R: Regex = Regex::new(r"R (\d+)").unwrap();
        }

        if let Some(up) = RE_U
            .captures(s)
            .map(|cap| Direction::Up(cap[1].parse().unwrap()))
        {
            Ok(up)
        } else if let Some(down) = RE_D
            .captures(s)
            .map(|cap| Direction::Down(cap[1].parse().unwrap()))
        {
            Ok(down)
        } else if let Some(left) = RE_L
            .captures(s)
            .map(|cap| Direction::Left(cap[1].parse().unwrap()))
        {
            Ok(left)
        } else if let Some(right) = RE_R
            .captures(s)
            .map(|cap| Direction::Right(cap[1].parse().unwrap()))
        {
            Ok(right)
        } else {
            println!("{}",s);
            panic!();
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rope {
    pub head: Head,
    pub knots: Vec<Knot>,
    pub length: i32,
}

impl Rope {
    fn update_knot(&mut self, pos: usize) {
        for movement in &self.head.movements {
            match movement {
                Direction::Down(d) => {
                    // add a movement to the knot
                    self.knots[pos].movements.push(Direction::Down(*d));
                    // move to the down
                    for _i in 0..*d {
                        self.head.y -= 1;
                        if (self.head.y - self.knots[0].y).abs() > self.length {
                            self.knots[0].y -= 1;
                            if self.knots[0].x != self.head.x {
                                self.knots[0].x = self.head.x;
                            }
                            let x = self.knots[0].x;
                            let y = self.knots[0].y;
                            self.knots[0].visited.push((x, y));
                        }
                    }
                },
                Direction::Up(u) => {
                    // add a movement to the knot
                    self.knots[pos].movements.push(Direction::Up(*u));
                    // move to the up
                    for _i in 0..*u {
                        self.head.y += 1;
                        if (self.head.y - self.knots[0].y).abs() > self.length {
                            self.knots[0].y += 1;
                            if self.knots[0].x != self.head.x {
                                self.knots[0].x = self.head.x;
                            }
                            let x = self.knots[0].x;
                            let y = self.knots[0].y;
                            self.knots[0].visited.push((x, y));
                        }
                    }
                },
                Direction::Left(l) => {
                    // add a movement to the knot
                    self.knots[pos].movements.push(Direction::Left(*l));
                    // move to the left
                    for _i in 0..*l {
                        self.head.x -= 1;
                        if (self.head.x - self.knots[0].x).abs() > self.length {
                            self.knots[0].x -= 1;
                            if self.knots[0].y != self.head.y {
                                self.knots[0].y = self.head.y;
                            }
                            let x = self.knots[0].x;
                            let y = self.knots[0].y;
                            self.knots[0].visited.push((x, y));
                        }
                    }
                },
                Direction::Right(r) => {
                    // add a movement to the knot
                    self.knots[pos].movements.push(Direction::Right(*r));
                    // move to the right
                    for _i in 0..*r {
                        self.head.x += 1;
                        if (self.head.x - self.knots[0].x).abs() > self.length {
                            self.knots[0].x += 1;
                            if self.knots[0].y != self.head.y {
                               self.knots[0].y = self.head.y;
                            }
                            let x = self.knots[0].x;
                            let y = self.knots[0].y;
                            self.knots[0].visited.push((x, y));
                        }
                    }
                },
            };
        }
    }
    fn amount_visited(&self) -> usize {
        let unique: Vec<_> = self.knots[0].visited.iter().unique().collect();
        return unique.len();
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Head {
    pub x: i32,
    pub y: i32,
    pub movements: Vec<Direction>
}
#[derive(Debug, Eq, PartialEq)]
struct Knot {
    pub x: i32,
    pub y: i32,
    pub movements: Vec<Direction>,
    pub visited: Vec<(i32, i32)>,
}

fn parse_input(input_file: &str) -> Rope {
    let binding = fs::read_to_string(input_file).unwrap();
    let input = binding.lines().collect::<Vec<&str>>();
    let head = Head {
        x: 0,
        y: 0,
        movements: Vec::new(),
    };
    let mut knot = Knot {
        x: 0,
        y: 0,
        movements: Vec::new(),
        visited: Vec::new(),
    };
    knot.visited.push((0,0));
    let mut rope = Rope {
        head: head,
        knots: vec![knot],
        length: 1,
    };
    let input: Vec<Direction> = input.iter().map(|line| Direction::from_str(line).unwrap()).collect::<Vec<_>>();
    rope.head.movements = input;
    rope
}

fn solve_part1(input: &str) -> u32 {
    let mut rope = parse_input(input);
    rope.update_knot(0);
    //println!("{:?}", rope.head);
    //println!("{:?}", rope.knot);
    rope.amount_visited() as u32
}

fn solve_part2(input: &str) -> u32 {
    let rope = parse_input(input);
    let mut max = 0;
    max
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/nine/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/nine/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_right_four_steps() {
        let mut initial = Rope {
            head : Head {
                x: 0,
                y: 0,
                movements: vec![Direction::Right(4)],
            },
            knots : vec![Knot {
                x: 0,
                y: 0,
                movements: Vec::new(),
                visited: vec![(0, 0)],
            }],
            length: 1,
        };
        initial.update_knot(0);
        let expected = Rope {
            head: Head {
                x: 4,
                y: 0,
                movements: vec![Direction::Right(4)],
            },
            knots : vec![Knot {
                x: 3,
                y: 0,
                movements: vec![Direction::Right(4)],
                visited: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            }],
            length: 1,
        };
        assert_eq!(expected, initial);
    }

    #[test]
    fn test_move_right_three_steps_with_knot_right_of_head() {
        let mut initial = Rope {
            head : Head {
                x: 0,
                y: 0,
                movements: vec![Direction::Right(3)],
            },
            knots : vec![Knot {
                x: 1,
                y: 0,
                movements: Vec::new(),
                visited: vec![(1, 0)],
            }],
            length: 1,
        };
        initial.update_knot(0);
        let expected = Rope {
            head: Head {
                x: 3,
                y: 0,
                movements: vec![Direction::Right(3)],
            },
            knots : vec![Knot {
                x: 2,
                y: 0,
                movements: vec![Direction::Right(3)],
                visited: vec![(1, 0), (2, 0)],
            }],
            length: 1,
        };
        assert_eq!(expected, initial);
    }

    #[test]
    fn test_move_right_three_steps_with_knot_above_head() {
        let mut initial = Rope {
            head : Head {
                x: 0,
                y: 0,
                movements: vec![Direction::Right(3)],
            },
            knots : vec![Knot {
                x: 0,
                y: 1,
                movements: Vec::new(),
                visited: vec![(0, 1)],
            }],
            length: 1,
        };
        initial.update_knot(0);
        let expected = Rope {
            head: Head {
                x: 3,
                y: 0,
                movements: vec![Direction::Right(3)],
            },
            knots : vec![Knot {
                x: 2,
                y: 0,
                movements: vec![Direction::Right(3)],
                visited: vec![(0, 1), (1, 0), (2, 0)],
            }],
            length: 1,
        };
        assert_eq!(expected, initial);
    }

    #[test]
    fn test_move_left_five_steps_with_knot_above_and_left_of_head() {
        let mut initial = Rope {
            head : Head {
                x: 5,
                y: 2,
                movements: vec![Direction::Left(5)],
            },
            knots : vec![Knot {
                x: 4,
                y: 3,
                movements: Vec::new(),
                visited: vec![(4, 3)],
            }],
            length: 1,
        };
        initial.update_knot(0);
        let expected = Rope {
            head: Head {
                x: 0,
                y: 2,
                movements: vec![Direction::Left(5)],
            },
            knots : vec![Knot {
                x: 1,
                y: 2,
                movements: vec![Direction::Left(5)],
                visited: vec![(4, 3), (3, 2), (2, 2), (1, 2)],
            }],
            length: 1,
        };
        assert_eq!(expected, initial);
    }

    #[test]
    fn test_sample_part1() {
        let expected: u32 = 13;
        let output = solve_part1("src/nine/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: u32 = 8;
        let output = solve_part2("src/nine/sample-input.txt");
        assert_eq!(expected, output);
    }
}
