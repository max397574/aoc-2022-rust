use std::collections::HashMap;

#[aoc(day7, part1)]
fn entry(input: &str) -> usize {
    let lines = input.split('\n');
    let mut cwd: Vec<String> = Vec::new();
    let mut dir_sizes = HashMap::new();
    for line in lines {
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
                    if !dir_sizes.contains_key(&dir_name) {
                        dir_sizes.insert(dir_name.clone(), size.parse::<usize>().unwrap());
                    } else {
                        dir_sizes.insert(
                            dir_name.clone(),
                            dir_sizes.get(&dir_name).unwrap() + size.parse::<usize>().unwrap(),
                        );
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    let mut sum = 0;
    for (_, size) in dir_sizes {
        if size <= 100000 {
            sum += size
        }
    }
    sum
}
