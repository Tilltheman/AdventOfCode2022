fn solve_first() -> u32 {
    *create_sums().iter().max().unwrap()
}

fn solve_second() -> u32 {
    let mut sums = create_sums();
    let max_elf1 = *sums.iter().max().unwrap();
    sums.swap_remove(sums.iter().position(|value| *value == max_elf1).unwrap());
    let max_elf2 = *sums.iter().max().unwrap();
    sums.swap_remove(sums.iter().position(|value| *value == max_elf2).unwrap());
    let max_elf3 = *sums.iter().max().unwrap();
    sums.swap_remove(sums.iter().position(|value| *value == max_elf3).unwrap());
    max_elf1 + max_elf2 + max_elf3
}

fn create_sums() -> Vec<u32> {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut elf_sums = vec![];
    let mut sum: u32 = 0;
    for line in lines {
        if line == Some("").unwrap() {
            elf_sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    elf_sums
}


pub fn solve() {
    println!("Solution for first part: {}", solve_first());
    println!("Solution for second part: {}", solve_second());
}

