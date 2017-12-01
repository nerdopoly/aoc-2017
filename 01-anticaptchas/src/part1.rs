use std::io::prelude::*;
use std::io;
use std::process::exit;

mod common;

fn main() {
    let input = common::get_input().unwrap_or_else(|err| {
        writeln!(io::stderr(), "{}", err).unwrap();
        exit(1)
    });

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

    println!("{}", sum);
}
