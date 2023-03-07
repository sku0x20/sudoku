#![allow(non_snake_case)]

use std::fs;
use std::io::Result;
use std::panic::catch_unwind;
use std::path::Path;

#[path = "../src/FileParser.rs"]
mod FileParser;

// for now making FileParse as a helper/util class

const TEMP_FILE_PATH: &str = "./resources/temp_file.csv";

#[test]
fn parsePanics() {
    createFile();

    let result = catch_unwind(|| {
        FileParser::parse(TEMP_FILE_PATH);
    });

    assert!(result.is_err(), "should be Result::Err type");

    removeFile()
}

fn createFile() {
    fs::write(TEMP_FILE_PATH, "test").expect("unable to write to temp file")
}

fn removeFile() {
    fs::remove_file(TEMP_FILE_PATH).expect("temp file should be removed")
}