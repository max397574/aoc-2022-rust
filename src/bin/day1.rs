fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|set| {
            set.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let mut sorted = input
        .split("\n\n")
        .map(|set| {
            set.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    sorted.sort();
    let length = sorted.len();
    sorted.into_iter().skip(length - 3).sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let start = std::time::Instant::now();

    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
    println!("time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 45000);
    }
}
