#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Result;

mod spike;

pub fn main() -> Result<()> {
    let mut file = File::open("./resources/test_puzzle.csv")?;
    let mut bufferedReader = BufReader::new(file);
    let mut contents = String::new();

    bufferedReader.read_to_string(&mut contents)?;

    println!("{contents}");

    spike::readFile();

    Ok(())
}
