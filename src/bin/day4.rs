use regex::Regex;

#[aoc(day4, part1)]
fn part_1(input: &str) -> usize {
    let lines = input.split('\n');
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

#[aoc(day4, part2)]
fn part_2(input: &str) -> usize {
    let lines = input.split('\n');
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
