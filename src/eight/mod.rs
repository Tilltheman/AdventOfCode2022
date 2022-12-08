use std::fs;

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

fn parse_input(input_file: &str) -> Vec<Vec<u32>> {
    let mut output: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
    let content = fs::read_to_string(input_file).unwrap();
    for line in content.lines() {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        output.push(row);
    }
    output
}

fn check_visible_in_direction(map: &Vec<Vec<u32>>, pos_x: usize, pos_y: usize, direction: Direction) -> bool {
    let len_x = map.len()-1;
    let len_y = map[0].len()-1;
    let elem = map[pos_x][pos_y];
    match direction {
        Direction::Top => {
            for x in (0..pos_x).rev() {
                if map[x][pos_y] >= elem{
                    return false;
                }
            }
            return true;
        },
        Direction::Bottom => {
            for x in pos_x+1..=len_x {
                if map[x][pos_y] >= elem {
                    return false;
                }
            }
            return true
        },
        Direction::Left => {
            for y in (0..pos_y).rev() {
                if map[pos_x][y] >= elem {
                    return false;
                }
            }
            return true;
        },
        Direction::Right => {
            for y in pos_y+1..=len_y {
                if map[pos_x][y] >= elem {
                    return false;
                }
            }
            return true;
        },
    }
}

fn count_visible_in_direction(map: &Vec<Vec<u32>>, pos_x: usize, pos_y: usize, direction: Direction) -> u32 {
    let len_x = map.len()-1;
    let len_y = map[0].len()-1;
    let elem = map[pos_x][pos_y];
    let mut count = 0;
    match direction {
        Direction::Top => {
            for x in (0..pos_x).rev() {
                count += 1;
                if map[x][pos_y] >= elem{
                    return count;
                }
            }
            return count;
        },
        Direction::Bottom => {
            for x in pos_x+1..=len_x {
                count += 1;
                if map[x][pos_y] >= elem {
                    return count;
                }
            }
            return count;
        },
        Direction::Left => {
            for y in (0..pos_y).rev() {
                count += 1;
                if map[pos_x][y] >= elem {
                    return count;
                }
            }
            return count
        },
        Direction::Right => {
            for y in pos_y+1..=len_y {
                count += 1;
                if map[pos_x][y] >= elem {
                    return count;
                }
            }
            return count;
        },
    }
}

fn count_trees_in_view(map: &Vec<Vec<u32>>, pos_x: usize, pos_y: usize) -> u32 {
    count_visible_in_direction(&map, pos_x, pos_y, Direction::Top)  *
        count_visible_in_direction(&map, pos_x, pos_y, Direction::Bottom) *
        count_visible_in_direction(&map, pos_x, pos_y, Direction::Left) *
        count_visible_in_direction(&map, pos_x, pos_y, Direction::Right)
}

fn is_visible(map: &Vec<Vec<u32>>, pos_x: usize, pos_y: usize) -> bool {
    if pos_x == 0 || pos_y == 0 {
        return true;
    }
    let len_x = map.len()-1;
    let len_y = map[0].len()-1;
    if pos_x == len_x || pos_y == len_y {
        return true;
    }
    check_visible_in_direction(&map, pos_x, pos_y, Direction::Top)  ||
        check_visible_in_direction(&map, pos_x, pos_y, Direction::Bottom) ||
        check_visible_in_direction(&map, pos_x, pos_y, Direction::Left) ||
        check_visible_in_direction(&map, pos_x, pos_y, Direction::Right)
}

fn solve_part1(input: &str) -> u32 {
    let map = parse_input(input);
    let len_x = map.len();
    let len_y = map[0].len();
    let mut sum = 0;
    for x in 0..len_x {
        for y in 0..len_y {
            if is_visible(&map, x, y) {
                sum += 1
            }
        }
    }
    sum
}

fn solve_part2(input: &str) -> u32 {
    let map = parse_input(input);
    let len_x = map.len();
    let len_y = map[0].len();
    let mut max = 0;
    for x in 0..len_x {
        for y in 0..len_y {
            let count = count_trees_in_view(&map, x, y);
            if count >= max {
                max = count;
            }
        }
    }
    max
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/eight/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/eight/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_visibility_1() {
        let expected: bool = true;
        let map = vec![vec![1,1],vec![1,1]];
        let output = is_visible(&map, 0, 0);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_basic_visibility_2() {
        let expected: bool = true;
        let map = vec![vec![1,1],vec![1,1]];
        let output = is_visible(&map, 1, 1);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_basic_visibility_3() {
        let expected: bool = true;
        let map = vec![vec![1,2,3],vec![4,9,5],vec![6,7,8]];
        let output = is_visible(&map, 1, 1);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_basic_visibility_not_visible() {
        let expected: bool = false;
        let map = vec![vec![1,2,3],vec![4,1,5],vec![6,7,8]];
        let output = is_visible(&map, 1, 1);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_top_right_one_not_visible() {
        let expected: bool = false;
        let map = parse_input("src/eight/sample-input.txt");
        let output = is_visible(&map, 1, 3);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_top_left_five_visible() {
        let expected: bool = true;
        let map = parse_input("src/eight/sample-input.txt");
        let output = is_visible(&map, 1, 1);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_top_mid_five_visible() {
        let expected: bool = true;
        let map = parse_input("src/eight/sample-input.txt");
        let output = is_visible(&map, 1, 2);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_top_mid_five_count() {
        let expected: u32 = 4;
        let map = parse_input("src/eight/sample-input.txt");
        let output = count_trees_in_view(&map, 1, 2);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part1() {
        let expected: u32 = 21;
        let output = solve_part1("src/eight/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: u32 = 8;
        let output = solve_part2("src/eight/sample-input.txt");
        assert_eq!(expected, output);
    }
}
