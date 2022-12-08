#[aoc(day2, part1)]
fn part_1(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut score = 0;
    for line in lines {
        let (opponent, player): (i32, i32) = match line {
            "A X" => (1, 1),
            "A Y" => (1, 2),
            "A Z" => (1, 3),
            "B X" => (2, 1),
            "B Y" => (2, 2),
            "B Z" => (2, 3),
            "C X" => (3, 1),
            "C Y" => (3, 2),
            "C Z" => (3, 3),
            _ => (0, 0),
        };
        score += player;
        if opponent == player {
            score += 3;
        } else if (player - opponent) == 1 || (player == 1 && opponent == 3) {
            score += 6;
        }
    }
    score
}

#[aoc(day2, part2)]
fn part_2(input: &str) -> usize {
    let lines = input.split('\n');
    let mut score = 0;
    for line in lines {
        let (opponent, result) = match line {
            "A X" => (1, 1),
            "A Y" => (1, 2),
            "A Z" => (1, 3),
            "B X" => (2, 1),
            "B Y" => (2, 2),
            "B Z" => (2, 3),
            "C X" => (3, 1),
            "C Y" => (3, 2),
            "C Z" => (3, 3),
            _ => (0, 0),
        };
        score += (result - 1) * 3;
        // you have to loose
        if result == 1 {
            if opponent == 1 {
                score += 3;
            } else {
                score += opponent - 1;
            }
        // you have to draw
        } else if result == 2 {
            // take same as opponent
            score += opponent;
        // you have to win
        } else if opponent == 3 {
            score += 1;
        } else {
            score += opponent + 1;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 12);
    }
}
