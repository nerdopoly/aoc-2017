extern crate getinput;
use getinput::get_input_or_exit;

fn sum_consecutive_matches(input: &str) -> u32 {
    let mut digits = input.trim().chars()
                                 .filter_map(|c| { c.to_digit(10) })
                                 .peekable();

    let mut sum = 0;
    let first = *digits.peek().unwrap();
    while let Some(this) = digits.next() {
        let next = digits.peek().unwrap_or(&first);
        if this == *next {
            sum += this;
        }
    }

    sum
}

fn sum_opposite_matches(input: &str) -> u32 {
    let digits: Vec<_> = input.trim().chars()
                                     .filter_map(|c| { c.to_digit(10) })
                                     .collect();

    let mut sum = 0;
    let d = digits.len() / 2;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + d) % digits.len()] {
            sum += digits[i];
        }
    }

    sum
}

fn main() {
    let input = get_input_or_exit();

    println!("Part 1: {}", sum_consecutive_matches(&input));
    println!("Part 2: {}", sum_opposite_matches(&input));
}
