use std::fs;

enum Gender {
    Male,
    Female
}

struct Elf {
    name: String,
    age: u32,
    gender: Gender,
    production: f64,
}

fn parse_elf(line: &str) -> Option<Elf> {
    let mut parts = line.split(", ");

    if let (Some(name), Some(age), Some(gender), Some(production)) = (
        parts.next().unwrap().split(": ").nth(1),
        parts.next().unwrap().split(": ").nth(1),
        parts.next().unwrap().split(": ").nth(1),
        parts.next().unwrap().split(": ").nth(1)
    ) {
        let decimal_length = 12;
        let mut seen_decimal = false;
        let mut decimal_index = 0;
        let production = production
            .replace(",", ".")
            .chars()
            .fold(String::new(), |mut acc, c| {
                if c == '.' {
                    seen_decimal = true;
                    acc.push(c);
                } else if !seen_decimal {
                    acc.push(c);
                } else if decimal_index < decimal_length {
                    acc.push(c);
                    decimal_index += 1;
                }
                acc
            });
        if let Ok(age) = age.parse::<u32>() {
            if let Ok(production) = production.parse::<f64>() {
                println!("{}: {}", name, production);
                return Some(Elf {
                    name: name.to_string(),
                    age: age,
                    gender: match gender {
                        "Male" => Gender::Male,
                        "Female" => Gender::Female,
                        _ => Gender::Male
                    },
                    production: production,
                });
            }
        }
    }

    None
}

fn main() {
    //read input from file
    let mut input = fs::read_to_string("input.txt").unwrap();

    let elfs: Vec<Elf> = input
        .split("\n")
        .map(|line| parse_elf(line).unwrap())
        .collect();


    for (i, elf) in elfs.iter().enumerate() {
        println!("{}: {}", i, elf.name);
    }
}