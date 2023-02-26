#![allow(non_snake_case)]

#[path = "../src/main.rs"]
mod main;

#[test]
fn takesACsvFileAndOutputsSolvedCsvFile(){
    main::main().unwrap();
    assert_eq!(true, false);
}
