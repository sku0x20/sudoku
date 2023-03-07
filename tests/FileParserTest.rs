#![allow(non_snake_case)]

use std::{fs, thread};
use std::io::Result;
use std::path::Path;

#[path = "../src/FileParser.rs"]
mod FileParser;

// for now making FileParse as a helper/util class

const TEMP_FILE_PATH: &str = "./resources/temp_file.csv";

#[test]
fn parsePanics() {
    createFile();

    let handle = thread::spawn(|| {
        FileParser::parse(TEMP_FILE_PATH);
    });
    let result = handle.join();

    assert!(result.is_err(), "should be Result::Err type");

    removeFile()
}

fn createFile() {
    fs::write(TEMP_FILE_PATH, "test").expect("unable to write to temp file")
}

fn removeFile() {
    fs::remove_file(TEMP_FILE_PATH).expect("temp file should be removed")
}