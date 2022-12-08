// #[aoc_generator(day8)]
// fn generator(input: &str) -> _ {}

#[aoc(day8, part1)]
fn part_1(input: &str) -> usize {
    let cells = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|byte| byte.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();
    let len = cells.len();
    let row_len = cells[0].len();
    let mut visible = 0;
    let mut maximums = Vec::new();
    let mut seen: Vec<Vec<i16>> = Vec::new();
    for i in 0..len {
        seen.push(Vec::new());
        let len = row_len;
        for _ in 0..len {
            seen[i].push(0);
        }
        maximums.push(-1);
    }
    for i in 0..len {
        for j in 0..row_len {
            if cells[i][j] > maximums[i] {
                maximums[i] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..len {
        maximums.push(-1);
    }
    for i in 0..len {
        for x in 0..row_len {
            let j = row_len - x - 1;
            if cells[i][j] > maximums[i] {
                maximums[i] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }

    maximums.clear();
    for _ in 0..row_len {
        maximums.push(-1);
    }
    for j in 0..row_len {
        for i in 0..len {
            if cells[i][j] > maximums[j] {
                maximums[j] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..row_len {
        maximums.push(-1);
    }
    for j in 0..row_len {
        for x in 0..len {
            let i = len - x - 1;
            if cells[i][j] > maximums[j] {
                maximums[j] = cells[i][j];
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
