#![allow(non_snake_case)]


// #[path = "../src/spike.rs"]
// mod spike;

#[path = "../src/main.rs"]
mod main;

#[test]
fn takesACsvFileAndOutputsSolvedCsvFile(){
    // fail.
    // spike::readFile();
    main::main().unwrap();
    assert_eq!(true, false);
}
