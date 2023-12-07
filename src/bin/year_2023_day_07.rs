use aoc_helper::{AocDay, Puzzle};
use nom::{
    character::complete::{digit1, space0},
    error::{Error, ErrorKind},
    sequence::separated_pair,
    IResult, Parser,
};

struct Game {
    hand_rank: u32,
    pot: u32,
    card: String
}

enum HandRankKind {
    OneOfAKind = 1,
    TwoOfAKind = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let copy_input = input.to_string();
    let (input, (hand_rank, pot)) = separated_pair(
        parse_hand_rank,
        space0,
        digit1.map(|x: &str| x.parse::<u32>().unwrap()),
    )(input)?;

    let game = Game { card: copy_input, hand_rank, pot };

    Ok((input, game))
}

fn parse_hand_rank(input: &str) -> IResult<&str, u32> {
    let mut cards = [0; 5];
    let mut chars = input.chars();
    for i in 0..5 {
        cards[i] = match chars.next() {
            Some('A') => 14,
            Some('K') => 13,
            Some('Q') => 12,
            Some('J') => 11,
            Some('T') => 10,
            Some(x) if x.is_digit(10) => x.to_digit(10).unwrap() as u8,
            _ => return Result::Err(nom::Err::Failure(Error::new(input, ErrorKind::Digit))),
        }
    }

    let hand_rank = compute_hand_rank(&cards);

    Ok((&input[5..], hand_rank))
}

fn compute_hand_rank(cards: &[u8]) -> u32 {
    let hand_rank_kind = compute_hand_rank_kind(cards) as u32;
    let lexicographic_rank = compute_lexicographic_rank(cards);

    // 4 bits for hand rank kind and 4 * 5 = 20 bits for lexicographic rank (which fits into a u32)
    (hand_rank_kind << 20) + lexicographic_rank
}

fn compute_hand_rank_kind(cards: &[u8]) -> HandRankKind {
    let mut counts: [u8; 15] = [0; 15];
    for card in cards.iter() {
        counts[*card as usize] += 1;
    }

    let (first_max, second_max) =
        counts
            .iter()
            .fold((0, 0), |(first_max, second_max), count| {
                if *count >= first_max {
                    (*count, first_max)
                } else if *count > second_max {
                    (first_max, *count)
                } else {
                    (first_max, second_max)
                }
            });

    match (first_max, second_max) {
        (5, _) => HandRankKind::FiveOfAKind,
        (4, _) => HandRankKind::FourOfAKind,
        (3, 2) => HandRankKind::FullHouse,
        (3, _) => HandRankKind::ThreeOfAKind,
        (2, _) => HandRankKind::TwoOfAKind,
        _ => HandRankKind::OneOfAKind,
    }
}

fn compute_lexicographic_rank(cards: &[u8]) -> u32 {
    let mut result = 0;
    for (index, card) in cards.iter().rev().enumerate() {
        result += (*card as u32) << (index * 4);
    }
    result
}

fn solution_1(input: &str) -> u64 {
    let mut solution: u64 = 0;
    let mut games = Vec::new();
    for line in input.lines() {
        if let Ok((_, game)) = parse_game(line) {
            games.push(game);
        }
    }

    games.sort_by_key(|x| x.hand_rank);

    for (index, game) in games.iter().enumerate() {
        solution += game.pot as u64 * (index as u64 + 1);
    }

    solution
}

fn solution_2(input: &str) -> u32 {
    0
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 7);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(&x));
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(&x));
    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);
    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            6440
        );
    }

    #[test]
    fn test_solution_1_1() {
        let input = vec![
            "1AAAA 10",
            "2AAAA 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            12
        );
    }

    #[test]
    fn test_solution_1_2() {
        let input = vec![
            "2AAAA 10",
            "1AAAA 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            21
        );
    }

    #[test]
    fn test_solution_1_3() {
        let input = vec![
            "A1AAA 10",
            "A2AAA 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            12
        );
    }

    #[test]
    fn test_solution_1_4() {
        let input = vec![
            "A2AAA 10",
            "A1AAA 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            21
        );
    }

    #[test]
    fn test_solution_1_5() {
        let input = vec![
            "1AAAA 10",
            "2KKKK 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            12
        );
    }

    #[test]
    fn test_solution_1_6() {
        let input = vec![
            "2KKKK 10",
            "1AAAA 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            21
        );
    }

    #[test]
    fn test_solution_1_7() {
        let input = vec![
            "AKKKQ 10",
            "KKAAA 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            12
        );
    }

    #[test]
    fn test_solution_1_8() {
        let input = vec![
            "12345 10",
            "54321 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            12
        );
    }

    #[test]
    fn test_solution_1_9() {
        let input = vec![
            "1122A 10",
            "2211A 1",
        ].join("\n");

        assert_eq!(
            solution_1(&input),
            12
        );
    }
}
