use aoc_helper::{AocDay, Puzzle};

struct RingBuffer<T> {
    buffer: Vec<T>,
    write_index: usize,
}

impl <T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(size),
            write_index: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() < self.buffer.capacity() {
            self.buffer.push(item);
        } else {
            self.buffer[self.write_index] = item;
            self.write_index = (self.write_index + 1) % self.buffer.capacity();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }
}

fn solution(input: String, marker_length: usize) -> u32 {
    let mut ring_buffer = RingBuffer::<char>::new(marker_length);

    for c in input.chars().take(marker_length - 1) {
        ring_buffer.push(c);
    }

    let solution: u32 = 0;
    
    for (i, c) in input.chars().enumerate().skip(marker_length - 1) {
        ring_buffer.push(c);

        let mut bit_array = [false; 26];

        let is_distinct = ring_buffer.iter().fold(true,|acc, c| {
            let bit_array_index = c.to_ascii_lowercase() as usize - 'a' as usize;
            let bit_array_value = bit_array[bit_array_index];
            bit_array[bit_array_index] = true;
            return acc && !bit_array_value;
        });

        if is_distinct {
            return (i + 1) as u32;
        }
    }

    return solution;
}

fn main() {
    let mut aoc_day = AocDay::new(2022, 6);
    let aoc_puzzle_part_1 = Puzzle::new(1, |x: String| solution(x, 4));
    let aoc_puzzle_part_2 = Puzzle::new(2, |x: String| solution(x, 14));

    let _result_part_1 = aoc_day.run(&aoc_puzzle_part_1);
    let _result_part_2 = aoc_day.run(&aoc_puzzle_part_2);

    _result_part_1.map_err(|err| println!("Error while running part 1: {}", err)).unwrap();
    _result_part_2.map_err(|err| println!("Error while running part 2: {}", err)).unwrap();
}