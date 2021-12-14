use std::collections::HashMap;
// use std::fmt::Debug;
use std::fs;
use std::str::Lines;

use itertools::Itertools;

pub fn run() {
    let file = "src/day8/input.txt";

    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let lines = contents.lines();
    let problems = parse_input(lines.clone());
    let mut total = 0;
    problems.clone().into_iter().for_each(|(_q, a)| {
        a.split_whitespace().for_each(|digit| {
            match digit.len() {
                2 | 3 | 4 | 7 => total += 1,
                _ => {}
            }
        })
    });

    println!("part 1: {}", total);


    let mut total = 0;
    problems.into_iter().for_each(|(q, a)| {
        //println!("problem: {} | {}", q, a);

        let mapping = get_mapping(q);
        //println!("mapping: {:?}", mapping);


        let mut answer = String::new();
        a.split_whitespace()
            .map(|s| s.chars().sorted().collect::<String>())
            .for_each(|number| {
                let digit = mapping.iter().find_map(|(&digit, pattern)| {
                    if pattern.clone() == number {
                        Some(digit)
                    } else {
                        None
                    }
                }).unwrap();
                answer.push(digit);
            });
        //println!("{}", answer);
        total += answer.parse::<u32>().unwrap();
    });
    println!("part 2: {}", total);
}

fn match_chars(digits: &[String], matches: &str) -> Vec<String> {
    digits.to_owned()
        .into_iter()
        .filter(|digit| {
            let matches_chars = matches.chars().collect_vec();

            for &c in matches_chars.iter() {
                if !digit.contains(c) {
                    return false;
                }
            }
            true
        }).collect_vec()
}

fn get_mapping(q: &str) -> HashMap<char, String> {
    let mut mapping: HashMap<char, String> = HashMap::new();

    let digits = q.split_whitespace()
        .map(|digit| digit.chars()
            .sorted()
            .collect::<String>())
        .collect::<Vec<String>>();

    //println!("Digits : {:?}", digits);

    for digit in digits.clone() {
        if digit.len() == 2 {
            mapping.insert('1', digit.clone());
        } else if digit.len() == 3 {
            mapping.insert('7', digit.clone());
        } else if digit.len() == 4 {
            mapping.insert('4', digit.clone());
        } else if digit.len() == 7 {
            mapping.insert('8', digit.clone());
        }
    }

    //println!("first 4: {:?}", mapping);

    // 0 is six segments, contains 7 and left from 4
    let seven = mapping.get(&'7').unwrap().clone();

    // contains 0 3 9
    let mut matches_seven = match_chars(&digits, &seven);
    for (index, pattern) in matches_seven.clone().into_iter().enumerate().rev() {
        if mapping.values().contains(&pattern) {
            matches_seven.remove(index);
        }
    }

    //println!("matches 7: {:?}", matches_seven);

    for (index, digit) in matches_seven.clone().iter().enumerate() {
        if digit.len() == 5 {
            mapping.insert('3', digit.clone());
            matches_seven.remove(index);
        }
    }

    //assert_eq!(matches_seven.len(), 2);
    // println!("matches 7: {:?}", matches_seven);

    // 9 matches 3
    let three= mapping.get(&'3').unwrap().clone();
    let nine_matches_3 = match_chars(&matches_seven, &three);

    // println!("nine: {:?}", nine_matches_3);
    let nine = nine_matches_3.first().unwrap().clone();
    mapping.insert('9', nine.clone());

    // 0 is left
    let zero = matches_seven.clone().into_iter()
        .filter(|digit| {
            digit.clone() != nine
        })
        .collect::<Vec<String>>()
        .pop()
        .unwrap();
    mapping.insert('0', zero);

    //println!("add 3 9 0: {:?}", mapping);

    // got 0134789
    // need 2, 5, 6
    let four = mapping.get(&'4').unwrap().clone();
    let mut remaining = digits;
    for known in mapping.values() {
        remaining = remaining.into_iter()
            .filter(|digit| digit.clone() != known.clone())
            .collect::<Vec<String>>();
    }

    //println!("remaining: {:?}", remaining);
    assert_eq!(remaining.len(), 3);

    for (index, digit) in remaining.clone().iter().enumerate().rev() {
        if digit.len() == 6 {
            mapping.insert('6', digit.clone());
            remaining.remove(index);
        }
    }
    assert_eq!(remaining.len(), 2);
    let a = remaining.get(0).unwrap();
    let b = remaining.get(1).unwrap();
    let mut a_missing_in_four = 4;
    for c in four.chars() {
        if a.contains(c) {
            a_missing_in_four -= 1;
        }
    }

    // 2 is missing 2 in 4
    // 5 is missing 1 in 4

    if a_missing_in_four == 2 {
        mapping.insert('2', a.clone());
        mapping.insert('5', b.clone());
    } else {
        mapping.insert('5', a.clone());
        mapping.insert('2', b.clone());
    }

    // let it be known I hated this and feel dumb
    mapping
}


fn parse_input(mut lines: Lines) -> Vec<(&str, &str)> {
    let mut problems = vec![];
    loop {
        let problem = lines.next();
        if problem.is_none() {
            break;
        }

        let problem = problem.unwrap();
        let (question, answer) = problem.split_at(problem.find('|').unwrap());
        problems.push((question.trim(), answer[1..].trim()));
    }
    problems
}


#[cfg(test)]
mod tests {
    use std::fs;

    use itertools::Itertools;

    use crate::day8::{get_mapping, parse_input};

    #[test]
    fn it_parses_test_input() {
        let filename = "src/day8/test.txt";

        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines = contents.lines();
        let problems = parse_input(lines.clone());
        assert_eq!(problems.len(), 10);
        assert_eq!(problems[0].0, "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb");
        assert_eq!(problems[0].1, "fdgacbe cefdb cefbgd gcbe");
        assert_eq!(problems[9].0, "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc");
        assert_eq!(problems[9].1, "fgae cfgab fg bagce");
    }

    #[test]
    fn it_parses_mappings() {
        let q = "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc";
        let mapping = get_mapping(q);
        println!("Mapping: {:?}", mapping);
        assert_eq!(mapping[&'1'], "fg");
        assert_eq!(mapping[&'4'], "aefg");
        assert_eq!(mapping[&'7'], "cfg");
        assert_eq!(mapping[&'8'], "abcdefg");
        assert_eq!(mapping[&'0'], "bcdefg");
        assert_eq!(mapping[&'2'], "abcdf");
        assert_eq!(mapping[&'3'], "abcfg");
        assert_eq!(mapping[&'5'], "abceg");
        assert_eq!(mapping[&'6'], "abcdeg");
        assert_eq!(mapping[&'9'], "abcefg");
    }

    #[test]
    fn part1_works() {
        let filename = "src/day8/test.txt";
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines = contents.lines();

        let problems = parse_input(lines.clone());
        let mut total = 0;
        problems.into_iter().for_each(|(_q, a)| {
            a.split_whitespace().for_each(|digit| {
                match digit.len() {
                    2 | 3 | 4 | 7 => total += 1,
                    _ => {}
                }
            })
        });
        assert_eq!(total, 26);
    }

    #[test]
    fn part2_works() {
        let filename = "src/day8/test.txt";
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines = contents.lines();

        let problems = parse_input(lines.clone());
        let mut total = 0;
        problems.into_iter().for_each(|(q, a)| {
            println!("problem: {} | {}", q, a);

            let mapping = get_mapping(q);
            println!("mapping: {:?}", mapping);


            let mut answer = String::new();
            a.split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .for_each(|number| {
                    let digit = mapping.iter().find_map(|(&digit, pattern)| {
                        if pattern.clone() == number {
                            Some(digit)
                        } else {
                            None
                        }
                    }).unwrap();
                    answer.push(digit);
                });
            println!("{}", answer);
            total += answer.parse::<u32>().unwrap();
        });

        assert_eq!(total, 61229);
    }
}
