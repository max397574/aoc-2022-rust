pub fn grid<T>(text_block: &str, callback: impl Fn(char) -> T) -> Vec<Vec<T>> {
    text_block
        .lines()
        .map(|line| line.chars().map(&callback).collect())
        .collect()
}

pub fn gcd(a: usize, b: usize) -> usize {
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

pub fn single_lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn get_lcm(numbers: Vec<usize>) -> usize {
    let mut tmp_lcm = single_lcm(*numbers.first().unwrap(), *numbers.get(1).unwrap());
    for num in numbers.iter().skip(2) {
        tmp_lcm = single_lcm(tmp_lcm, *num);
    }
    tmp_lcm
}
