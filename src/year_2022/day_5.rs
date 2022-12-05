#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

struct Supplies {
    stacks: Vec<Vec<u8>>,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Supplies {
    let (stacks_raw, instructions_raw) = input.split_once("\n\n").unwrap();
    let mut stacks = vec![];
    for line in stacks_raw.lines().rev().skip(1) {
        let bytes = line.as_bytes();
        let count = (bytes.len() + 1) / 4;
        if stacks.len() < count {
            for _ in 0..count {
                stacks.push(vec![]);
            }
        }
        for i in 0..count {
            let crate_type = bytes[i * 4 + 1];
            if crate_type != b' ' {
                stacks[i].push(crate_type);
            }
        }
    }
    let mut instructions = vec![];
    for line in instructions_raw.lines() {
        let [_, amount, _, from, _, to] = line.split_ascii_whitespace().collect::<Vec<_>>()[..] else {unreachable!("couldn't parse line")};
        instructions.push(Instruction {
            amount: amount.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        })
    }
    Supplies {
        stacks,
        instructions,
    }
}

#[aoc(day5, part1)]
fn part_1(input: &Supplies) -> String {
    let mut stacks = input.stacks.to_owned();
    for instruction in input.instructions.iter() {
        for _ in 0..instruction.amount {
            let top = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(top);
        }
    }
    let mut top_crates = vec![];
    for stack in stacks.iter_mut() {
        top_crates.push(stack.pop().unwrap());
    }
    String::from_utf8(top_crates).unwrap()
}

#[aoc(day5, part2)]
fn part_2(input: &Supplies) -> String {
    let mut stacks = input.stacks.to_owned();
    for instruction in input.instructions.iter() {
        let index = stacks[instruction.from - 1].len() - instruction.amount;
        let top = stacks[instruction.from - 1].split_off(index);
        stacks[instruction.to - 1].extend(top);
    }
    let mut top_crates = vec![];
    for stack in stacks.iter_mut() {
        top_crates.push(stack.pop().unwrap());
    }
    String::from_utf8(top_crates).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = generator("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\n move 1 from 1 to 2\n");
        assert_eq!(part_1(&input), String::from("CMZ"));
    }

    #[test]
    fn part2() {
        let input = generator("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\n move 1 from 1 to 2\n");
        assert_eq!(part_2(&input), String::from("MCD"));
    }
}
