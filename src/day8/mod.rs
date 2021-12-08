
// use std::fmt::Debug;
use std::fs;
use std::str::Lines;

pub fn run() {
    let file = "src/day8/input.txt";

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let _ = parse_input(lines.clone());

}


fn parse_input(mut lines: Lines) -> u32 {
    let _ = lines.next().unwrap();
    todo!()
}


#[cfg(test)]
mod tests {
    use std::fs;



    #[test]
    fn it_parses_test_input() {
        let filename = "src/day8/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        // let crabs = parse_input(lines.clone());
        // let expected = Crabs::from(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        // println!("{:?}", crabs);
        // println!("{:?}", expected);
        //
        // assert_eq!(crabs, expected);
    }

    #[test]
    fn part1_works() {
        let filename = "src/day8/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        // let crabs = parse_input(lines.clone());
        // let maneuver = crabs.find_simple_maneuver();
        // assert_eq!(maneuver, (2, 37));
    }

    #[test]
    fn part2_works() {
        let filename = "src/day8/test.txt";

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let lines = contents.lines();
        // let crabs = parse_input(lines.clone());
        // let maneuver = crabs.find_better_maneuver();
        // assert_eq!(maneuver, (5, 168));
    }
}
