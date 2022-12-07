use std::cmp::Ordering;
use std::fs;

#[derive(Debug)]
struct ExecutedCommand {
    pub command: String,
    pub results: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Eq, PartialEq)]
struct PathEntry {
    pub path: String,
    pub size: u32,
    pub t: EntryType,
}

impl Ord for PathEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for PathEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Filesystem {
    pub content: Vec<PathEntry>,
}


fn parse_input(input_file: &str) -> Filesystem {
    let content = fs::read_to_string(input_file).unwrap();
    let mut executed_commands: Vec<ExecutedCommand> = Vec::new();
    let mut last_command = 0;
    for line in content.lines() {
        if &line[..1] == "$" {
            let ec = ExecutedCommand {
                command : line.to_string(),
                results : Vec::<String>::new(),
            };
            executed_commands.push(ec);
            last_command += 1;
        } else {
            executed_commands[last_command-1].results.push(line.to_string());
        }
    }
    let mut current_dir = "/".to_string();
    let mut filesystem = Filesystem {
        content : Vec::new(),
    };
    for command in executed_commands {
        if command.command.starts_with("$ cd /") {
            let root = PathEntry {
                path: "/".to_string(),
                size: 0,
                t: EntryType::Dir,
            };
            filesystem.content.push(root);
        } else if command.command.starts_with("$ cd ..") {
            // directory up
            let split: Vec<&str> = current_dir.split("/").collect::<Vec<_>>();
            let len = split.len();
            let mut joined = split[..len-2].join("/");
            if ! joined.starts_with("/") {
                current_dir = "/".to_string();
                continue;
            }
            joined.push('/');
            current_dir = joined;
        } else if command.command.starts_with("$ cd ") {
            // visit directory thats following the "$ cd "
            current_dir.push_str((&command.command[5..]).clone());
            current_dir.push('/');
        } else if command.command == "$ ls" {
            // check the results of the command and add according dir entries
            for result in command.results {
                if result.starts_with("dir ") {
                    let mut name = (&result[4..]).clone().to_string();
                    name.push('/');
                    let mut path: String = current_dir.clone();
                    path.push_str(&name);
                    let dir = PathEntry {
                        size : 0,
                        path: path,
                        t: EntryType::Dir,
                    };
                    filesystem.content.push(dir);
                } else {
                    // create file in directory
                    let split: Vec<_> = result.split(" ").collect();
                    let mut path: String = current_dir.clone();
                    let name = split[1].to_string();
                    path.push_str(&name);
                    let size = split[0].parse::<u32>().unwrap();
                    let file = PathEntry {
                        path : path,
                        size : size,
                        t: EntryType::File,
                    };
                    filesystem.content.push(file);
                }
            }
        }
    }
//    println!("{:?}", filesystem);
    filesystem
}

fn solve_part1(input: &str) -> u32 {
    let filesystem = parse_input(input);
    let mut directories: Vec<PathEntry> = Vec::new();
    let mut files: Vec<PathEntry> = Vec::new();
    for path_entry in filesystem.content {
        if path_entry.t == EntryType::File {
            files.push(path_entry);
        } else {
            directories.push(path_entry);
        }
    }
    let mut res = 0;
    for mut d in directories {
        let mut sum = 0;
        for f in &files {
            if f.path.starts_with(&d.path) {
                sum += f.size;
            }
        }
        d.size = sum;
        if sum < 100000 {
            res += sum;
        }
    }
    res
}

fn solve_part2(input: &str) -> u32 {
    let filesystem = parse_input(input);
    let mut directories: Vec<PathEntry> = Vec::new();
    let mut files: Vec<PathEntry> = Vec::new();
    for path_entry in filesystem.content {
        if path_entry.t == EntryType::File {
            files.push(path_entry);
        } else {
            directories.push(path_entry);
        }
    }
    let filesystem_size = 70000000;
    let mut size_used = 0;
    let mut possible_delete: Vec<PathEntry> = Vec::new();
    let mut directories_after: Vec<PathEntry> = Vec::new();
    for mut d in directories {
        let mut sum = 0;
        for f in &files {
            if f.path.starts_with(&d.path) {
                sum += f.size;
            }
        }
        d.size = sum;
        directories_after.push(d);
    }
    let mut directories_after_2: Vec<PathEntry> = Vec::new();
    for d in directories_after {
        if d.path == "/" {
            size_used = d.size;
        }
        directories_after_2.push(d);
    }
    println!("{}", size_used);
    let needed_space = 30000000 - (filesystem_size - size_used);
    println!("{}", filesystem_size - size_used);
    println!("{}", needed_space);
    for d in directories_after_2 {
        if d.size >= needed_space {
            possible_delete.push(d);
        }
    }
    let res = possible_delete.iter().min().unwrap();
    println!("{}", res.size);
    res.size
}

pub fn solve() {
    println!("Solution for part 1 {}", solve_part1("src/seven/input.txt"));
    println!("Solution for part 2 {}", solve_part2("src/seven/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let expected: u32= 95437;
        let output = solve_part1("src/seven/sample-input.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_sample_part2() {
        let expected: u32 = 24933642;
        let output = solve_part2("src/seven/sample-input.txt");
        assert_eq!(expected, output);
    }
}
