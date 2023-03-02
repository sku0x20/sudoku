#![allow(non_snake_case)]

#[path = "../src/Sudoku.rs"]
mod Sudoku;

#[test]
fn parsesFromString(){
    // nope i think, parser should take care of all the cases and checks.
    assert_eq!(1, 2)
}