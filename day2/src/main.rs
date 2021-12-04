use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file failed");
    let input = input.lines();

    let regex = Regex::new(r"^([a-z]+) (\d+)$").expect("Regex failed to compile");

    let input : Vec<(String, u32)> = input.map(|line| {
        regex.captures(line).map(|captures| {
            (
                captures[1].to_string(),
                captures[2].parse::<u32>().unwrap()
            )
        }).unwrap()
    }).collect();

    // Part 1
    let mut command_sum : HashMap<String, u32> = HashMap::new();
    for (command, count) in &input {
        *command_sum.entry(command.to_string()).or_insert(0) += count;
    }

    let depth = command_sum.get("down").unwrap() - command_sum.get("up").unwrap();
    println!("Part 1: depth * horizontal position = {}", depth * command_sum.get("forward").unwrap());

    // Part 2
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_pos = 0;
    for (command, count) in input {
        match command.as_str() {
            "down" => {
                aim += count;
            },
            "up" => {
                aim -= count;
            },
            "forward" => {
                horizontal_pos += count;
                depth += aim * count;
            },
            _ => { panic!(); }
        }
    }

    println!("Part 2: depth * horizontal position = {}", depth * horizontal_pos);
}
