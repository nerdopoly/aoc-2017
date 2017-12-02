use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process::exit;

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

pub fn get_input_or_exit() -> String {
    get_input().unwrap_or_else(|err| {
        writeln!(io::stderr(), "{}", err).unwrap();
        exit(1)
    })
}
