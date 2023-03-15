#![allow(non_snake_case)]

use std::fs;
use std::io::Result;
use std::panic::catch_unwind;
use std::path::Path;

use crate::TestFixture::withTestFixture;

mod TestFixture;

#[path = "../src/FileParser.rs"]
mod FileParser;

// for now making FileParse as a helper/util class

const TEMP_FILE_PATH: &str = "./resources/temp_file.csv";

#[test]
#[should_panic]
fn parsePanics() {
    withTestFixture(setUp, tearDown, || {
        FileParser::parse(TEMP_FILE_PATH);
    })
}

fn setUp() {
    fs::write(TEMP_FILE_PATH, "test").expect("unable to write to temp file")
}

fn tearDown() {
    fs::remove_file(TEMP_FILE_PATH).expect("temp file should be removed")
}