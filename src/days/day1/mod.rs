use crate::utils::Input;
use std::iter::zip;

fn parse(input: &str) -> Vec<i32> { input.lines().values() }

fn solve(xs: Vec<i32>, shift: usize) -> usize {
    zip(&xs[0..], &xs[shift..])
        .filter(|(current, next)| current < next)
        .count()
}

pub fn part1(input: &str) -> usize { solve(parse(input), 1) }

pub fn part2(input: &str) -> usize { solve(parse(input), 3) }


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 7) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 1559) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 5) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 1600) }
}
