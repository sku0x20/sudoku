#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::io::Result;

#[allow(dead_code)]
fn main() -> Result<()> {
    runApp()
}

pub fn runApp() -> Result<()> {
    let file = File::open("./resources/test_puzzle.csv")?;
    let mut bufferedReader = BufReader::new(file);
    let mut contents = String::new();

    bufferedReader.read_to_string(&mut contents)?;

    println!("{contents}");

    let file = File::create("./resources/test_puzzle_solved.csv")?;
    let mut bufferedWriter = BufWriter::new(file);

    bufferedWriter.write_all(b"Test")?;

    bufferedWriter.flush()?;

    Ok(())
}
