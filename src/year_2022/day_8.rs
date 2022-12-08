// #[aoc_generator(day8)]
// fn generator(input: &str) -> _ {}

#[aoc(day8, part1)]
fn part_1(input: &str) -> usize {
    let vec = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|byte| byte.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();
    let mut visible = 0;
    let mut maximums = Vec::new();
    let mut seen: Vec<Vec<i16>> = Vec::new();
    for i in 0..vec.len() {
        seen.push(Vec::new());
        for _ in 0..vec[0].len() {
            seen[i].push(0);
        }
        maximums.push(-1);
    }
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            if vec[i][j] > maximums[i] {
                maximums[i] = vec[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..vec.len() {
        maximums.push(-1);
    }
    for i in 0..vec.len() {
        for x in 0..vec[0].len() {
            let j = vec[0].len() - x - 1;
            if vec[i][j] > maximums[i] {
                maximums[i] = vec[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }

    maximums.clear();
    for _ in 0..vec[0].len() {
        maximums.push(-1);
    }
    for j in 0..vec[0].len() {
        for i in 0..vec.len() {
            if vec[i][j] > maximums[j] {
                maximums[j] = vec[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..vec[0].len() {
        maximums.push(-1);
    }
    for j in 0..vec[0].len() {
        for x in 0..vec.len() {
            let i = vec.len() - x - 1;
            if vec[i][j] > maximums[j] {
                maximums[j] = vec[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    visible
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 21);
    }
}
