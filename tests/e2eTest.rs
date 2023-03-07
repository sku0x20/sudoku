#![allow(non_snake_case)]

use std::fs;
use std::io::Result;
use std::path::Path;

#[path = "../src/main.rs"]
mod main;

// e2e means use cases

#[test]
fn solvesSudoku() -> Result<()> {
    let args = vec![
        "--solve".to_string(),
        "./resources/test_puzzle.csv".to_string(),
    ];

    main::runApp(args)?;

    let solvedFilePath = Path::new("./resources/test_puzzle_solved.csv");
    assert!(solvedFilePath.exists());

    let expectedSolvedString = fs::read_to_string("./resources/test_puzzle_solved_for_e2e.csv")?;
    let actualSolvedString = fs::read_to_string(solvedFilePath)?;

    assert_eq!(expectedSolvedString, actualSolvedString);

    fs::remove_file(solvedFilePath)?;
    Ok(())
}
