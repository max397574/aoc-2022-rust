use std::collections::HashMap;

#[aoc(day9, part1)]
fn part_1(input: &str) -> usize {
    let mut positions = Vec::new();
    positions.push((0, 0));
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;
    for line in input.split('\n') {
        let [direction,amount]=line.split_whitespace().collect::<Vec<_>>()[..] else {unreachable!()};
        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => hy += 1,
                "D" => hy -= 1,
                "R" => hx += 1,
                "L" => hx -= 1,
                _ => unreachable!(),
            }
            if (ty - hy).abs().max((tx - hx).abs()) <= 1 {
                continue;
            }
            if tx == hx && ty < hy {
                ty += 1;
            }
            if tx == hx && ty > hy {
                ty -= 1;
            }
            if ty == hy && tx < hx {
                tx += 1;
            }
            if ty == hy && tx > hx {
                tx -= 1;
            }
            if tx < hx && ty < hy {
                tx += 1;
                ty += 1;
            }
            if tx < hx && ty > hy {
                tx += 1;
                ty -= 1;
            }
            if tx > hx && ty < hy {
                tx -= 1;
                ty += 1;
            }
            if tx > hx && ty > hy {
                tx -= 1;
                ty -= 1;
            }
            if !positions.contains(&(tx, ty)) {
                positions.push((tx, ty));
            }
        }
    }
    positions.len()
}

fn catch_up(mut tx: i32, mut ty: i32, hx: i32, hy: i32) -> (i32, i32) {
    if (ty - hy).abs().max((tx - hx).abs()) <= 1 {
        return (tx, ty);
    }
    if tx == hx && ty < hy {
        ty += 1;
    }
    if tx == hx && ty > hy {
        ty -= 1;
    }
    if ty == hy && tx < hx {
        tx += 1;
    }
    if ty == hy && tx > hx {
        tx -= 1;
    }
    if tx < hx && ty < hy {
        tx += 1;
        ty += 1;
    }
    if tx < hx && ty > hy {
        tx += 1;
        ty -= 1;
    }
    if tx > hx && ty < hy {
        tx -= 1;
        ty += 1;
    }
    if tx > hx && ty > hy {
        tx -= 1;
        ty -= 1;
    }
    (tx, ty)
}

#[aoc(day9, part2)]
fn part_2(input: &str) -> usize {
    let mut positions = Vec::new();
    positions.push((0, 0));
    let mut hx: i32;
    let mut hy: i32;
    let mut tx: i32;
    let mut ty: i32;
    let mut tail_positions = HashMap::new();
    for i in 0..10 {
        tail_positions.insert(i, (0, 0));
    }
    for line in input.split('\n') {
        let [direction,amount]=line.split_whitespace().collect::<Vec<_>>()[..] else {unreachable!()};
        for _ in 0..amount.parse().unwrap() {
            (hx, hy) = *tail_positions.get(&0).unwrap();
            match direction {
                "U" => hy += 1,
                "D" => hy -= 1,
                "R" => hx += 1,
                "L" => hx -= 1,
                _ => unreachable!(),
            }
            tail_positions.insert(0, (hx, hy));
            for i in 0..9 {
                (hx, hy) = *tail_positions.get(&i).unwrap();
                (tx, ty) = *tail_positions.get(&(i + 1)).unwrap();
                let new_tail = catch_up(tx, ty, hx, hy);
                tail_positions.insert(i + 1, new_tail);
            }
            let last_tail = *tail_positions.get(&9).unwrap();
            if !positions.contains(&last_tail) {
                positions.push(last_tail);
            }
        }
    }
    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const INPUT2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1), 13)
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2), 36)
    }
}
