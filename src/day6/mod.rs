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
    println!("part 1: {}", sea.population());

    for _ in 80u32..256 {
        sea.day();
    }
    println!("part 2: {}", sea.population());
}

#[derive(Debug, Clone, PartialEq)]
struct Sea(Vec<u64>);

impl From<Vec<u32>> for Sea {
    fn from(fish: Vec<u32>) -> Sea {
        let mut sea = Sea::new();

        for f in fish {
            sea.0[f as usize] += 1;
        }

        sea
    }
}

impl Sea {
    fn new() -> Sea {
        // 0-8 are valid ages
        Sea(vec![0u64; 9])
    }

    fn day(&mut self) {
        // remove the 0s
        let (&reproducing, rest) = self.0.split_first().unwrap();
        self.0 = rest.to_vec();

        // println!("reproducing: \n{}", reproducing);
        // println!("rest: \n{}", self);

        // 0s reproduce, become 8s
        self.0.push(reproducing);

        // reproducing are replaced with 6s
        self.0[6] += reproducing;
    }

    fn population(&self) -> u64 {
        let mut population = 0u64;
        for age in self.0.iter() {
            population += age;
        }
        population
    }
}

impl Display for Sea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().enumerate().for_each(|(age, num)| {
            writeln!(f, "age: {}, num: {}", age, num);
        });

        write!(f, "")
    }
}

fn parse_input(mut lines: Lines) -> Sea {
    Sea::from(lines.next()
        .unwrap()
        .split(",")
        .map(|n| { n.parse::<u32>().unwrap() })
        .collect::<Vec<u32>>())
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day6::{parse_input, Sea};

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day6/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let sea = parse_input(lines.clone());
        let expected = Sea::from(vec![3, 4, 3, 1, 2]);
        println!("{}", sea);
        println!("{}", expected);

        assert_eq!(sea, expected);
    }

    #[test]
    fn fishes_decrement() {
        let mut sea = Sea::new();
        println!("{}", sea);
        sea.0[1] = 2;
        println!("{}", sea);
        sea.day();

        println!("{}", sea);
        assert_eq!(sea.0[1], 0);
        assert_eq!(sea.0[0], 2);
    }

    #[test]
    fn fishes_reproduce() {
        let mut sea = Sea::new();
        sea.0[0] = 1;
        sea.day();
        assert_eq!(sea.0[0], 0);
        assert_eq!(sea.0[8], 1);
        assert_eq!(sea.0[6], 1);
        assert_eq!(sea.population(), 2)
    }

    #[test]
    fn part1_works() {
        let part1_file = "src/day6/test.txt";

        let contents = fs::read_to_string(part1_file)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let mut sea = parse_input(lines.clone());

        assert_eq!(sea.population(), 5);
        sea.day();
        assert_eq!(sea.population(), 5);
        sea.day();
        assert_eq!(sea.population(), 6);

        for i in 2u32..80 {
            sea.day();
            println!("Day {}: {}", i, sea);
        }
        assert_eq!(sea.population(), 5934);
        panic!();
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
            // println!("Day {}: {}", i, sea);
        }
        assert_eq!(sea.population(), 26984457539);
    }
}
