use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file failed");
    let input = input.lines();

    let number_of_bits = input.clone().next().unwrap().chars().count();

    let mut bit_counts : Vec<HashMap<char, u32>> = Vec::new();

    for _ in 0..number_of_bits {
        bit_counts.push(HashMap::new());
    }

    for line in input {
        for (i, c) in line.chars().enumerate() {
            *bit_counts[i].entry(c).or_default() += 1;
        }
    }

    let mut most_common_bits = 0;
    let mut least_common_bits = 0;

    for bit_count in bit_counts {
        most_common_bits <<= 1;
        least_common_bits <<= 1;

        if bit_count.get(&'1').unwrap() > bit_count.get(&'0').unwrap() {
            most_common_bits |= 1;
        } else {
            least_common_bits |= 1;
        }
    }

    println!("least * most common bits = {}", most_common_bits * least_common_bits);
}
