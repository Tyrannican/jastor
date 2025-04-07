use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod error;
mod event;

use error::JastorError;
use event::{Event, parse_event};

fn main() -> Result<(), JastorError> {
    let fh = File::open("combat.log").map_err(|e| JastorError::FileReadError(e.to_string()))?;
    let reader = BufReader::new(fh);

    let mut counter = 0;
    for line in reader.lines() {
        let line = line.map_err(|e| JastorError::ParseError(e.to_string()))?;
        parse_event(line)?;
        counter += 1;
    }

    println!("Total events parsed: {counter}");

    Ok(())
}
