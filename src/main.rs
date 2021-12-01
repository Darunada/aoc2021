use std::fs;
use std::str::Lines;

fn main() {
    let part1_file = "input.txt";

    let contents = fs::read_to_string(part1_file)
        .expect("Something went wrong reading the file");

    let mut lines1 = contents.lines();
    part1(lines1);

    let part2_file = "input2.txt";
    let contents = fs::read_to_string(part2_file)
        .expect("Something went wrong reading the file");

    let mut lines2 = contents.lines();
    part2(lines2);
}

fn part1(mut lines: Lines) -> u32 {
    let mut prev = lines.next().unwrap().parse::<u32>().unwrap();

    let mut gt_count = 0;

    loop {
        let next = lines.next();
        if next.is_none() {
            println!("got none");
            break;
        }
        let num = next.unwrap().parse::<u32>().unwrap();;
        if num > prev {
            println!("num {} gt prv {}", num, prev);
            gt_count += 1;
        }
        prev = num;
    }

    println!("gt count: {}", gt_count);
    gt_count
}

fn part2(mut lines: Lines) -> u32 {
    let mut first = lines.next().unwrap().parse::<u32>().unwrap();
    let mut second = lines.next().unwrap().parse::<u32>().unwrap();
    let mut third = lines.next().unwrap().parse::<u32>().unwrap();
    let mut gt_count = 0;

    let mut window1 = 0;
    let mut window2 = 0;

    loop {
        let next = lines.next();
        if next.is_none() {
            println!("got none");
            break;
        }
        let fourth = next.unwrap().parse::<u32>().unwrap();
        window1 = first + second + third;
        window2 = second + third + fourth;
        if window2 > window1 {
            gt_count += 1;
        }
        window1 = window2;
        first = second;
        second = third;
        third = fourth;
    }

    println!("gt count: {}", gt_count);
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
