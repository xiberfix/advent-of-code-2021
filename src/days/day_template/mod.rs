pub fn part1(_input: &str) -> usize { 0 }

pub fn part2(_input: &str) -> usize { 0 }


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 0) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 0) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 0) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 0) }
}
