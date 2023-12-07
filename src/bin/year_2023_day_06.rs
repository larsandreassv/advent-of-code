use aoc_helper::{AocDay, Puzzle};

fn parse_race(input: &str) -> Vec<(u32, u32)> {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();

    let mut first_line_iter = first_line[5..].split_whitespace().filter(|x| !x.is_empty());
    let mut second_line_iter = second_line[9..]
        .split_whitespace()
        .filter(|x| !x.is_empty());

    //zip
    let result =
        first_line_iter
            .zip(second_line_iter)
            .fold(Vec::new(), |mut acc, (time, distance)| {
                acc.push((
                    time.parse::<u32>().unwrap(),
                    distance.parse::<u32>().unwrap(),
                ));
                acc
            });

    result
}

fn parse_race_part_2(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();

    let time = first_line[5..].split(" ").filter(|x| !x.is_empty()).collect::<String>().parse::<u64>().unwrap();
    let distance = second_line[9..].split(" ").filter(|x| !x.is_empty()).collect::<String>().parse::<u64>().unwrap();

    (time, distance)
}

fn solution_1(input: &str) -> u32 {
    let mut solution = 1;
    let races = parse_race(input);

    for (time, record) in races {
        let criterion = (time * time - 4 * record) as f64;
        let criterion_sqrt = criterion.sqrt();
        let min = (time as f64 - criterion_sqrt) / 2.0;
        let max = (time as f64 + criterion_sqrt) / 2.0;
        let mut result = max.floor() - min.floor();
        if max.floor() == max {
            result -= 1.0;
        }
        solution *= result as u32;
    }

    solution
}

fn solution_2(input: &str) -> u64 {
    let (time, record) = parse_race_part_2(input);
    let criterion = (time * time - 4 * record) as f64;
    let criterion_sqrt = criterion.sqrt();
    let min = (time as f64 - criterion_sqrt) / 2.0;
    let max = (time as f64 + criterion_sqrt) / 2.0;
    let mut result = max.floor() - min.floor();
    if max.floor() == max {
        result -= 1.0;
    }
    result as u64
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 6);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(&x));
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(&x));
    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);
    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse_race(&vec!["Time:      7  15   30", "Distance:  9  40  200"].join("\n")),
            vec![(7, 9), (15, 40), (30, 200)]
        );
    }

    #[test]
    fn solution_1_test() {
        assert_eq!(
            solution_1(&vec!["Time:      7  15   30", "Distance:  9  40  200"].join("\n")),
            288
        );
    }
}
