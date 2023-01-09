fn part_1(input: &str) -> i32 {
    let mut result = 0;
    let mut register = 1;
    let mut cycles = 1;
    let mut to_add;
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["addx", value] => {
                if cycles % 40 == 20 || cycles == 20 {
                    result += cycles * register;
                }
                to_add = value.parse::<i32>().unwrap();
                cycles += 1;
                if cycles % 40 == 20 || cycles == 20 {
                    result += cycles * register;
                }
                cycles += 1;
                register += to_add;
            }
            ["noop"] => {
                if cycles % 40 == 20 || cycles == 20 {
                    result += cycles * register;
                }
                cycles += 1;
            }
            _ => {
                unreachable!()
            }
        }
    }
    result
}

fn part_2(input: &str) -> String {
    let mut register: i32 = 1;
    let mut cycles: i32 = 1;
    let mut to_add;
    let mut output = String::with_capacity(40 * 7);
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["addx", value] => {
                if ((cycles - 1) % 40- register).abs() <= 1{
                    output.push('#');
                } else {
                    output.push(' ')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }

                to_add = value.parse::<i32>().unwrap();
                cycles += 1;
                if ((cycles - 1) % 40- register).abs() <= 1{
                    output.push('#');
                } else {
                    output.push(' ')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }

                cycles += 1;
                register += to_add;
            }
            ["noop"] => {
                if ((cycles - 1) % 40- register).abs() <= 1{
                    output.push('#');
                } else {
                    output.push(' ')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }
                cycles += 1;
            }
            _ => {
                unreachable!()
            }
        }
    }
    output
}

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    let start = std::time::Instant::now();

    println!("part1: {}", part_1(&input));
    println!("part2:\n{}", part_2(&input));
    println!("time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13
addx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35
addx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21
addx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5
noop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2
addx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13
addx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop
addx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3
addx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3
addx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18
addx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1
addx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop
addx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 13140)
    }

    #[test]
    fn part2() {
        assert!(part_2(INPUT).eq(&String::from(
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     \n"
        )))
    }
}
