use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        match self {
            Operation::Add(x) => x + old,
            Operation::Mul(x) => x * old,
            Operation::Square => old * old,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    success: usize,
    failure: usize,
    inspections: usize,
}

fn generator(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey in input.split("\n\n") {
        let mut op = Operation::Square;
        let lines = monkey
            .lines()
            .filter_map(|l| l.split(':').last())
            .collect::<Vec<&str>>();
        let items_vec: VecDeque<usize> = lines[1].trim().split(", ").flat_map(str::parse).collect();
        let operation_parts: Vec<&str> = lines[2]
            .split(" = ")
            .last()
            .unwrap()
            .split_whitespace()
            .collect();
        if operation_parts[2] == "old" {
            op = Operation::Square;
        } else if operation_parts[1] == "*" {
            op = Operation::Mul(operation_parts[2].parse().unwrap());
        } else if operation_parts[1] == "+" {
            op = Operation::Add(operation_parts[2].parse().unwrap());
        }
        let test = lines[3]
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .expect("Invalid test")
            .parse()
            .unwrap();
        let success = lines[4]
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let failure = lines[5]
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            operation: op,
            test,
            success,
            failure,
            items: items_vec,
            inspections: 0,
        })
    }
    monkeys
}

fn round(monkeys: &mut [Monkey], callback: impl Fn(&usize) -> usize) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let mut monkey = &mut monkeys[i];
            let new = callback(&monkey.operation.apply(item));
            monkey.inspections += 1;
            if new % monkeys[i].test == 0 {
                monkeys[monkeys[i].success].items.push_back(new)
            } else {
                monkeys[monkeys[i].failure].items.push_back(new)
            }
        }
    }
}

fn part_1(monkeys: &mut [Monkey]) -> usize {
    for _ in 0..20 {
        round(monkeys, |x| *x / 3);
    }
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    inspections[monkeys.len() - 2..].iter().product()
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut c;
    while b != 0 {
        c = a;
        a = b;
        b = c % b;
    }
    a
}

fn single_lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn get_lcm(numbers: Vec<usize>) -> usize {
    let mut tmp_lcm = single_lcm(*numbers.first().unwrap(), *numbers.get(1).unwrap());
    for num in numbers.iter().skip(2) {
        tmp_lcm = single_lcm(tmp_lcm, *num);
    }
    tmp_lcm
}

fn part_2(monkeys: &mut [Monkey]) -> usize {
    let lcm = get_lcm(monkeys.iter().map(|monkey| monkey.test).collect::<Vec<_>>());
    for _ in 0..10000 {
        round(monkeys, |x| x % lcm);
    }
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    inspections[monkeys.len() - 2..].iter().product()
}

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    let start = std::time::Instant::now();
    let mut monkeys = generator(&input);

    println!("part1: {}", part_1(&mut monkeys));

    let mut monkeys = generator(&input);
    println!("part2: {}", part_2(&mut monkeys));
    println!("time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1() {
        assert_eq!(part_1(&mut generator(INPUT)), 10605)
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(&mut generator(INPUT)), 0)
    }
}
