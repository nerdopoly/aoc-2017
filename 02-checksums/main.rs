extern crate getinput;
use getinput::get_input_or_exit;

fn calc_checksum(input: &str) -> usize {
    input.lines().map(|line| {
        let nums: Vec<_> = line.split_whitespace().filter_map(|num| {
            num.parse::<usize>().ok()
        }).collect();

        nums.iter().max().unwrap() - nums.iter().min().unwrap()
    }).sum()
}

fn calc_divsum(input: &str) -> usize {
    input.lines().filter_map(|line| {
        let mut nums = line.split_whitespace()
                       .filter_map(|num| {
                            num.parse::<usize>().ok()
                       })
                       .collect::<Vec<_>>();
        nums.sort_by(|a, b| b.cmp(a)); // reverse sort numbers

        for (i, numerator) in nums.iter().enumerate() {
            for divisor in &nums[i+1..] {
                if numerator % divisor == 0 {
                    return Some(numerator/divisor)
                }
            }
        }

        None
    }).sum()
}

fn main() {
    let input = get_input_or_exit();
    println!("Part 1: {}", calc_checksum(&input));
    println!("Part 2: {}", calc_divsum(&input));
}
