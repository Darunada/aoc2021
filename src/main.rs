use std::fs;
use std::str::Lines;

fn main() {
    let part1_file = "input.txt";

    let contents = fs::read_to_string(part1_file)
        .expect("Something went wrong reading the file");

    let lines1 = contents.lines();
    println!("part 1: {}", part1(lines1));

    let part2_file = "input2.txt";
    let contents = fs::read_to_string(part2_file)
        .expect("Something went wrong reading the file");

    let lines2 = contents.lines();
    println!("part 2: {}", part2(lines2));
}

fn part1(mut lines: Lines) -> u32 {
    let mut prev = lines.next().unwrap().parse::<u32>().unwrap();

    let mut gt_count = 0;

    loop {
        let next = lines.next();
        if next.is_none() {
            break;
        }
        let num = next.unwrap().parse::<u32>().unwrap();
        if num > prev {
            gt_count += 1;
        }
        prev = num;
    }

    gt_count
}

fn part2(mut lines: Lines) -> u32 {
    let mut first = lines.next().unwrap().parse::<u32>().unwrap();
    let mut second = lines.next().unwrap().parse::<u32>().unwrap();
    let mut third = lines.next().unwrap().parse::<u32>().unwrap();
    let mut gt_count = 0;

    loop {
        let next = lines.next();
        if next.is_none() {
            break;
        }
        let fourth = next.unwrap().parse::<u32>().unwrap();
        let window1 = first + second + third;
        let window2 = second + third + fourth;
        if window2 > window1 {
            gt_count += 1;
        }
        first = second;
        second = third;
        third = fourth;
    }

    gt_count
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{part1, part2};

    #[test]
    fn part1_works() {
        let filename = "test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        assert_eq!(part1(lines.clone()), 7);
    }

    #[test]
    fn part2_works() {
        let filename = "test2.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        assert_eq!(part2(lines.clone()), 5);
    }
}
