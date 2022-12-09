#!/bin/zsh
day=$1
year=2022
template="#[aoc(day$1, part1)]\n
fn part_1(input: &str) -> usize {\n
    0\n
}\n
\n
#[aoc(day$1, part2)]\n
fn part_2(input: &str) -> usize {\n
    0\n
}\n
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
[ -e "./input/$year/day$day.txt" ] || cargo aoc input -d $day -y $year
[ -e "./src/year_$year/day_$day.rs" ] ||
    (
        echo $template > "./src/year_$year/day_$day.rs" &&
        echo "mod day_$day;" >> "./src/year_$year/mod.rs"  &&
        echo "Created template file"
    )
open "https://adventofcode.com/$year/day/$day"
