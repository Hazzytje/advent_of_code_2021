use std::fs;

#[derive(Debug)]
struct BingoCard {
    pub digits : Vec<Vec<(u32, bool)>>,
    pub last_call : Option<u32>,
}

impl BingoCard {
    pub fn new(digits_str : &str) -> BingoCard {
        BingoCard { digits: digits_str.lines().map(|bingo_card_line|
            bingo_card_line.split_whitespace().map(|bingo_digit|
                (bingo_digit.parse::<u32>().unwrap(), false)
            ).collect()
        ).collect(), last_call: None}
    }

    pub fn call(&mut self, new_call : u32) {
        self.last_call = Some(new_call);
        for line in &mut self.digits {
            for (digit, called) in line.iter_mut() {
                if *digit == new_call {
                    *called = true;
                }
            }
        }
    }

    pub fn has_bingo(&self) -> bool {
        for row in &self.digits {
            if row.into_iter().all(|(_, called)| *called) {
                return true;
            }
        }
        for column in 0..5 {
            if (&self.digits).into_iter().all(|row| row[column].1) {
                return true;
            }
        }

        return false;
    }

    pub fn score(&self) -> u32 {
        self.digits.iter().flatten().filter(|(_, called)| !called).map(|board| board.0).sum::<u32>() * self.last_call.unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file failed");
    let mut input = input.split("\n\n");

    let bingo_draws = input.next().unwrap();
    let bingo_draws : Vec<u32> = bingo_draws.split(',').map(|n| n.parse::<u32>().unwrap()).collect();

    let calls_and_scores : Vec<(usize, u32)> = input.map(|bingo_card_digits| {
        let mut bingo_card = BingoCard::new(bingo_card_digits);
        for (draw_index, bingo_draw) in bingo_draws.iter().enumerate() {
            (&mut bingo_card).call(*bingo_draw);
            if bingo_card.has_bingo() {
                return (draw_index, bingo_card.score());
            }
        }
        return (9999999, 0);
    }).collect();

    let score_for_first_bingo = calls_and_scores.iter().min_by_key(|board| board.0).map(|(_, score)| score).unwrap();

    println!("Score for first bingo: {}", score_for_first_bingo);

    let score_for_last_bingo = calls_and_scores.iter().max_by_key(|board| board.0).map(|(_, score)| score).unwrap();

    println!("Score for last bingo: {}", score_for_last_bingo);
}
