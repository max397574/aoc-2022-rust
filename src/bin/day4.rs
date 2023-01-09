use regex::Regex;

fn part_1(input: &str) -> usize {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut amount = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        // TODO: rewrite like this https://github.com/orlp/aoc2022/blob/fd9b8157e5fdd1a9acc4d1df9dafefdc552fe42c/src/bin/day04.rs#L14
        // TODO: you could use captures[1] for example.
        if (captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
            <= captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
            && captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
                >= captures.get(4).unwrap().as_str().parse::<i32>().unwrap())
            || (captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
                <= captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                && captures.get(4).unwrap().as_str().parse::<i32>().unwrap()
                    >= captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
        {
            amount += 1;
        }
    }
    amount
}

fn part_2(input: &str) -> usize {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut amount = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        if (captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
            >= captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
            && captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                <= captures.get(3).unwrap().as_str().parse::<i32>().unwrap())
            || (captures.get(4).unwrap().as_str().parse::<i32>().unwrap()
                >= captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                && captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
                    <= captures.get(1).unwrap().as_str().parse::<i32>().unwrap())
        {
            amount += 1;
        }
    }
    amount
}

fn main() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();
    let start = std::time::Instant::now();

    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
    println!("time: {:?}", start.elapsed());
}
