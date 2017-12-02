extern crate getinput;
use getinput::get_input_or_exit;

fn calc_checksum(input: String) -> usize {
    input.lines().map(|line| {
        let nums: Vec<_> = line.split_whitespace().filter_map(|num| {
            num.parse::<usize>().ok()
        }).collect();

        nums.iter().max().unwrap() - nums.iter().min().unwrap()
    }).sum()
}

fn main() {
    let input = get_input_or_exit();
    println!("Part 1: {}", calc_checksum(input));
}
