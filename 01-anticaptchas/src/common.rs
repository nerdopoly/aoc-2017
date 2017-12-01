use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io;

pub fn get_input() -> io::Result<String> {
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
