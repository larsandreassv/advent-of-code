use aoc_helper::{AocDay, Puzzle};

fn solution_1(input: String) -> u32 {
    let mut solution: u32 = 0;

    for line in input.lines() {
        let numbers = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let first_digit = numbers.first().unwrap();
        let last_digit = numbers.last().unwrap();

        solution += 10 * first_digit + last_digit;
    }

    return solution;
}

fn solution_2(input: String) -> u32 {
    let mut solution: u32 = 0;

    let dictionary: Vec<(&str, u32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in input.lines() {
        let mut first_digit : Option<u32> = None;
        let mut last_digit : Option<u32> = None;

        for i in 0..line.len() {
            let mut digit: Option<u32> = None;

            digit = dictionary.iter().fold(None, |acc, (word, value)| {
                if line[i..].starts_with(word) {
                    return Some(*value);
                } else {
                    return acc;
                }
            });

            if !digit.is_some() {
                digit = line.chars().nth(i).unwrap().to_digit(10);
            }

            match (digit, first_digit) {
                (Some(digit), None) => {
                    first_digit = Some(digit);
                },
                (Some(digit), Some(_)) => {
                    last_digit = Some(digit);
                },
                _ => {},
            }

        }

        match (first_digit, last_digit) {
            (Some(first_digit), Some(last_digit)) => {
                solution += 10 * first_digit + last_digit;
            },
            (Some(first_digit), None) => {
                solution += 10 * first_digit + first_digit;
            },
            _ => {},
        }

    }

    return solution;
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 1);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(x));
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(x));

    let _result_part_1 = aoc_day
        .run(&aoc_puzzle_part_1);

    let _result_part_2 = aoc_day
        .run(&aoc_puzzle_part_2);

    if let Err(err) = _result_part_1 {
        println!("Error while running part 1: {}", err);
    }

    if let Err(err) = _result_part_2 {
        println!("Error while running part 2: {}", err);
    }
}
