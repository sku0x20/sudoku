#![allow(non_snake_case)]

use std::fs;
use std::path::Path;
use std::io::Result;

#[path = "../src/main.rs"]
mod main;

#[test]
fn takesACsvFileAndOutputsSolvedCsvFile() -> Result<()>{
    main::runApp()?;
    let solvedFilePath = Path::new("./resources/test_puzzle_solved.csv");
    assert!(solvedFilePath.exists());
    fs::remove_file(solvedFilePath)?;

    Ok(())
}
