#!/bin/zsh
day=$(date +%-d)
year=$(date +%Y)
template="#[aoc_generator(day$day)]
fn generator(input: &str) -> _ {
}
#[aoc(day$day, part1)]
fn entry(input: &_) -> usize {
    0
}
"
[ -e "./input/$year/day$day.txt" ] || cargo aoc input
[ -e "./src/year_$year/day_$day.rs" ] ||
    (
        echo $template > "./src/year_$year/day_$day.rs" &&
        echo "mod day_$day;" >> "./src/year_$year/mod.rs"  &&
        echo "Created template file"
    )
open "https://adventofcode.com/$year/day/$day"
