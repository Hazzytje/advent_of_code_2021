use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file failed");
    let input = input.lines();
    let input : Vec<u32> = input.map(|n| n.parse::<u32>().unwrap()).collect();

    let ans1 = input.windows(2).filter(|w| w[1] > w[0]).count();
    println!("Numbet of increasing numbers is {}!", ans1);

    let sum_of_windows : Vec<u32> = input.windows(3).map(|w| w.iter().sum()).collect();
    let ans2 = sum_of_windows.windows(2).filter(|w| w[1] > w[0]).count();
    println!("Numbet of increasing windows is {}!", ans2);
}
