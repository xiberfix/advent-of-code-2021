use std::str::FromStr;

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, n) = s.split_once(' ').ok_or("bad format")?;
        let n = n.parse().ok().ok_or("bad argument")?;
        match command {
            "up"      => Ok(Command::Up(n)),
            "down"    => Ok(Command::Down(n)),
            "forward" => Ok(Command::Forward(n)),
            _ => Err("bad command")
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect()
}


#[derive(Default)]
struct State1 {
    position: i32,
    depth: i32,
}

impl State1 {
    fn update(mut self, command: Command) -> Self {
        match command {
            Command::Up(n)      => { self.depth -= n }
            Command::Down(n)    => { self.depth += n }
            Command::Forward(n) => { self.position += n }
        }
        self
    }
}

pub fn part1(input: &str) -> i32 {
    let commands = parse(input);
    let result = commands.into_iter().fold(State1::default(), State1::update);
    result.position * result.depth
}


#[derive(Default)]
struct State2 {
    position: i32,
    depth: i32,
    aim: i32,
}

impl State2 {
    fn update(mut self, command: Command) -> Self {
        match command {
            Command::Up(n)      => { self.aim -= n }
            Command::Down(n)    => { self.aim += n }
            Command::Forward(n) => {
                self.position += n;
                self.depth += self.aim * n
            }
        }
        self
    }
}

pub fn part2(input: &str) -> i32 {
    let commands = parse(input);
    let result = commands.into_iter().fold(State2::default(), State2::update);
    result.position * result.depth
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 150) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 1488669) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 900) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 1176514794) }
}
