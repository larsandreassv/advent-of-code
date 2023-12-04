use std::collections::HashMap;

use aoc_helper::{AocDay, Puzzle};
use nom::{bytes::complete::tag, character::complete::digit1, multi::*, sequence::terminated, *};

fn parse_card(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, card_id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    Ok((input, card_id.parse::<u32>().unwrap()))
}

fn parse_scratch_numbers(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, winning_numbers) = separated_list1(many1(tag(" ")), digit1)(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, scratch_numbers) = separated_list1(many1(tag(" ")), digit1)(input)?;
    Ok((
        input,
        (
            winning_numbers
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
            scratch_numbers
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        ),
    ))
}

fn parse_scratch_card_line(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (input, card_id) = parse_card(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, (winning_numbers, scratch_numbers)) = parse_scratch_numbers(input)?;
    Ok((input, (card_id, winning_numbers, scratch_numbers)))
}

fn solution_1(input: &str) -> u32 {
    let mut solution = 0;

    let mut lines = &input[..];

    while lines.len() > 0 {
        let mut bits_part_1: u64 = 0;
        let mut bits_part_2: u64 = 0;

        let winning_numbers: Vec<u32>;
        let scratch_numbers: Vec<u32>;

        (lines, (_, winning_numbers, scratch_numbers)) = parse_scratch_card_line(lines).unwrap();

        if lines.chars().nth(0) == Some('\n') {
            lines = &lines[1..];
        }

        for number in winning_numbers {
            if number > 50 {
                bits_part_2 |= 1 << (number - 50);
            } else {
                bits_part_1 |= 1 << number;
            }
        }

        let mut amount = 0;
        for number in scratch_numbers {
            if number > 50 {
                if bits_part_2 & (1 << (number - 50)) != 0 {
                    amount += 1;
                }
            } else {
                if bits_part_1 & (1 << number) != 0 {
                    amount += 1;
                }
            }
        }

        if amount > 0 {
            solution += 2u32.pow(amount - 1);
        }
    }

    return solution;
}

fn solution_2(input: &str) -> u32 {
    let mut solution = 0;

    let mut lines = &input[..];

    // guess this is big enough?
    let mut copies: [u32; 1000] = [1; 1000];
    let mut last_card_id = 0;

    while lines.len() > 0 {
        let mut bits_part_1: u64 = 0;
        let mut bits_part_2: u64 = 0;

        let card_id: u32;
        let winning_numbers: Vec<u32>;
        let scratch_numbers: Vec<u32>;

        (lines, (card_id, winning_numbers, scratch_numbers)) =
            parse_scratch_card_line(lines).unwrap();

        if lines.chars().nth(0) == Some('\n') {
            lines = &lines[1..];
        }

        for number in winning_numbers {
            if number > 50 {
                bits_part_2 |= 1 << (number - 50);
            } else {
                bits_part_1 |= 1 << number;
            }
        }

        let mut amount = 0;
        for number in scratch_numbers {
            if number > 50 {
                if bits_part_2 & (1 << (number - 50)) != 0 {
                    amount += 1;
                }
            } else {
                if bits_part_1 & (1 << number) != 0 {
                    amount += 1;
                }
            }
        }

        let card_copies = copies[card_id as usize];
        for i in 0..amount {
            copies[(card_id + i + 1) as usize] += card_copies;
        }

        last_card_id = card_id;
    }

    solution = copies.iter().skip(1).take(last_card_id as usize).sum();

    return solution;
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 4);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(&x)).with_examples(&[vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ]
    .join("\n")]);
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(&x)).with_examples(&[vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ]
    .join("\n")]);
    let _test_result_part_1 = aoc_day.test(&aoc_puzzle_part_1);
    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);

    let _test_result_part_2 = aoc_day.test(&aoc_puzzle_part_2);
    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);
}
