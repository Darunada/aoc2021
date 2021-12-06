use std::fmt::{Debug, Display, Error, Formatter};
use std::fs;
use std::str::{FromStr, Lines};

use min_max::*;

pub fn run() {
    let file = "src/day5/input.txt";

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let sea_floor = parse_input(lines.clone(), 1000);

    let dangerous_areas = sea_floor.0.iter()
        .flatten()
        .filter(|&c| *c >= 2u32)
        .count();
    println!("part 1: {}", dangerous_areas);
}

#[derive(Debug, PartialEq)]
struct SeaFloor(Vec<Vec<u32>>);

impl SeaFloor {
    fn map_vent(&mut self, vent: Range) {
        if vent.is_cardinal() {
            for location in vent.locations() {
                self.0[location.1][location.0] += 1
            }
        }
    }

    fn get_width(&self) -> usize {
        self.0[0].len()
    }

    fn get_height(&self) -> usize {
        self.0.len()
    }
}

impl Display for SeaFloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if self.0[y][x] == 0 {
                    write!(f, ".").unwrap();
                } else {
                    write!(f, "{}", self.0[y][x]).unwrap();
                }
            }
            writeln!(f, "").unwrap();
        }

        writeln!(f, "")
    }
}

#[derive(Debug, Copy, Clone)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: Vec<usize> = s.split(",")
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        Ok(Coordinate(n[0], n[1]))
    }
}

#[derive(Debug, Copy, Clone)]
struct Range(Coordinate, Coordinate);

impl Range {
    fn from(c1: Coordinate, c2: Coordinate) -> Range {
        Range(Coordinate(min!(c1.0, c2.0), min!(c1.1, c2.1)), Coordinate(max!(c1.0, c2.0), max!(c1.1, c2.1)))
    }

    fn locations(&self) -> Vec<Coordinate> {
        let mut coordinates = vec![];

        for x in self.0.0..=self.1.0 {
            for y in self.0.1..=self.1.1 {
                coordinates.push(Coordinate(x, y));
            }
        }

        coordinates
    }

    fn is_cardinal(&self) -> bool {
        self.0.0 == self.1.0 || self.0.1 == self.1.1
    }
}

fn parse_input(mut lines: Lines, width: usize) -> SeaFloor {
    let mut sea_floor = SeaFloor(vec![vec![0; width]; width]);

    for vent in lines {
        let coords: Vec<Coordinate> = vent.split(" -> ")
            .map(|coord| coord.parse::<Coordinate>().unwrap().into())
            .collect();

        sea_floor.map_vent(Range::from(coords[0], coords[1]));
    }

    sea_floor
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day5::{parse_input, SeaFloor};

    fn it_works() {}

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day5/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea_floor = parse_input(lines.clone(), 10);
        let expected = SeaFloor(vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ]);

        assert_eq!(sea_floor, expected);
    }


    #[test]
    fn part1_works() {
        let part1_file = "src/day5/test.txt";

        let contents = fs::read_to_string(part1_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea_floor = parse_input(lines.clone(), 10);

        let dangerous_areas = sea_floor.0.iter()
            .flatten()
            .filter(|&c| *c >= 2u32)
            .count();
        assert_eq!(dangerous_areas, 5);
    }

    #[test]
    fn part2_works() {
        let part2_file = "src/day5/test.txt";

        let contents = fs::read_to_string(part2_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        // let (calls, bingo_boards) = parse_input(&lines, 5);
    }
}
