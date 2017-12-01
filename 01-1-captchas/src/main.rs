use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::process::exit;

fn get_input() -> io::Result<String> {
    let mut input = String::new();

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let mut file = File::open(path)?;

        file.read_to_string(&mut input)?;
    } else {
        io::stdin().read_to_string(&mut input)?;
    }

    Ok(input)
}

fn main() {
    let input = get_input().unwrap_or_else(|err| {
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
