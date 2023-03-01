#![allow(non_snake_case)]

use std::fs;
use std::io::Result;
use std::path::Path;

#[path = "../src/main.rs"]
mod main;

#[test]
fn takesACsvFileAndOutputsSolvedCsvFile() -> Result<()> {
    let args = vec![
        "--solve".to_string(),
        "./resources/test_name.csv".to_string(),
    ];

    main::runApp(args)?;

    let solvedFilePath = Path::new("./resources/test_name_solved.csv");
    assert!(solvedFilePath.exists());



    fs::remove_file(solvedFilePath)?;
    Ok(())
}
