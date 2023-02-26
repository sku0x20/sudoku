#![allow(non_snake_case)]

use std::fs;
use std::path::Path;

#[path = "../src/main.rs"]
mod main;

#[test]
fn takesACsvFileAndOutputsSolvedCsvFile() {
    main::runApp().unwrap();
    let solvedFile = Path::new("./resources/test_puzzle_solved.csv");
    assert!(solvedFile.exists());
    fs::remove_file(solvedFile).unwrap();
}
