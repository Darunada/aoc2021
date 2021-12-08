use std::fmt::{Debug, Display, Error, Formatter};
use std::fs;
use std::str::{FromStr, Lines};

use min_max::*;

pub fn run() {
    let file = "src/day6/input.txt";

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let mut sea = parse_input(lines.clone());

    for _ in 0u32..80 {
        sea.day();
    }
    println!("part 1: {}", sea.0.len());
}

#[derive(Debug, PartialEq)]
struct LanternFish(u32);

impl From<u32> for LanternFish {
    fn from(num: u32) -> Self {
        LanternFish(num)
    }
}

#[derive(Debug, PartialEq)]
struct Sea(Vec<LanternFish>);

impl Sea {
    fn day(&mut self) {
        let mut new_fishes = vec![];
        self.0.iter_mut().for_each(|fish| {
            if fish.0 == 0 {
                new_fishes.push(LanternFish(8));
                fish.0 = 6;
            } else {
                fish.0 -= 1;
            }
        });

        self.0.append(&mut new_fishes);
    }
}

impl Display for Sea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for fish in &self.0 {
            write!(f, "{}", fish.0);
        }

        writeln!(f, "")
    }
}

fn parse_input(mut lines: Lines) -> Sea {
    Sea(lines.next()
        .unwrap()
        .split(",")
        .map(|n| { n.parse::<u32>().unwrap() })
        .map(|n| { n.into() })
        .collect())
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day6::{LanternFish, parse_input, Sea};

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day6/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea = parse_input(lines.clone());
        let expected = Sea(vec![3.into(), 4.into(), 3.into(), 1.into(), 2.into()]);
        println!("{}", sea);
        println!("{}", expected);

        assert_eq!(sea, expected);
    }

    #[test]
    fn fishes_decrement() {
        let mut sea = Sea(vec![LanternFish(1)]);
        sea.day();
        assert_eq!(sea.0.first().unwrap().0, 0);
        assert_eq!(sea.0.len(), 1);
    }

    #[test]
    fn fishes_reproduce() {
        let mut sea = Sea(vec![LanternFish(0)]);
        sea.day();
        assert_eq!(sea.0.first().unwrap().0, 6);
        assert_eq!(sea.0.len(), 2);
        assert_eq!(sea.0.get(1).unwrap().0, 8);
    }

    #[test]
    fn part1_works() {
        let part1_file = "src/day6/test.txt";

        let contents = fs::read_to_string(part1_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let mut sea = parse_input(lines.clone());

        for i in 0u32..80 {
            sea.day();
        }
        assert_eq!(sea.0.len(), 5934);
    }


    #[test]
    fn part2_works() {
        let part2_file = "src/day6/test.txt";

        let contents = fs::read_to_string(part2_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let mut sea = parse_input(lines.clone());

        for i in 0u32..256 {
            sea.day();
        }
        assert_eq!(sea.0.len(), 26984457539);
    }
}
