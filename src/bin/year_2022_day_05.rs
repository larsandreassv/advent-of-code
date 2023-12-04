struct Piles<const Width: usize, const Height: usize> {
    data: [u8; Width * Height]
    top_index: [u8; Width]
}

fn main() {


    let piles = 
}

fn parse_move(line: String) -> (u8, u8, u8) {
    let numbers = line.split("")
        .filter(|s| s.parse::<u8>().is_err())
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    return (numbers[0], numbers[1], numbers[2]);
}