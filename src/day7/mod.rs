
use std::{fs};
use std::fmt::{Debug};
use std::str::{Lines};

pub fn run() {
    let file = "src/day7/input.txt";

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let crabs = parse_input(lines.clone());
    let maneuver = crabs.find_simple_maneuver();
    println!("part 1: {}", maneuver.1);

    let maneuver = crabs.find_better_maneuver();
    println!("part 2: {}", maneuver.1);

    //
    // for _ in 80u32..256 {
    //     sea.day();
    // }
    // println!("part 2: {}", sea.population());
}

#[derive(Debug, Clone, PartialEq)]
struct Crabs(Vec<u32>);

impl From<Vec<u32>> for Crabs {
    fn from(crabs: Vec<u32>) -> Crabs {
        Crabs(crabs)
    }
}

impl Crabs {
    fn simple_fuel_cost(&self, direction: u32) -> u32 {
        self.0.iter()
            .map(|&c| {
                (c as i32 - direction as i32).abs() as u32
            })
            .sum()
    }

    fn find_simple_maneuver(&self) -> (u32, u32) {
        let mut least: (u32, u32) = (0, self.simple_fuel_cost(0));

        for i in 1..1000 {
            if self.simple_fuel_cost(i) < least.1 {
                least = (i, self.simple_fuel_cost(i));
            }
        }

       least
    }

    fn find_better_maneuver(&self) -> (u32, u32) {
        let mut least: (u32, u32) = (0, self.better_fuel_cost(0));

        for i in 1..1000 {
            if self.better_fuel_cost(i) < least.1 {
                least = (i, self.better_fuel_cost(i));
            }
        }

        least
    }

    fn better_fuel_cost(&self, direction: u32) -> u32 {
        self.0.iter()
            .map(|&c| {
                let num_steps = (c as i32 - direction as i32).abs() as u32;
                (0u32..num_steps).sum::<u32>() + num_steps
            })
            .sum()
    }
}


fn parse_input(mut lines: Lines) -> Crabs {
    Crabs::from(lines.next()
        .unwrap()
        .split(',')
        .map(|n| { n.parse::<u32>().unwrap() })
        .collect::<Vec<u32>>())
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day7::{Crabs, parse_input};

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day7/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let crabs = parse_input(lines.clone());
        let expected = Crabs::from(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        println!("{:?}", crabs);
        println!("{:?}", expected);

        assert_eq!(crabs, expected);
    }

    #[test]
    fn crabs_calculate_fuel_cost() {
        let crabs = Crabs::from(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(crabs.simple_fuel_cost(2), 37);
        assert_eq!(crabs.simple_fuel_cost(1), 41);
        assert_eq!(crabs.simple_fuel_cost(3), 39);
        assert_eq!(crabs.simple_fuel_cost(10), 71);
    }

    #[test]
    fn crabs_calculate_better_fuel_cost() {
        let crabs = Crabs::from(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(crabs.better_fuel_cost(2), 206);
        assert_eq!(crabs.better_fuel_cost(5), 168);
    }


    #[test]
    fn part1_works() {
        let filename = "src/day7/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let crabs = parse_input(lines.clone());
        let maneuver = crabs.find_simple_maneuver();
        assert_eq!(maneuver, (2, 37));
    }

    #[test]
    fn part2_works() {
        let filename = "src/day7/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let crabs = parse_input(lines.clone());
        let maneuver = crabs.find_better_maneuver();
        assert_eq!(maneuver, (5, 168));
    }


    //
    //
    // #[test]
    // fn part2_works() {
    //     let part2_file = "src/day6/test.txt";
    //
    //     let contents = fs::read_to_string(part2_file)
    //         .expect("Something went wrong reading the file");
    //
    //     let lines = contents.lines();
    //     let mut sea = parse_input(lines.clone());
    //
    //     for i in 0u32..256 {
    //         sea.day();
    //         // println!("Day {}: {}", i, sea);
    //     }
    //     assert_eq!(sea.population(), 26984457539);
    // }
}
