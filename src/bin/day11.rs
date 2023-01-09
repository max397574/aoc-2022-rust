#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        return match self {
            Operation::Add(x) => x + old,
            Operation::Mul(x) => x * old,
            Operation::Square => old * old,
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct Monkey {
    operation: Operation,
    test: usize,
    success: usize,
    failure: usize,
}

#[derive(Clone, Copy, Debug)]
struct Item {
    value: usize,
    index: usize,
}

#[aoc_generator(day11)]
fn generator(input: &str) -> (Vec<Monkey>, Vec<Item>) {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut items: Vec<Item> = vec![];
    for monkey in input.split("\n\n") {
        let mut op = Operation::Square;
        let lines = monkey
            .lines()
            .filter_map(|l| l.split(':').last())
            .collect::<Vec<&str>>();
        let index = monkey
            .lines()
            .next()
            .unwrap()
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let items_vec: Vec<usize> = lines[1].trim().split(", ").flat_map(str::parse).collect();
        items.extend(items_vec.iter().map(|item| Item {
            value: *item,
            index,
        }));
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
        })
    }
    (monkeys, items)
}

fn get_items_for_monkey(monkey: usize, items: &Vec<Item>) -> Vec<Item> {
    let mut monkey_items = vec![];
    for item in items.iter() {
        if item.index == monkey {
            monkey_items.push(*item);
        }
    }
    monkey_items
}

#[aoc(day11, part1)]
fn part_1((monkeys, items): &(Vec<Monkey>, Vec<Item>)) -> usize {
    let mut items = (*items).clone();
    let mut handled_items = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let mut monkey_items = get_items_for_monkey(i, &items);
            for _ in 0..monkey_items.len() {
                monkey_items.reverse();
                let mut worry_lvl = monkey_items.last().unwrap().value;
                handled_items[i] += 1;
                worry_lvl = monkey.operation.apply(worry_lvl) / 3;
                let new_idx = if worry_lvl % monkey.test == 0 {
                    monkey.success
                } else {
                    monkey.failure
                };
                for item in items.iter_mut() {
                    if item.index == i && item.value == monkey_items.last().unwrap().value {
                        item.index = new_idx;
                    }
                }
            }
        }
    }
    handled_items.sort();

    let handled_items = &handled_items[handled_items.len() - 2..];

    handled_items[0] * handled_items[1]
}

#[aoc(day11, part2)]
fn part_2((monkeys, items): &(Vec<Monkey>, Vec<Item>)) -> usize {
    0
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
        assert_eq!(part_1(&generator(INPUT)), 10605)
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(&generator(INPUT)), 0)
    }
}
