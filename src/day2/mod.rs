use std::fs;
use std::str::Lines;

#[derive(Debug, PartialEq)]
enum Move {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn run() {
    let part1_file = "src/day2/input.txt";

    let contents = fs::read_to_string(part1_file)
        .expect("Something went wrong reading the file");

    let lines1 = contents.lines();
    println!("part 1: {}", part1(lines1));

    let part2_file = "src/day2/input.txt";
    let contents = fs::read_to_string(part2_file)
        .expect("Something went wrong reading the file");

    let lines2 = contents.lines();
    println!("part 2: {}", part2(lines2));
}

fn get_moves(mut lines: Lines) -> Vec<Move> {
    let mut moves: Vec<Move> = vec!();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let motion = line.unwrap();
        let mut split = motion.split_ascii_whitespace();

        let direction = split.next().unwrap();
        let distance = split.next().unwrap();

        let the_move = match direction {
            "forward" => Move::Forward(distance.parse::<u32>().unwrap()),
            "down" => Move::Down(distance.parse::<u32>().unwrap()),
            "up" => Move::Up(distance.parse::<u32>().unwrap()),
            _ => panic!("reached unknown move: {}", direction),
        };

        moves.push(the_move);
    }

    moves
}

fn part1(lines: Lines) -> u32 {
    let moves = get_moves(lines);
    let mut hpos = 0;
    let mut vpos = 0;
    for a_move in moves {
        match a_move {
            Move::Forward(d) => hpos += d,
            Move::Down(d) => vpos += d,
            Move::Up(d) => vpos -= d,
        }
    }

    hpos * vpos
}

fn part2(lines: Lines) -> u32 {
    let moves = get_moves(lines);
    let mut hpos = 0;
    let mut vpos = 0;
    let mut aim = 0;
    for a_move in moves {
        match a_move {
            Move::Forward(d) => {
                hpos += d;
                vpos += d * aim;
            },
            Move::Down(d) => aim += d,
            Move::Up(d) => aim -= d,
        }
    }

    hpos * vpos
}

#[cfg(test)]
mod tests {
    use std::fs;

    
    use crate::day2::{get_moves, part1, part2};
    use crate::day2::Move::{Down, Forward, Up};

    #[test]
    fn it_gets_moves() {
        let filename = "src/day2/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();

        let result = get_moves(lines.clone());
        assert_eq!(result[0], Forward(5));
        assert_eq!(result[1], Down(5));
        assert_eq!(result[2], Forward(8));
        assert_eq!(result[3], Up(3));
        assert_eq!(result[4], Down(8));
        assert_eq!(result[5], Forward(2));
    }

    #[test]
    fn part1_works() {
        let filename = "src/day2/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        assert_eq!(part1(lines.clone()), 150);
    }

    #[test]
    fn part2_works() {
        let filename = "src/day2/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        assert_eq!(part2(lines.clone()), 900);
    }
}
