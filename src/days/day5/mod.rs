use crate::utils::Input;
use std::cmp::max;
use std::str::FromStr;
use nalgebra::Vector2;
use ndarray::Array2;

type Pos = Vector2<i32>;
type Board = Array2<i32>;

struct Line {
    a: Pos,
    b: Pos,
}

fn parse_pos(s: &str) -> Option<Pos> {
    let (sx, sy) = s.split_once(',')?;
    let x = sx.parse().ok()?;
    let y = sy.parse().ok()?;
    Some(Pos::new(x, y))
}

fn parse_line(s: &str) -> Option<Line> {
    let (sa, sb) = s.split_once(" -> ")?;
    let a = parse_pos(sa)?;
    let b = parse_pos(sb)?;
    Some(Line { a, b })
}

fn parse(input: &str) -> Vec<Line> { input.lines().values() }

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_line(s).ok_or("bad format")
    }
}

impl Line {
    fn direction(&self) -> Pos {
        let d = self.b - self.a;
        Pos::new(d.x.signum(), d.y.signum())
    }

    fn length(&self) -> i32 {
        let d = self.b - self.a;
        d.abs().max()
    }

    fn is_horizontal(&self) -> bool { self.a.x == self.b.x }
    fn is_vertical(&self) -> bool { self.a.y == self.b.y }
    fn is_diagonal(&self) -> bool { !self.is_horizontal() && !self.is_vertical() }
}

fn create_board(lines: &Vec<Line>) -> Board {
    let max_x = lines.iter().map(|l| max(l.a.x, l.b.x)).max().unwrap_or(0) as usize;
    let max_y = lines.iter().map(|l| max(l.a.y, l.b.y)).max().unwrap_or(0) as usize;
    Board::default((max_x + 1, max_y + 1))
}

fn add_line(board: &mut Board, line: &Line) {
    for k in 0..=line.length() {
        let p = line.a + k * line.direction();
        board[[p.x as usize, p.y as usize]] += 1;
    }
}

pub fn part1(input: &str) -> usize {
    let lines = parse(input);
    let mut board = create_board(&lines);
    for line in lines.iter().filter(|&l| !l.is_diagonal()) {
        add_line(&mut board, line);
    }
    board.into_iter().filter(|&x| x > 1).count()
}

pub fn part2(input: &str) -> usize {
    let lines = parse(input);
    let mut board = create_board(&lines);
    for line in lines.iter() {
        add_line(&mut board, line);
    }
    board.into_iter().filter(|&x| x > 1).count()
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 5) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 6225) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 12) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 22116) }
}
