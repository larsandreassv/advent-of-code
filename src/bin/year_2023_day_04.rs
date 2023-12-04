use aoc_helper::{AocDay, Puzzle};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    multi::*,
    sequence::{delimited, terminated, tuple},
    *,
};

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    scratch_numbers: Vec<u32>,
}

impl Card {
    fn win_amount(&self) -> u32 {
        let mut bits_part_1: u64 = 0;
        let mut bits_part_2: u64 = 0;

        for number in &self.winning_numbers {
            if *number > 50 {
                bits_part_2 |= 1 << (*number - 50);
            } else {
                bits_part_1 |= 1 << *number;
            }
        }

        let mut amount = 0;
        for number in &self.scratch_numbers {
            if *number > 50 {
                if bits_part_2 & (1 << (*number - 50)) != 0 {
                    amount += 1;
                }
            } else {
                if bits_part_1 & (1 << *number) != 0 {
                    amount += 1;
                }
            }
        }

        return amount;
    }

    fn score(&self) -> u32 {
        let amount = self.win_amount();
        match amount {
            0 => 0,
            x => 2u32.pow(x - 1),
        }
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    fold_many1(
        terminated(nom::character::complete::u32, space0),
        Vec::new,
        |mut acc: Vec<u32>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, card_id) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((nom::character::complete::char(':'), space1)))
    (input)?;

    let (input, (winning_numbers, _, scratch_numbers)) = tuple((
        delimited(space0, parse_numbers, space0),
        character::complete::char('|'),
        delimited(space0, parse_numbers, space0)
    ))(input)?;

    Ok((input, Card {
        id: card_id.parse().unwrap(),
        winning_numbers,
        scratch_numbers,
    }))
}

fn solution_1(input: &str) -> u32 {
    let mut solution = 0;

    for line in input.lines() {
        let (_, card) = parse_card(line).unwrap();
        solution += card.score();
    }

    return solution;
}

fn solution_2(input: &str) -> u32 {
    // guess this is big enough?
    let mut copies: [u32; 1000] = [1; 1000];

    for line in input.lines() {
        let (_, card) = parse_card(line).unwrap();

        let amount = card.win_amount();
        let card_copies = copies[card.id as usize];

        for i in 0..amount {
            copies[(card.id + i + 1) as usize] += card_copies;
        }
    }

    let last_card_id = input.lines().count();
    let solution = copies.iter().skip(1).take(last_card_id).sum();

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
