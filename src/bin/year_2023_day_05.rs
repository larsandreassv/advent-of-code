use std::vec;

use aoc_helper::{AocDay, Puzzle};

#[derive(Clone, Copy)]
struct Interval {
    start: u64,
    width: u64,
}

impl Interval {
    fn new(start: u64, width: u64) -> Self {
        Interval { start, width }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.start + other.width && other.start < self.start + self.width
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            let start = self.start.max(other.start);
            let end = (self.start + self.width).min(other.start + other.width);
            Some(Interval::new(start, end - start))
        } else {
            None
        }
    }

    
}


// struct IntervalTree<T: Copy> {
//     interval: Interval<T>,
//     max: u64,
//     left: Option<Box<IntervalTree<T>>>,
//     right: Option<Box<IntervalTree<T>>>,
// }

// impl<T: Copy> IntervalTree<T> {
//     fn new(interval: Interval<T>) -> Self {
//         IntervalTree {
//             interval,
//             max: interval.end,
//             left: None,
//             right: None,
//         }
//     }

//     fn insert(&mut self, interval: Interval<T>) {
//         if self.interval.start > interval.start {
//             if let Some(left_node) = &mut self.left {
//                 left_node.insert(interval);
//             } else {
//                 self.left = Some(Box::new(IntervalTree::new(interval)));
//             }
//         } else {
//             if let Some(right_node) = &mut self.right {
//                 right_node.insert(interval);
//             } else {
//                 self.right = Some(Box::new(IntervalTree::new(interval)));
//             }
//         }

//         self.max = self.max.max(interval.end);
//     }

//     fn find_value(&self, value: u64) -> Option<&Interval<T>> {
//         if self.interval.start < value && value <= self.interval.end {
//             return Some(&self.interval);
//         }

//         if let Some(left_node) = &self.left {
//             return left_node.find_value(value);
//         }

//         if let Some(right_node) = &self.right {
//             if right_node.max > value {
//                 return right_node.find_value(value);
//             }
//         }

//         None
//     }
// }

fn parse_seeds(input: &str) -> Vec<u64> {
    let mut seeds = Vec::new();
    let first_line = input.lines().next().unwrap();
    let first_line = first_line.split(":").nth(1).unwrap();
    for seed in first_line.split_whitespace().filter(|x| !x.is_empty()) {
        seeds.push(seed.parse::<u64>().unwrap());
    }
    seeds
}

// fn parse_seed_ranges(input: &str) -> Vec<std::ops::Range<u64>> {
//     let mut seeds = Vec::new();
//     let first_line = input.lines().next().unwrap();
//     let first_line = first_line.split(":").nth(1).unwrap();
//     for seed in first_line.split_whitespace().filter(|x| !x.is_empty()) {
//         let start_index = seed.parse::<u64>().unwrap();
//         let width = seed.parse::<u64>().unwrap();
//         seeds.push(start_index..start_index + width);
//     }
//     seeds
// }

fn parse_seed_intervals(input: &str) -> Vec<Interval> {
    let mut seeds = Vec::new();
    let first_line = input.lines().next().unwrap();
    let first_line = first_line.split(":").nth(1).unwrap();
    for seed in first_line.split_whitespace().filter(|x| !x.is_empty()) {
        let start_index = seed.parse::<u64>().unwrap();
        let width = seed.parse::<u64>().unwrap();
        seeds.push(Interval::new(start_index, width - start_index));
    }
    seeds
}

// fn parse_map(input: String) -> Vec<(u64, u64, u64)> {
//     let mut map = Vec::new();
//     let mut lines = input.lines();
//     lines.next();
//     for line in lines {
//         let mut parts = line.split_whitespace().filter(|x| !x.is_empty());
//         let target = parts.next().unwrap().parse::<u64>().unwrap();
//         let source = parts.next().unwrap().parse::<u64>().unwrap();
//         let width = parts.next().unwrap().parse::<u64>().unwrap();
//         map.push((target, source, width))
//     }
//     map
// }

// fn parse_map(input: String) -> IntervalTree<u64> {
//     let mut interval_tree: IntervalTree<u64> = IntervalTree::new(Interval::new(0, 0, 0));
//     let mut lines = input.lines();
//     lines.next();
//     for line in lines {
//         let mut parts = line.split_whitespace().filter(|x| !x.is_empty());
//         let target = parts.next().unwrap().parse::<u64>().unwrap();
//         let source = parts.next().unwrap().parse::<u64>().unwrap();
//         let width = parts.next().unwrap().parse::<u64>().unwrap();
//         interval_tree.insert(Interval::new(source, source + width, target));
//     }
//     interval_tree
// }

// fn parse_maps(input: &str) -> Vec<IntervalTree<u64>> {
//     let mut maps = Vec::new();
//     for map in input.split("\n\n") {
//         maps.push(parse_map(map.to_string()));
//     }
//     maps
// }

fn parse_map_interval(input: String) -> Vec<(Interval, Interval)> {
    let mut map = Vec::new();
    let mut lines = input.lines();
    lines.next();
    for line in lines {
        let mut parts = line.split_whitespace().filter(|x| !x.is_empty());
        let target = parts.next().unwrap().parse::<u64>().unwrap();
        let source = parts.next().unwrap().parse::<u64>().unwrap();
        let width = parts.next().unwrap().parse::<u64>().unwrap();
        map.push((Interval::new(source, width), Interval::new(target, width)));
    }
    map
}

fn parse_map_intervals(input: &str) -> Vec<Vec<(Interval, Interval)>> {
    let mut maps = Vec::new();
    for map in input.split("\n\n") {
        maps.push(parse_map_interval(map.to_string()));
    }
    maps
}

// fn solution_1(input: &str) -> u64 {
//     let mut seeds = parse_seeds(input);
//     let maps = parse_maps(input);

//     maps.iter().fold(seeds, |mut acc, map| {
//         for (i, seed) in acc.iter_mut().enumerate() {
//             for (target, source, offset) in map.iter() {
//                 let mut source_end = source + offset;
//                 if source <= seed && seed < &mut source_end {
//                     *seed = (target + *seed) - source;
//                     break;
//                 }
//             }
//         }
//         acc
//     }).iter().min_by(|a, b| a.cmp(b)).unwrap().clone()

// }

fn solution_1(input: &str) -> u64 {
    // let mut seeds = parse_seeds(input);
    // let maps = parse_maps(input);

    // let mut minimum_land = u64::max_value();
    // for seed in seeds {
    //     let mut result = seed;
    //     for map in &maps {
    //         if let Some(interval) = map.find_value(result) {
    //             result = interval.value + result - interval.start;
    //         }
    //     }
    //     minimum_land = minimum_land.min(result);
    // }

    // minimum_land
    0
}

// fn solution_2(input: &str) -> u64 {
//     let seed_ranges = parse_seed_ranges(input);
//     let maps = parse_maps(input);

//     let mut range_count = 0;
//     let mut minimum_land = u64::max_value();
//     for seed_range in seed_ranges {
//         for seed in seed_range {
//             let mut result = seed;
//             for map in &maps {
//                 for (target, source, offset) in map {
//                     let mut source_end = source + offset;
//                     if source <= &result && &result < &mut source_end {
//                         result = (target + result) - source;
//                         break;
//                     }
//                 }
//             }
//             minimum_land = minimum_land.min(result);
//         }
//         range_count += 1;
//         println!("{}", range_count);
//     }

//     minimum_land
// }

fn solution_2(input: &str) -> u64 {
    // let seed_ranges = parse_seed_ranges(input);
    // let maps = parse_maps(input);

    let seed_intervals: Vec<Interval> = parse_seed_intervals(input);
    let interval_maps: Vec<Vec<(Interval, Interval)>> = parse_map_intervals(input);

    let mut minimum_land = u64::max_value();
    for seed_interval in seed_intervals {
        let mut source_intervals = vec![seed_interval];
        let mut target_intervals = Vec::new();

        for map in &interval_maps {
            for (map_source_interval, map_target_interval) in map {
                // map source intervals to target intervals
                for source_interval in &source_intervals {
                    if let Some(intersect) = source_interval.intersect(&map_source_interval) {
                        target_intervals.push(Interval::new(
                            map_target_interval.start + intersect.start - map_source_interval.start,
                            intersect.width,
                        ));
                    }
                }

                source_intervals = target_intervals;
                target_intervals = Vec::new();
            }

            source_intervals = target_intervals;
            target_intervals = Vec::new();
        }

        for source_interval in source_intervals {
            minimum_land = minimum_land.min(source_interval.start);
        }
    }

    // let mut range_count = 0;
    // let mut minimum_land = u64::max_value();
    // for seed_range in seed_ranges {
    //     for seed in seed_range {
    //         let mut result = seed;
    //         for map in &maps {
    //             if let Some(interval) = map.find_value(result) {
    //                 result = interval.value + result - interval.start;
    //             }
    //         }
    //         minimum_land = minimum_land.min(result);
    //     }
    //     range_count += 1;
    //     println!("{}", range_count);
    // }

    minimum_land
}

fn main() {
    let mut aoc_day = AocDay::new(2023, 5);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution_1(&x));
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution_2(&x));
    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);
    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seeds_test() {
        assert!(
            parse_seeds("seeds: 79 14 55 13").eq(&vec![79, 14, 55, 13]),
            "parse_seeds_test failed"
        );
    }

    #[test]
    fn parse_seed_ranges_test() {
        assert_eq!(
            parse_seed_ranges("seeds: 79 14 55 13"),
            vec![79..93, 55..69]
        )
    }

    #[test]
    fn parse_map_test() {
        let input = vec!["seed-to-soil map:", "50 98 2", "52 50 48"].join("\n");

        assert!(
            parse_map(input).eq(&vec![(50..52, 98..100), (52..100, 50..98)]),
            "parse_map_test failed"
        );
    }

    #[test]
    fn parse_maps_test() {
        let input = vec![
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-water map:",
            "50 98 2",
            "52 50 48",
        ]
        .join("\n");

        assert!(
            parse_maps(&input).eq(&vec![
                vec![(50..52, 98..100), (52..100, 50..98)],
                vec![(50..52, 98..100), (52..100, 50..98)]
            ]),
            "parse_maps_test failed"
        );
    }

    #[test]
    fn solution_1_test() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .join("\n");

        assert_eq!(solution_1(&input), 35, "solution_1_test failed");
    }
}
