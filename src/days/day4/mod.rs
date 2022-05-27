use ndarray::{Array2};

type Number = i32;
type Turn = i32;
type Board = Array2<Number>;

const INF: i32 = 100; // maximum possible number & turn

struct Game {
    boards: Vec<Board>,
    numbers: Vec<Number>,
    turns: Vec<Turn>,
}

impl Game {
    fn parse(input: &str) -> (Vec<Board>, Vec<Number>) {
        let parts: Vec<_> = input.split("\n\n").collect();
        let numbers = parts[0]
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let boards = parts[1..].iter()
            .filter_map(|s| {
                let cells = s
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                Board::from_shape_vec((5, 5), cells).ok()
            })
            .collect();
        (boards, numbers)
    }

    fn invert(numbers: &Vec<Number>) -> Vec<Turn> {
        let mut turns = vec![INF as Turn; INF as usize];
        for (turn, &number) in numbers.iter().enumerate().rev() {
            turns[number as usize] = turn as Turn
        }
        turns
    }

    fn new(input: &str) -> Game {
        let (boards, numbers) = Game::parse(input);
        let turns = Game::invert(&numbers);
        Game { boards, numbers, turns }
    }

    fn all_turn<'a, It>(&'a self, numbers: It) -> Turn where It: IntoIterator<Item=&'a Number> {
        let ts = numbers.into_iter().map(|&n| self.turns[n as usize]);
        ts.max().unwrap_or(INF)
    }

    fn winning_turn(&self, board: &Board) -> Turn {
        let rs = board.rows().into_iter().map(|r| self.all_turn(r));
        let cs = board.columns().into_iter().map(|c| self.all_turn(c));
        rs.chain(cs).min().unwrap_or(INF)
    }

    fn score(&self, board: &Board, turn: Turn) -> i32 {
        let last: i32 = self.numbers[turn as usize];
        let unmarked: i32 = board.into_iter()
            .filter(|&&n| self.turns[n as usize] > turn)
            .sum();
        last * unmarked
    }
}

pub fn part1(input: &str) -> i32 {
    let game = Game::new(input);
    let (turn, board) = game.boards.iter()
        .map(|board| (game.winning_turn(board), board))
        .min_by_key(|&(t, _)| t).unwrap();
    game.score(board, turn)
}

pub fn part2(input: &str) -> i32 {
    let game = Game::new(input);
    let (turn, board) = game.boards.iter()
        .map(|board| (game.winning_turn(board), board))
        .max_by_key(|&(t, _)| t).unwrap();
    game.score(board, turn)
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_example() { assert_eq!(super::part1(EXAMPLE), 4512) }

    #[test]
    fn part1_input() { assert_eq!(super::part1(INPUT), 4662) }

    #[test]
    fn part2_example() { assert_eq!(super::part2(EXAMPLE), 1924) }

    #[test]
    fn part2_input() { assert_eq!(super::part2(INPUT), 12080) }
}
