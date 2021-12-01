use std::fs;
use std::str::Lines;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines();
    calc(lines);
}

fn calc(mut lines: Lines) -> u32 {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::calc;

    #[test]
    fn it_works() {
        let filename = "test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        assert_eq!(calc(lines.clone()), 7);
    }
}
