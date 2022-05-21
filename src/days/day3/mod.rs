fn parse(input: &str) -> (usize, Vec<i32>) {
    let n = input.find('\n').unwrap_or(0);
    let xs = input
        .lines()
        .filter_map(|s| i32::from_str_radix(s, 2).ok())
        .collect();
    (n, xs)
}

fn criteria(xs: &Vec<i32>, pos: usize, inv: bool) -> i32 {
    let mask = 1 << pos;
    let count0 = xs.into_iter().filter(|&&x| x & mask == 0).count();
    let count1 = xs.len() - count0;
    let v = if count1 >= count0 { 1 } else { 0 };
    let v = if inv { 1 - v } else { v };
    v << pos
}


pub fn part1(input: &str) -> i32 {
    let (n, xs) = parse(input);

    let step = |acc: i32, pos: usize, inv: bool| -> i32 {
        let v = criteria(&xs, pos, inv);
        acc | v
    };

    let gamma = (0..n).rev().fold(0, |acc, pos| step(acc, pos, false));
    let epsilon = (0..n).rev().fold(0, |acc, pos| step(acc, pos, true));
    gamma * epsilon
}


pub fn part2(input: &str) -> i32 {
    let (n, xs) = parse(input);

    fn step(acc: Vec<i32>, pos: usize, inv: bool) -> Vec<i32> {
        if acc.len() == 1 { acc } else {
            let v = criteria(&acc, pos, inv);
            let mask = 1 << pos;
            acc.into_iter().filter(|&x| x & mask == v).collect()
        }
    }

    let o2 = (0..n).rev().fold(xs.clone(), |acc, pos| step(acc, pos, false));
    let co2 = (0..n).rev().fold(xs.clone(), |acc, pos| step(acc, pos, true));
    o2[0] * co2[0]
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 198) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 3958484) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 230) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 1613181) }
}
