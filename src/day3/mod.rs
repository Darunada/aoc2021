use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Error;

use std::str::{FromStr, Lines};


// too low 1216568
pub fn run() {
    let part1_file = "src/day3/input.txt";

    let contents = fs::read_to_string(part1_file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let report = parse_report(&lines);
    println!("part 1: {}", PowerConsumption::analyze(&report, 12).power_consumption());
    println!("part 2: {}", LifeSupportRating::analyze(&report, 12).life_support_rating());
}

#[derive(Clone, PartialEq)]
struct ReportLine(usize);

impl Debug for ReportLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:05b}", self.0)
    }
}

impl FromStr for ReportLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed: usize = 0;
        s.chars()
            .rev()
            .enumerate()
            .map(|(_i, c)| c.to_string().parse::<usize>().unwrap())
            .enumerate()
            .map(|(i, c)| {
                match c {
                    0 => 0 << i,
                    1 => 1 << i,
                    _ => panic!("Unexpected char while parsing"),
                }
            })
            .for_each(|mask| {
                parsed |= mask;
            });

        Ok(ReportLine(parsed))
    }
}

#[derive(Debug, PartialEq)]
struct GammaRate(usize);

#[derive(Debug, PartialEq)]
struct EpsilonRate(usize);

#[derive(PartialEq)]
struct OxygenRating(usize);

impl Debug for OxygenRating {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

#[derive(Debug, PartialEq)]
struct Co2Rating(usize);

#[derive(Debug)]
struct PowerConsumption(GammaRate, EpsilonRate);

impl PowerConsumption {
    fn power_consumption(&self) -> usize {
        self.0.0 * self.1.0
    }

    fn analyze(diagnostic_report: &[ReportLine], bit_width: u32) -> PowerConsumption {
        let mut gamma_rate: usize = 0;
        let mut epsilon_rate: usize = 0;
        let total = diagnostic_report.len() as u32;

        for bit_position in 0..bit_width {
            let ones = ones_at_position(&diagnostic_report.to_owned(), bit_position);
            if ones > (total / 2) {
                gamma_rate |= 0x1 << bit_position;
            } else {
                epsilon_rate |= 0x1 << bit_position;
            }
        }

        PowerConsumption(GammaRate(gamma_rate), EpsilonRate(epsilon_rate))
    }
}

#[derive(Debug)]
struct LifeSupportRating(OxygenRating, Co2Rating);

impl LifeSupportRating {
    fn life_support_rating(&self) -> usize {
        self.0.0 * self.1.0
    }

    fn analyze(diagnostic_report: &[ReportLine], bit_width: u32) -> LifeSupportRating {
        let mut oxygen_rating = diagnostic_report.to_owned();
        let mut co2_rating = diagnostic_report.to_owned();

        for bit_position in 0..bit_width {
            let bit = bit_width - bit_position - 1;
            if oxygen_rating.len() == 1 && co2_rating.len() == 1 {
                break;
            }

            if oxygen_rating.len() > 1 {
                let total = oxygen_rating.len() as u32;
                let ones = ones_at_position(&oxygen_rating, bit);
                let zeros = total - ones;

                if ones >= zeros {
                    // keep values with 1 in position
                    oxygen_rating = oxygen_rating.into_iter().filter(|line| {
                        let index = 0x1 << bit;
                        (line.0 & index) > 0
                    }).collect();
                } else {
                    // keep values with 0 in position
                    oxygen_rating = oxygen_rating.into_iter().filter(|line| {
                        let index = 0x1 << bit;
                        (line.0 & index) == 0
                    }).collect();
                }
            }


            if co2_rating.len() > 1 {
                let total = co2_rating.len() as u32;
                let ones = ones_at_position(&co2_rating, bit);
                let zeros = total - ones;
                if ones >= zeros {
                    // keep values with 0 in position
                    co2_rating = co2_rating.into_iter().filter(|line| {
                        let index = 0x1 << bit;
                        (line.0 & index) == 0
                    }).collect();
                } else {
                    // keep values with 1 in position
                    co2_rating = co2_rating.into_iter().filter(|line| {
                        let index = 0x1 << bit;
                        (line.0 & index) > 0
                    }).collect();
                }
            }
        }

        LifeSupportRating(OxygenRating(oxygen_rating.get(0).unwrap().0), Co2Rating(co2_rating.get(0).unwrap().0))
    }
}

fn parse_report(diagnostic_report: &Lines) -> Vec<ReportLine> {
    diagnostic_report
        .clone()
        .map(|line| line.parse::<ReportLine>().unwrap())
        .collect::<Vec<ReportLine>>()
}

fn ones_at_position(report: &[ReportLine], bit_position: u32) -> u32 {
    let mut ones: u32 = 0;
    report.iter()
        .for_each(|line| {
            let index = 0x1 << bit_position;
            if (line.0 & index) > 0 {
                ones += 1;
            }
        });

    ones
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day3::{Co2Rating, EpsilonRate, GammaRate, LifeSupportRating, OxygenRating, parse_report, PowerConsumption, ReportLine};

    #[test]
    fn it_parses_report_lines() {
        let line = "01010101";
        let result = line.parse::<ReportLine>();
        assert_eq!(result.unwrap(), ReportLine(1 + 4 + 16 + 64));
    }

    #[test]
    fn it_parses_reports() {
        let report = "0000\n0001\n0011\n0111\n1111";
        let lines = report.lines();
        let parsed = parse_report(&lines);
        assert_eq!(parsed.len(), 5);
        assert_eq!(*parsed.get(0).unwrap(), ReportLine(0));
        assert_eq!(*parsed.get(1).unwrap(), ReportLine(1));
        assert_eq!(*parsed.get(2).unwrap(), ReportLine(3));
        assert_eq!(*parsed.get(3).unwrap(), ReportLine(7));
        assert_eq!(*parsed.get(4).unwrap(), ReportLine(15));
    }

    #[test]
    fn it_analyzes_power_consumption() {
        let report = "0000\n0001\n0011\n0111\n1111";
        let lines = report.lines();
        let parsed = parse_report(&lines);
        let result = PowerConsumption::analyze(&parsed, 4);
        // 0011
        assert_eq!(result.0, GammaRate(3));
        assert_eq!(result.1, EpsilonRate(12));
    }

    #[test]
    fn test_power_consumption_works() {
        let filename = "src/day3/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let report = parse_report(&lines);
        assert_eq!(report.len(), 12);
        let result = PowerConsumption::analyze(&report, 5);

        assert_eq!(result.0, GammaRate(0b10110));
        assert_eq!(result.1, EpsilonRate(0b01001));
    }

    #[test]
    fn test_life_support_works() {
        let filename = "src/day3/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let report = parse_report(&lines);
        assert_eq!(report.len(), 12);
        let result = LifeSupportRating::analyze(&report, 5);

        assert_eq!(result.0, OxygenRating(23));
        assert_eq!(result.1, Co2Rating(10));
    }

    #[test]
    fn part1_works() {
        let filename = "src/day3/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let report = parse_report(&lines);
        let result = PowerConsumption::analyze(&report, 5).power_consumption();
        assert_eq!(result, 198);
    }

    #[test]
    fn part2_works() {
        let filename = "src/day3/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        let report = parse_report(&lines);
        assert_eq!(report.len(), 12);
        let result = LifeSupportRating::analyze(&report, 5);
        assert_eq!(result.0, OxygenRating(23));
        assert_eq!(result.1, Co2Rating(10));
        assert_eq!(result.life_support_rating(), 230);
    }
}
