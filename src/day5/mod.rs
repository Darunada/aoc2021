use std::fmt::{Debug, Display, Error, Formatter};
use std::fs;
use std::str::{FromStr, Lines};



pub fn run() {
    let file = "src/day5/input.txt";

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let sea_floor1 = parse_input(lines.clone(), 1000, false);

    let dangerous_areas = sea_floor1.0.iter()
        .flatten()
        .filter(|&c| *c >= 2u32)
        .count();
    println!("part 1: {}", dangerous_areas);

    let sea_floor2 = parse_input(lines.clone(), 1000, true);

    let dangerous_areas = sea_floor2.0.iter()
        .flatten()
        .filter(|&c| *c >= 2u32)
        .count();
    println!("part 2: {}", dangerous_areas);
}

#[derive(Debug, PartialEq)]
struct SeaFloor(Vec<Vec<u32>>);

impl SeaFloor {
    fn map_vent(&mut self, vent: Range, use_diagonals: bool) {
        if vent.is_cardinal() || use_diagonals {
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
            writeln!(f).unwrap();
        }

        writeln!(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: Vec<usize> = s.split(',')
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        Ok(Coordinate(n[0], n[1]))
    }
}

#[derive(Debug, Copy, Clone)]
struct Range(Coordinate, Coordinate);

impl Range {
    fn from(c1: Coordinate, c2: Coordinate) -> Range {
        let mut range = Range(c1, c2);
        // same x
        if c1.0 == c2.0 {
            if c2.1 < c1.1 {
                range = range.flip();
            }
        } else  {
            // put least x in c1
            if c2.0 < c1.0 {
                range = range.flip();
            }
        }

        range
    }

    fn flip(&self) -> Range {
        Range(self.1, self.0)
    }

    fn locations(&self) -> Vec<Coordinate> {
        let mut coordinates = vec![];

        if self.is_cardinal() {
            for x in self.0.0..=self.1.0 {
                for y in self.0.1..=self.1.1 {
                    coordinates.push(Coordinate(x, y));
                }
            }
        } else {
            let mut x = self.0.0;
            let mut y = self.0.1;
            let incrementing_y = self.0.1 < self.1.1;
            loop {
                let coordinate = Coordinate(x, y);

                coordinates.push(coordinate);
                if coordinate == self.1 {
                    break;
                }

                x += 1;
                if incrementing_y {
                       y += 1;
                } else {
                    y -= 1;
                }
            }
        }
        coordinates
    }

    fn is_cardinal(&self) -> bool {
        self.0.0 == self.1.0 || self.0.1 == self.1.1
    }
}

fn parse_input(lines: Lines, width: usize, use_diagonals: bool) -> SeaFloor {
    let mut sea_floor = SeaFloor(vec![vec![0; width]; width]);

    for vent in lines {
        let coords: Vec<Coordinate> = vent.split(" -> ")
            .map(|coord| coord.parse::<Coordinate>().unwrap())
            .collect();

        sea_floor.map_vent(Range::from(coords[0], coords[1]), use_diagonals);
    }

    sea_floor
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day5::{parse_input, SeaFloor};

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day5/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea_floor = parse_input(lines.clone(), 10, false);
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
    fn it_parses_test_input_with_diagonals() {
        let filename = "src/day5/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea_floor = parse_input(lines.clone(), 10, true);
        let expected = SeaFloor(vec![
            vec![1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            vec![0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            vec![0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            vec![0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ]);
        println!("{}", sea_floor);
        println!("{}", expected);

        assert_eq!(sea_floor, expected);
    }



    #[test]
    fn part1_works() {
        let part1_file = "src/day5/test.txt";

        let contents = fs::read_to_string(part1_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea_floor = parse_input(lines.clone(), 10, false);

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
        let sea_floor = parse_input(lines.clone(), 10, true);

        let dangerous_areas = sea_floor.0.iter()
            .flatten()
            .filter(|&c| *c >= 2u32)
            .count();
        assert_eq!(dangerous_areas, 12);
    }
}
