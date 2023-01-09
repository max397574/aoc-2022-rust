#[aoc(day3, part1)]
fn part_1(input: &str) -> usize {
    let lines = input.split('\n');
    let mut score = 0;
    for line in lines {
        let mut checked: Vec<char> = vec![];
        let length = line.len();
        let first = &line[..length / 2];
        let second = &line[length / 2..];
        for item in first.chars() {
            if second.contains(item) {
                if checked.contains(&item) {
                    break;
                }
                checked.push(item);
                if item.is_lowercase() {
                    score += (item as usize) - 96;
                } else {
                    score += (item as usize) - 38;
                }
            }
        }
    }
    score
}
#[aoc(day3, part2)]
fn part_2(input: &str) -> usize {
    let lines = input.split('\n');
    let mut score = 0;
    for line in lines.array_chunks::<3>() {
        for item in line[0].chars() {
            if line[1].contains(item) && line[2].contains(item) {
                if item.is_lowercase() {
                    score += (item as usize) - 96;
                } else {
                    score += (item as usize) - 38;
                }
                break;
            }
        }
    }
    score
}
