use core::num;
use std::vec;

use aoc_helper::{AocDay, Puzzle};

struct BitTable {
    bit_map: Vec<bool>,
    width: u32,
    height: u32,
}

impl BitTable {
    fn new(width: u32, height: u32) -> Self {
        Self {
            bit_map: vec![false; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    fn set(&mut self, x: u32, y: u32, value: bool) {
        if x < self.width && y < self.height {
            let index: usize = (x + y * self.width) as usize;
            self.bit_map[index] = value;
        }
    }

    fn set_around(&mut self, x: u32, y: u32, value: bool) {
        match (x, y) {
            (0, 0) => {
                self.set(x, y, value);
                self.set(x + 1, y, value);
                self.set(x, y + 1, value);
                self.set(x + 1, y + 1, value);
            }
            (0, _) => {
                self.set(x, y, value);
                self.set(x, y - 1, value);
                self.set(x, y + 1, value);
                self.set(x + 1, y - 1, value);
                self.set(x + 1, y, value);
                self.set(x + 1, y + 1, value);
            }
            (_, 0) => {
                self.set(x, y, value);
                self.set(x - 1, y, value);
                self.set(x + 1, y, value);
                self.set(x - 1, y + 1, value);
                self.set(x, y + 1, value);
                self.set(x + 1, y + 1, value);
            }
            (_, _) => {
                self.set(x, y, value);
                self.set(x - 1, y - 1, value);
                self.set(x, y - 1, value);
                self.set(x + 1, y - 1, value);
                self.set(x - 1, y, value);
                self.set(x + 1, y, value);
                self.set(x - 1, y + 1, value);
                self.set(x, y + 1, value);
                self.set(x + 1, y + 1, value);
            }
        }
    }

    fn get(&self, x: u32, y: u32) -> bool {
        if x <= self.width && y < self.height {
            let index: usize = (x + y * self.width) as usize;
            self.bit_map[index]
        } else {
            false
        }
    }
}

pub type IResult<I, O, E = String> = Result<(I, O), E>;

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

fn solution_1(input: &str) -> u32 {
    let mut solution = 0;

    let line_length = input.lines().nth(0).unwrap().len();
    let mut bit_map: BitTable = BitTable::new(line_length as u32, input.lines().count() as u32);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                bit_map.set_around(x as u32, y as u32, true);
            }
        }
    }

    let mut input_result = &input[..];

    let mut index = 0;
    while input_result.len() > 0 {
        let c = input_result.chars().nth(0).unwrap();
        match c {
            c if c.is_numeric() => {
                let x = index % line_length;
                let y = index / line_length;

                let number: u32;
                let digit_count = input_result.chars().take_while(|c| c.is_digit(10)).count();

                (input_result, number) = parse_number(input_result).unwrap();

                let is_included = (0..digit_count).fold(false, |acc, next| {
                    acc || bit_map.get(x as u32 + next as u32, y as u32)
                });

                if is_included {
                    solution += number;
                }

                index += digit_count;
            }
            '\n' => {
                input_result = &input_result[1..];
            }
            _ => {
                input_result = &input_result[1..];
                index += 1;
            }
        }
    }

    solution
}

fn solution_2(input: String) -> u32 {
    let mut solution = 0;

    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut index = 0;
    while width * height > index {
        let x = index % width;
        let y = index / width;
        let c = lines[y].chars().nth(x).unwrap();
        match c {
            '*' => {
                let mut numbers: Vec<u32> = Vec::new();
                let mut checked_indicies = Vec::new();

                let x_range = || match x {
                    0 => 0..1,
                    s if s == width - 1 => x - 1..x,
                    _ => x - 1..x + 1,
                };

                let y_range = || match y {
                    0 => 0..1,
                    s if s == height - 1 => y - 1..y,
                    _ => y - 1..y + 1,
                };

                for x in x_range() {
                    for y in y_range() {
                        let is_numeric = lines[y].chars().nth(x).unwrap().is_numeric();
                        let is_checked = checked_indicies.contains(&(x + y * width));
                        if is_numeric && !is_checked
                        {
                            let mut left_offset = 0;
                            let mut right_offset = 0;
                            
                            while x + right_offset + 1 < width {
                                if let Some(c) = lines[y].chars().nth(x + right_offset + 1) {
                                    if c.is_numeric() {
                                        right_offset += 1;
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }

                            while x - left_offset > 0 {
                                if let Some(c) = lines[y].chars().nth(x - left_offset - 1) {
                                    if c.is_numeric() {
                                        left_offset += 1;
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }

                            let number_string = lines[y]
                                .chars()
                                .skip(x - left_offset)
                                .take(left_offset + right_offset + 1)
                                .collect::<String>();

                            let number = number_string
                                .parse::<u32>()
                                .unwrap();

                            numbers.push(number);
                            for i in x - left_offset..x + right_offset + 1 {
                                checked_indicies.push(i + y * width);
                            }
                        }
                    }
                }

                if numbers.len() == 2 {
                    solution += numbers.iter().product::<u32>();
                }

                index += 1;
            }
            '\n' => (),
            _ => {
                index += 1;
            }
        }
    }

    solution
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 3);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(&x)).with_examples(&[
        vec![".....", ".123.", "....."].join("\n"),
        vec![".....", "*123.", "....."].join("\n"),
        vec![".....", ".123*", "....."].join("\n"),
        vec!["*....", ".123.", "....."].join("\n"),
        vec![".*...", ".123.", "....."].join("\n"),
        vec!["..*..", ".123.", "....."].join("\n"),
        vec!["...*.", ".123.", "....."].join("\n"),
        vec!["....*", ".123.", "....."].join("\n"),
        vec![".....", ".123.", "*...."].join("\n"),
        vec![".....", ".123.", ".*..."].join("\n"),
        vec![".....", ".123.", "..*.."].join("\n"),
        vec![".....", ".123.", "...*."].join("\n"),
        vec![".....", ".123.", "....*"].join("\n"),
        vec![".....", "1*23.", "....."].join("\n"),
        vec![".....", "12*3.", "....."].join("\n"),
        vec!["*....", ".123.", "....*"].join("\n"),
    ]);
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(x)).with_examples(&[
        vec![".....", "11*32", "....."].join("\n"),
        vec![".....", ".12*.", "....."].join("\n"),
        vec![".....", ".123*", "...32"].join("\n"),
    ]);

    let _test_result_part_1 = aoc_day.test(&aoc_puzzle_part_1);
    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);

    let _test_result_part_2 = aoc_day.test(&aoc_puzzle_part_2);
    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);
}
