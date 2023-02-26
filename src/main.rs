#![allow(non_snake_case)]

use std::fs;
use std::io::Result;

#[allow(dead_code)]
fn main() -> Result<()> {
    runApp()
}

pub fn runApp() -> Result<()> {
    fs::write("./resources/test_puzzle_solved.csv", "test")?;

    Ok(())
}
