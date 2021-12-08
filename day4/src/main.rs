use std::fs;

#[derive(Debug)]
struct BingoCard {
    pub digits : Vec<Vec<u32>>
}

impl BingoCard {
    pub fn new(digits_str : &str) -> BingoCard {
        BingoCard { digits: digits_str.lines().map(|bingo_card_line|
            bingo_card_line.split_whitespace().map(|bingo_digit|
                bingo_digit.parse::<u32>().unwrap()
            ).collect()
        ).collect()}
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file failed");
    let mut input = input.split("\n\n");

    let bingo_draws = input.next();

    for bingo_card in input {
        println!("{:?}", BingoCard::new(bingo_card));
    }
}
