#!/bin/zsh

# curl --cookie "session=xxx" "https://adventofcode.com/2022/day/$1/input"
day=$1
year=2022
template="fn part_1(input: &str) -> usize {\n
    0\n
}\n
\n
fn part_2(input: &str) -> usize {\n
    0\n
}\n
fn main() {
    let input = std::fs::read_to_string(\"input/day$day.txt\").unwrap();
    let start = std::time::Instant::now();

    println!(\"part1: {}\", part_1(&input));
    println!(\"part2: {}\", part_2(&input));
    println!(\"time: {:?}\", start.elapsed());
}
\n
#[cfg(test)]\n
mod tests {\n
    use super::*;\n
    const INPUT: &str = \"\";\n
\n
    #[test]\n
    fn part1() {\n
        assert_eq!(part_1(INPUT), 0)\n
    }\n
\n
    #[test]\n
    fn part2() {\n
        assert_eq!(part_2(INPUT), 0)\n
    }\n
}"
[ -e "./src/bin/day$day.rs" ] ||
    (
        echo $template > "./src/bin/day$day.rs" &&
        echo "Created template file"
    )
open "https://adventofcode.com/$year/day/$day"
