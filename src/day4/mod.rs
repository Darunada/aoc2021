use std::fmt::{Debug, Display, Formatter};
use std::fs;

use std::str::{Lines};

pub fn run() {
    let part1_file = "src/day4/input.txt";

    let contents = fs::read_to_string(part1_file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let (calls, bingo_boards) = parse_input(&lines, 5);
    let tournament = Tournament(bingo_boards.clone());

    let mut tournament1 = tournament.clone();

    for call in calls.clone() {
        let winners = tournament1.call(call);
        if winners > 0 {
            tournament1.winners()
                .iter()
                .for_each(|b| {
                    //println!("{}", b);
                    println!("part 1: {}", b.score().unwrap());
                });
            break;
        }
    }

    let mut tournament2 = tournament;
    for call in calls.clone() {
        let winners = tournament2.call(call);

        if winners == bingo_boards.len() as u32 {
            let mut winning_boards = tournament2.winners();
            winning_boards.sort_by(|a, b| a.calls.cmp(&b.calls));


            let last_winner = winning_boards.pop().unwrap();
            //println!("{}", last_winner);
            println!("part 2: {}", last_winner.score().unwrap());
            break;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BingoCell(u32, bool);

impl BingoCell {
    fn from(value: u32) -> BingoCell {
        BingoCell(value, false)
    }

    fn is_marked(&self) -> bool {
        self.1
    }

    fn call(&mut self, value: u32) -> bool {
        if self.0 == value && !self.1 {
            self.1 = true;
            return true;
        }
        false
    }
}

impl Display for BingoCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.1 {
            write!(f, "({}) ", self.0)
        } else {
            write!(f, " {}  ", self.0)
        }
    }
}

#[derive(Debug, Clone, Default)]
struct BingoBoard {
    calls: u32,
    winning_number: Option<u32>,
    score: Option<u32>,
    row_width: u32,
    col_height: u32,
    board: Vec<BingoCell>,
}

impl BingoBoard {
    fn from(board: Vec<u32>, row_width: u32, col_height: u32) -> BingoBoard {
        let cells = board.into_iter()
            .map(BingoCell::from)
            .collect();

        BingoBoard {
            row_width,
            col_height,
            board: cells,
            ..BingoBoard::default()
        }
    }

    fn call(&mut self, value: u32) -> u32 {
        let mut total = 0;
        if self.winning_number.is_none() {
            self.calls += 1;
            self.board.iter_mut().for_each(|cell| {
                let marked = cell.call(value);
                if marked {
                    total += 1
                }
            });

            if self.has_won() {
                self.winning_number = Some(value);
            }
        }
        total
    }

    fn has_won(&self) -> bool {
        // check for full row
        for row in 0..self.col_height {
            let mut marked = 0u32;
            let index: usize = (self.row_width * row) as usize;

            for col in 0..self.row_width {
                if self.board.get(index + (col as usize)).unwrap().is_marked() {
                    marked += 1;
                }
            }

            if marked == self.row_width {
                return true;
            }
        }

        // check for full col
        for col in 0..self.row_width {
            let mut marked = 0u32;
            for row in 0..self.col_height {
                let index: usize = (col + self.row_width * row) as usize;
                if self.board.get(index).unwrap().is_marked() {
                    marked += 1;
                }
            }

            if marked == self.col_height {
                return true;
            }
        }

        false
    }

    fn score(&self) -> Result<u32, String> {
        if self.winning_number.is_none() {
            return Err("You must win before calculating score".to_string());
        }

        // sum unmarked cells
        let unmarked: u32 = self.board
            .iter()
            .filter(|&cell| !cell.is_marked())
            .map(|cell| cell.0)
            .sum();

        Ok(unmarked * self.winning_number.unwrap())
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0u32..self.board.len() as u32 {
            if i % self.row_width == 0 {
                writeln!(f).unwrap();
            }
            write!(f, "{}", self.board.get(i as usize).unwrap()).unwrap();
        }
        writeln!(f)
    }
}

#[derive(Debug, Clone)]
struct Tournament(Vec<BingoBoard>);

impl Tournament {
    fn call(&mut self, value: u32) -> u32 {
        self.0
            .iter_mut()
            .for_each(|board| {
                board.call(value);
            });

        self.0
            .iter()
            .filter(|board| board.has_won())
            .count() as u32
    }

    fn winners(&self) -> Vec<BingoBoard> {
        self.0
            .iter()
            .filter(|board| board.has_won()).cloned()
            .collect()
    }
}

fn parse_input(input: &Lines, board_size: u32) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut input = input.clone().peekable();
    let calls = input.next().unwrap().split(',').map(|i| i.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let _whitespace = input.next().unwrap();

    let mut boards: Vec<BingoBoard> = vec![];
    loop {
        let mut board: Vec<Vec<u32>> = vec![];
        for _ in 0..board_size {
            board.push(input.next().unwrap().split_ascii_whitespace().map(|i| i.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        }

        let bingo_board = BingoBoard::from(board.concat(), board_size, board_size);
        boards.push(bingo_board.clone());

        let _whitespace = input.next();

        if input.peek() == None {
            break;
        }
    }

    (calls, boards)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day4::{BingoBoard, BingoCell, parse_input, Tournament};

    #[test]
    fn bingo_cell_starts_unmarked() {
        let cell = BingoCell::from(23);
        assert!(!cell.is_marked());
    }

    #[test]
    fn bingo_cell_marked_when_its_called() {
        let mut cell = BingoCell::from(11);
        let marked = cell.call(11);
        assert!(marked);
        assert!(cell.is_marked());
    }

    #[test]
    fn bingo_cell_marked_only_once() {
        let mut cell = BingoCell::from(77);
        cell.call(77);
        let marked = cell.call(77);
        assert!(!marked);
        assert!(cell.is_marked());
    }

    #[test]
    fn bingo_board_marks_cells() {
        let mut board = BingoBoard {
            row_width: 1,
            col_height: 1,
            board: vec![BingoCell::from(3)],
            ..BingoBoard::default()
        };

        let marked = board.call(3);
        assert_eq!(marked, 1);
        assert!(board.board.get(0).unwrap().is_marked());
    }

    #[test]
    fn bingo_board_marks_all_cells_that_match() {
        let mut board = BingoBoard {
            row_width: 5,
            col_height: 1,
            board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(5), BingoCell::from(5)],
            ..BingoBoard::default()
        };

        let marked = board.call(5);
        assert_eq!(marked, 2);
        assert!(board.board.get(3).unwrap().is_marked());
        assert!(board.board.get(4).unwrap().is_marked());
    }

    #[test]
    fn bingo_board_checks_for_wins() {
        let mut board = BingoBoard {
            row_width: 2,
            col_height: 2,

            // 1 2
            // 3 4
            board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4)],
            ..BingoBoard::default()
        };

        assert!(!board.has_won());
        board.call(1);
        board.call(4);
        assert!(!board.has_won());
    }

    #[test]
    fn bingo_board_tracks_winning_call() {
        let mut board = BingoBoard {
            row_width: 2,
            col_height: 2,

            // 1 2
            // 3 4
            board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4)],
            ..BingoBoard::default()
        };

        board.call(1);
        assert_eq!(board.winning_number, None);
        board.call(2);
        assert_eq!(board.winning_number, Some(2));
        board.call(3);
        assert_eq!(board.winning_number, Some(2));
    }


    #[test]
    fn bingo_board_checks_for_a_winning_row() {
        let mut board = BingoBoard {
            row_width: 2,
            col_height: 2,

            // 1 2
            // 3 4
            board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4)],
            ..BingoBoard::default()
        };

        board.call(1);
        board.call(2);
        assert!(board.has_won());
    }

    #[test]
    fn bingo_board_checks_for_a_winning_col() {
        let mut board = BingoBoard {
            row_width: 2,
            col_height: 2,

            // 1 2
            // 3 4
            board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4)],
            ..BingoBoard::default()
        };

        board.call(2);
        board.call(4);
        assert!(board.has_won());
    }

    #[test]
    fn bingo_board_checks_larger_boards() {
        let board = BingoBoard {
            row_width: 5,
            col_height: 5,

            //  1  2  3  4  5
            //  6  7  8  9 10
            // 11 12 13 14 15
            // 16 17 18 19 20
            // 21 22 23 24 25
            board: vec![
                BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4), BingoCell::from(5),
                BingoCell::from(6), BingoCell::from(7), BingoCell::from(8), BingoCell::from(9), BingoCell::from(10),
                BingoCell::from(11), BingoCell::from(12), BingoCell::from(13), BingoCell::from(14), BingoCell::from(15),
                BingoCell::from(16), BingoCell::from(17), BingoCell::from(18), BingoCell::from(19), BingoCell::from(20),
                BingoCell::from(21), BingoCell::from(22), BingoCell::from(23), BingoCell::from(24), BingoCell::from(25),
            ],
            ..BingoBoard::default()
        };
        assert!(!board.has_won());

        let mut board1 = board.clone();
        board1.call(2);
        board1.call(7);
        board1.call(12);
        board1.call(17);
        board1.call(22);
        assert!(board1.has_won());

        let mut board2 = board.clone();
        board2.call(21);
        board2.call(22);
        board2.call(23);
        board2.call(24);
        board2.call(25);
        assert!(board2.has_won());

        let mut board3 = board;
        board3.call(1);
        board3.call(3);
        board3.call(5);
        board3.call(7);
        board3.call(9);
        board3.call(11);
        board3.call(13);
        board3.call(15);
        board3.call(17);
        board3.call(19);
        board3.call(21);
        board3.call(23);
        board3.call(25);
        assert!(!board3.has_won());
    }

    #[test]
    fn bingo_board_gets_score() {
        let mut board = BingoBoard {
            row_width: 5,
            col_height: 5,

            // 14 21 17 24  4
            // 10 16 15  9 19
            // 18  8 23 26 20
            // 22 11 13  6  5
            //  2  0 12  3  7
            board: vec![
                BingoCell::from(14), BingoCell::from(21), BingoCell::from(17), BingoCell::from(24), BingoCell::from(4),
                BingoCell::from(10), BingoCell::from(16), BingoCell::from(15), BingoCell::from(9), BingoCell::from(19),
                BingoCell::from(18), BingoCell::from(8), BingoCell::from(23), BingoCell::from(26), BingoCell::from(20),
                BingoCell::from(22), BingoCell::from(11), BingoCell::from(13), BingoCell::from(6), BingoCell::from(5),
                BingoCell::from(2), BingoCell::from(0), BingoCell::from(12), BingoCell::from(3), BingoCell::from(7),
            ],
            ..BingoBoard::default()
        };

        board.call(7);
        board.call(4);
        board.call(9);
        board.call(5);
        board.call(11);
        board.call(17);
        board.call(23);
        board.call(2);
        board.call(0);
        board.call(14);
        board.call(21);
        board.call(24);
        assert_eq!(board.winning_number, Some(24));
        assert_eq!(board.score(), Ok(4512));
    }

    #[test]
    fn tournament_calls_each_board() {
        let mut tournament = Tournament(vec![
            BingoBoard {
                row_width: 2,
                col_height: 2,

                // 1 2
                // 3 4
                board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4)],
                ..BingoBoard::default()
            },
            BingoBoard {
                row_width: 2,
                col_height: 2,

                // 2 3
                // 4 1
                board: vec![BingoCell::from(2), BingoCell::from(3), BingoCell::from(4), BingoCell::from(1)],
                ..BingoBoard::default()
            },
        ]);

        tournament.call(1);
        assert!(tournament.0.get(0).unwrap().board.get(0).unwrap().is_marked());
        assert!(tournament.0.get(1).unwrap().board.get(3).unwrap().is_marked());
    }

    #[test]
    fn tournament_gets_winners_after_each_call() {
        let mut tournament = Tournament(vec![
            BingoBoard {
                row_width: 2,
                col_height: 2,

                // 1 2
                // 3 4
                board: vec![BingoCell::from(1), BingoCell::from(2), BingoCell::from(3), BingoCell::from(4)],
                ..BingoBoard::default()
            },
            BingoBoard {
                row_width: 2,
                col_height: 2,

                // 2 3
                // 4 1
                board: vec![BingoCell::from(2), BingoCell::from(3), BingoCell::from(4), BingoCell::from(1)],
                ..BingoBoard::default()
            },
        ]);

        assert_eq!(tournament.call(1), 0);
        assert_eq!(tournament.call(2), 1);
        assert_eq!(tournament.winners().len(), 1);
        let winner = tournament.winners().get(0).unwrap().clone();
        assert_eq!(winner.score(), Ok(14));

        assert_eq!(tournament.call(3), 2);
        assert_eq!(tournament.winners().len(), 2);
        let winner = tournament.winners().get(1).unwrap().clone();
        assert_eq!(winner.score(), Ok(12));
    }

    #[test]
    fn it_parses_bingo_input() {
        let doc = r#"1,2,3,4

1 2
3 4

4 5
6 7

"#;
        println!("{}", doc);

        let (calls, boards) = parse_input(&doc.lines(), 2);
        assert_eq!(calls, vec![1, 2, 3, 4]);
        assert_eq!(boards.len(), 2);
        assert_eq!(boards.get(0).unwrap().board.get(0).unwrap().clone(), BingoCell(1, false));
        assert_eq!(boards.get(0).unwrap().board.get(1).unwrap().clone(), BingoCell(2, false));
        assert_eq!(boards.get(0).unwrap().board.get(2).unwrap().clone(), BingoCell(3, false));
        assert_eq!(boards.get(0).unwrap().board.get(3).unwrap().clone(), BingoCell(4, false));
        assert_eq!(boards.get(1).unwrap().board.get(0).unwrap().clone(), BingoCell(4, false));
        assert_eq!(boards.get(1).unwrap().board.get(1).unwrap().clone(), BingoCell(5, false));
        assert_eq!(boards.get(1).unwrap().board.get(2).unwrap().clone(), BingoCell(6, false));
        assert_eq!(boards.get(1).unwrap().board.get(3).unwrap().clone(), BingoCell(7, false));
    }

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day4/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let (calls, boards) = parse_input(&lines, 5);
        assert_eq!(calls, vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1]);
        assert_eq!(boards.len(), 3);
    }


    #[test]
    fn part1_works() {
        let part1_file = "src/day4/test.txt";

        let contents = fs::read_to_string(part1_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let (calls, bingo_boards) = parse_input(&lines, 5);
        let mut tournament = Tournament(bingo_boards);

        let _winning_score = 0u32;
        for call in calls {
            let winners = tournament.call(call);
            if winners > 0 {
                assert_eq!(winners, 1);
                let winner = tournament.winners().get(0).unwrap().clone();
                assert_eq!(winner.score(), Ok(4512));
                break;
            }
        }
    }

    #[test]
    fn part2_works() {
        let part2_file = "src/day4/test.txt";

        let contents = fs::read_to_string(part2_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let (calls, bingo_boards) = parse_input(&lines, 5);
        let mut tournament = Tournament(bingo_boards.clone());

        for call in calls {
            let winners = tournament.call(call);
            println!("call: {} winners: {} total: {}", call, winners, bingo_boards.len() as u32);
            if winners == bingo_boards.len() as u32 {
                let mut winning_boards = tournament.winners();
                winning_boards.sort_by(|a, b| a.calls.cmp(&b.calls));

                winning_boards.iter().for_each(|b| {
                    println!("{}", b);
                    println!("calls: {} score: {}", b.calls, b.score().unwrap());
                });

                let last_winner = winning_boards.pop().unwrap();
                assert_eq!(last_winner.score(), Ok(1924));
                break;
            }
        }
    }
}
