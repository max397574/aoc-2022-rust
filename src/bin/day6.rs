fn part_1(input: &str) -> usize {
    let mut last = vec![];
    let mut count = 0;
    // TODO: rewrite with std::slice::Windows
    for char in input.bytes() {
        if last.len() < 4 {
            last.push(char)
        } else {
            last.remove(0);
            last.push(char);
        }
        count += 1;
        if last.len() == 4 {
            let new_vec = last.clone();
            let mut new_vec = new_vec.clone();
            new_vec.sort();
            new_vec.dedup();
            if new_vec.len() == 4 {
                return count;
            }
        }
    }
    return count;
}

fn part_2(input: &str) -> usize {
    let mut last = vec![];
    let mut count = 0;
    // TODO: rewrite with std::slice::Windows
    for char in input.bytes() {
        if last.len() < 14 {
            last.push(char)
        } else {
            last.remove(0);
            last.push(char);
        }
        count += 1;
        if last.len() == 14 {
            let new_vec = last.clone();
            let mut new_vec = new_vec.clone();
            new_vec.sort();
            new_vec.dedup();
            if new_vec.len() == 14 {
                return count;
            }
        }
    }
    return count;
}

fn main() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    let start = std::time::Instant::now();

    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
    println!("time: {:?}", start.elapsed());
}
