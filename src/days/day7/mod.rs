use crate::utils::Input;
use std::cmp::min;

fn parse(input: &str) -> Vec<i32> { input.trim().split(',').values() }

fn cost(xs: &Vec<i32>, pos0: i32, pos1: i32, fuel: fn(i32) -> i32) -> i32 {
    let cost0 = xs.iter().map(|x| fuel((x - pos0).abs())).sum();
    let cost1 = xs.iter().map(|x| fuel((x - pos1).abs())).sum();
    min(cost0, cost1)
}

pub fn part1(input: &str) -> i32 {
    let mut xs = parse(input);
    xs.sort();
    let mid = xs.len() / 2;
    cost(&xs, xs[mid], xs[mid - 1], |n| n)
}

pub fn part2(input: &str) -> i32 {
    let xs = parse(input);
    let average = xs.iter().sum::<i32>() / (xs.len() as i32);
    cost(&xs, average, average + 1, |n| n * (n + 1) / 2)
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 37) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 336721) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 168) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 91638945) }
}
