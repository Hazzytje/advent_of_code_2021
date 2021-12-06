use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

fn most_common_bit_at(readings: &[String], index: usize, result_tuple: (char, char, char)) -> char {
    let mut bit_counts : HashMap<char, u32> = HashMap::new();

    for line in readings {
        *bit_counts.entry(line.chars().nth(index).unwrap()).or_default() += 1;
    }

    return match bit_counts.get(&'1').unwrap_or(&0).cmp(bit_counts.get(&'0').unwrap_or(&0)) {
        Ordering::Greater => result_tuple.0,
        Ordering::Less => result_tuple.1,
        Ordering::Equal => result_tuple.2,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file failed");
    let input = input.lines();

    let number_of_bits = input.clone().next().unwrap().chars().count();

    let mut bit_counts : Vec<HashMap<char, u32>> = Vec::new();

    for _ in 0..number_of_bits {
        bit_counts.push(HashMap::new());
    }

    for line in input.clone() {
        for (i, c) in line.chars().enumerate() {
            *bit_counts[i].entry(c).or_default() += 1;
        }
    }

    let mut most_common_bits = 0;
    let mut least_common_bits = 0;

    let mut most_common_bits_vec : Vec<char> = Vec::new();
    let mut least_common_bits_vec : Vec<char> = Vec::new();

    for bit_count in bit_counts {
        most_common_bits <<= 1;
        least_common_bits <<= 1;

        if bit_count.get(&'1').unwrap() > bit_count.get(&'0').unwrap() {
            most_common_bits |= 1;
            most_common_bits_vec.push('1');
            least_common_bits_vec.push('0');
        } else {
            least_common_bits |= 1;
            least_common_bits_vec.push('1');
            most_common_bits_vec.push('0');
        }
    }

    println!("least * most common bits = {}", most_common_bits * least_common_bits);

    // Part 2
    let mut oxygen_generator_candidates : Vec<String> = input.clone().map(str::to_string).collect();
    let mut co2_scrubber_candidates : Vec<String> = input.map(str::to_string).collect();

    let mut oxygen_generator : Option<u32> = None;
    let mut co2_scrubber : Option<u32> = None;

    for bit in 0..number_of_bits {
        let oxygen_bit = most_common_bit_at(oxygen_generator_candidates.as_slice(), bit, ('1', '0', '1'));
        let co2_bit = most_common_bit_at(co2_scrubber_candidates.as_slice(), bit, ('0', '1', '0'));

        oxygen_generator_candidates.retain(|x| {
            x.chars().nth(bit).unwrap() == oxygen_bit
        });
        co2_scrubber_candidates.retain(|x| {
            x.chars().nth(bit).unwrap() == co2_bit
        });

        if oxygen_generator_candidates.len() == 1 {
            oxygen_generator = u32::from_str_radix(&oxygen_generator_candidates[0], 2).ok();
            println!("oxygen gen: {:?}, {}", &oxygen_generator_candidates[0], oxygen_generator.unwrap());
        }
        if co2_scrubber_candidates.len() == 1 {
            co2_scrubber = u32::from_str_radix(&co2_scrubber_candidates[0], 2).ok();
            println!("co2 scrubber: {:?}, {}",&co2_scrubber_candidates[0], co2_scrubber.unwrap());
        }
    }

    println!("oxygen * co2 = {}", oxygen_generator.unwrap() * co2_scrubber.unwrap());
}
