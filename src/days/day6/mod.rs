use crate::utils::Input;

fn parse(input: &str) -> Vec<usize> { input.trim().split(',').values() }

fn solve(ages: Vec<usize>, days: usize) -> usize {
    let mut counts = [0; 9];
    for age in ages { counts[age] += 1 }
    for day in 0..days { counts[(day + 7) % 9] += counts[day % 9] }
    counts.iter().sum()
}

pub fn part1(input: &str) -> usize { solve(parse(input), 80) }

pub fn part2(input: &str) -> usize { solve(parse(input), 256) }


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 5934) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 362346) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 26984457539) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 1639643057051) }
}
