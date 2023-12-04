use aoc_helper::{AocDay, Puzzle};

pub type IResult<I, O, E = String> = Result<(I, O), E>;

enum Color {
    Red,
    Green,
    Blue,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    let digit_count = input.chars().take_while(|c| c.is_digit(10)).count();
    match input
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
    {
        Ok(number) => Ok((&input[digit_count..], number)),
        Err(_) => Err("Error while parsing number".to_string()),
    }
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    match input.as_bytes().get(0) {
        Some(b'r') => Ok((&input[3..], Color::Red)),
        Some(b'g') => Ok((&input[5..], Color::Green)),
        Some(b'b') => Ok((&input[4..], Color::Blue)),
        _ => Err("Error while parsing color".to_string()),
    }
}

fn parse_color_count(input: &str) -> IResult<&str, (Color, u32)> {
    let (input, count) = parse_number(input)?;
    let input = &input[1..];
    let (input, color) = parse_color(input)?;

    Ok((input, (color, count)))
}

fn is_color_possible(color: Color, count: u32) -> bool {
    match color {
        Color::Red => count <= 12,
        Color::Green => count <= 13,
        Color::Blue => count <= 14,
    }
}

fn parse_is_color_possible(input: &str) -> IResult<&str, bool> {
    let (input, (color, count)) = parse_color_count(input)?;
    Ok((input, is_color_possible(color, count)))
}

fn parse_is_round_possible(input: &str) -> IResult<&str, bool> {
    let (mut input_result, mut possible) = parse_is_color_possible(input)?;
    while input_result.starts_with(", ") && possible {
        input_result = &input_result[2..];

        let color_possible: bool;
        (input_result, color_possible) = parse_is_color_possible(input_result)?;
        possible &= color_possible;
    }

    if !possible {
        match input_result.find(|c| c == '\n' || c == ';') {
            Some(skip_count) => input_result = &input_result[skip_count..],
            None => input_result = &input_result[input_result.len()..],
        }
    }

    Ok((input_result, possible))
}

fn parse_are_rounds_possible(input: &str) -> IResult<&str, bool> {
    let (mut input_result, mut possible) = parse_is_round_possible(input)?;
    while input_result.starts_with("; ") && possible {
        input_result = &input_result[2..];

        let round_possible: bool;
        (input_result, round_possible) = parse_is_round_possible(input_result)?;
        possible &= round_possible;
    }

    if !possible {
        match input_result.find('\n') {
            Some(skip_count) => input_result = &input_result[skip_count..],
            None => input_result = &input_result[input_result.len()..],
        }
    }

    Ok((input_result, possible))
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let input = &input[5..];
    let (input, game_id) = parse_number(input)?;
    Ok((input, game_id))
}

fn parse_is_game_possible(input: &str) -> IResult<&str, (u32, bool)> {
    let possible: bool;
    let (mut input_result, game_id) = parse_game_id(input)?;
    input_result = &input_result[2..];
    (input_result, possible) = parse_are_rounds_possible(input_result)?;
    Ok((input_result, (game_id, possible)))
}

fn parse_round_minimum_set(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let mut rgb_set = (0, 0, 0);
    let mut input_result = input;
    let mut color: Color;
    let mut count: u32;

    (input_result, (color, count)) = parse_color_count(input)?;
    match color {
        Color::Red => rgb_set.0 = rgb_set.0.max(count),
        Color::Green => rgb_set.1 = rgb_set.1.max(count),
        Color::Blue => rgb_set.2 = rgb_set.2.max(count),
    }
    while input_result.starts_with(", ") {
        input_result = &input_result[2..];
        (input_result, (color, count)) = parse_color_count(input_result)?;
        match color {
            Color::Red => rgb_set.0 = rgb_set.0.max(count),
            Color::Green => rgb_set.1 = rgb_set.1.max(count),
            Color::Blue => rgb_set.2 = rgb_set.2.max(count),
        }
    }

    Ok((input_result, rgb_set))
}

fn parse_rounds_minimum_set_power(input: &str) -> IResult<&str, u32> {
    let mut input_result = input;
    let mut round_minimum_set: (u32, u32, u32);
    let mut round_minimum_set_result: (u32, u32, u32);

    (input_result, round_minimum_set) = parse_round_minimum_set(input_result)?;
    round_minimum_set_result = round_minimum_set;

    while input_result.starts_with("; ") {
        input_result = &input_result[2..];
        (input_result, round_minimum_set) = parse_round_minimum_set(input_result)?;
        round_minimum_set_result.0 = round_minimum_set_result.0.max(round_minimum_set.0);
        round_minimum_set_result.1 = round_minimum_set_result.1.max(round_minimum_set.1);
        round_minimum_set_result.2 = round_minimum_set_result.2.max(round_minimum_set.2);
    }

    let result = round_minimum_set_result.0 * round_minimum_set_result.1 * round_minimum_set_result.2;
    Ok((input_result, result))
}

fn parse_game_minimum_set_power(input: &str) -> IResult<&str, u32> {
    let (input, _) = parse_game_id(input)?;
    let input = &input[2..];
    let (input, result) = parse_rounds_minimum_set_power(input)?;
    Ok((input, result))
}

fn solution_1(input: &str) -> u32 {
    let mut solution: u32 = 0;

    let mut input_result: &str;
    let mut game_id: u32;
    let mut possible: bool;

    (input_result, (game_id, possible)) = parse_is_game_possible(input).unwrap();

    if possible {
        solution += game_id;
    }

    while input_result.chars().nth(0) == Some('\n') {
        input_result = &input_result[1..];

        (input_result, (game_id, possible)) = parse_is_game_possible(input_result).unwrap();

        if possible {
            solution += game_id;
        }
    }

    return solution;
}

fn solution_2(input: String) -> u32 {
    let mut solution = 0;

    let mut input_result: &str;
    let mut result: u32;


    (input_result, result) = parse_game_minimum_set_power(&input).unwrap();
    solution += result;

    while input_result.chars().nth(0) == Some('\n') {
        input_result = &input_result[1..];
        (input_result, result) = parse_game_minimum_set_power(input_result).unwrap();
        solution += result;
    }

    return solution;
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 2);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(&x));
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(x));

    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);

    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);

    if let Err(err) = _result_part_1 {
        println!("Error while running part 1: {}", err);
    }

    if let Err(err) = _result_part_2 {
        println!("Error while running part 2: {}", err);
    }
}
