fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    let start = std::time::Instant::now();

    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
    println!("time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 0)
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 0)
    }
}
