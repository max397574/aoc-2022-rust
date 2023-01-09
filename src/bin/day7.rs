use std::collections::HashMap;

fn generator(input: &str) -> HashMap<String, usize> {
    let mut cwd: Vec<String> = Vec::new();
    let mut dir_sizes = HashMap::new();
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                cwd = vec!["".to_string()];
            }
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", x] => {
                cwd.push(x.to_string());
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _] => {
                let mut dir_name = String::from("/");
                for dir in cwd.iter() {
                    dir_name.push('/');
                    dir_name.push_str(dir);
                    let entry = dir_sizes.entry(dir_name.clone()).or_insert(0);
                    *entry += size.parse::<usize>().unwrap();
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    dir_sizes
}

fn part_1(input: &HashMap<String, usize>) -> usize {
    input.values().filter(|x| x <= &&100000).sum()
}

fn part_2(input: &HashMap<String, usize>) -> usize {
    let needed = 40000000;
    let cur = input.get("//").unwrap();
    let mut min = 50000000;
    for &size in input.values() {
        if size >= cur - needed && size < min {
            min = size;
        }
    }
    min
}

fn main() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    let start = std::time::Instant::now();
    let generated = generator(&input);

    println!("part1: {}", part_1(&generated));
    println!("part2: {}", part_2(&generated));
    println!("time: {:?}", start.elapsed());
}
