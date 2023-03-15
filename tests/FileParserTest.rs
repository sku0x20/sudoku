#![allow(non_snake_case)]

use std::fs;
use std::fs::File;
use std::io::Result;
use std::panic::catch_unwind;
use std::path::Path;

use crate::TestFixture::{withFixture, withTestFixture};

mod TestFixture;

#[path = "../src/FileParser.rs"]
mod FileParser;

// for now making FileParse as a helper/util class

const TEMP_FILE_PATH: &str = "./resources/temp_file.csv";

#[test]
fn parsesCsvFile() {
    let sampleData = Vec::from(SAMPLE_DATA);
    let parsedVec = FileParser::parseFile("./resources/test_puzzle.csv");
    assert_eq!(parsedVec, sampleData)
}

#[test]
fn panicsOnInvalidDigits() {
    // only 0-9 are valid; 0 representing empty
    withFixture(setUp, tearDown, || {
        let result = catch_unwind(|| {
            writeToFile("1,2,3,4,0,a");
            FileParser::parseFile(TEMP_FILE_PATH);
        });
        assert!(result.is_err());

        let result = catch_unwind(|| {
            writeToFile("1,2,3,4,0,-1");
            FileParser::parseFile(TEMP_FILE_PATH);
        });
        assert!(result.is_err());

        let result = catch_unwind(|| {
            writeToFile("1,2,3,4,0,10");
            FileParser::parseFile(TEMP_FILE_PATH);
        });
        assert!(result.is_err());
    })
}

fn setUp() {
    writeToFile("");
}

fn tearDown() {
    fs::remove_file(TEMP_FILE_PATH).expect("temp file should be removed");
}

fn writeToFile(data: &str) {
    fs::write(TEMP_FILE_PATH, data).expect("unable to write to temp file");
}

const SAMPLE_DATA: [u8; 81] = [
    0, 0, 8, 0, 6, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 7, 0, 3, 0,
    6, 1, 0, 5, 0, 0, 2, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 5, 0, 0, 0, 6,
    2, 9, 0, 6, 0, 0, 1, 0, 0,
    9, 8, 0, 0, 3, 0, 0, 2, 0,
    0, 0, 4, 9, 0, 0, 0, 0, 0,
    0, 0, 5, 0, 0, 0, 8, 0, 0
];