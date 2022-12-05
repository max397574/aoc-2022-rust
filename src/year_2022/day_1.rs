#[aoc(day1, part1)]
fn part_1(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut max = 0;
    let mut current: i32 = 0;
    for line in lines {
        if !line.is_empty() {
            current += line.parse::<i32>().expect("expected number");
        } else {
            if current > max {
                max = current;
            }
            current = 0
        }
    }
    max
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    let mut current: i32 = 0;
    for line in lines {
        if !line.is_empty() {
            current += line.parse::<i32>().expect("expected number");
        } else {
            if current > max1 {
                max3 = max2;
                max2 = max1;
                max1 = current;
            } else if current > max2 {
                max3 = max2;
                max2 = current
            } else if current > max3 {
                max3 = current;
            }
            current = 0
        }
    }
    max1 + max2 + max3
}
