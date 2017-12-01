use std::io::prelude::*;
use std::io;
use std::process::exit;

mod common;

fn main() {
    let input = common::get_input().unwrap_or_else(|err| {
        writeln!(io::stderr(), "{}", err).unwrap();
        exit(1)
    });

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

    println!("{}", sum);
}
