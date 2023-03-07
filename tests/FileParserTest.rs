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
#[should_panic]
fn parsePanics() {
    let testFixture = TestFixture::scoped();

    FileParser::parse(TEMP_FILE_PATH);
}

fn createFile() {
    fs::write(TEMP_FILE_PATH, "test").expect("unable to write to temp file")
}

fn removeFile() {
    fs::remove_file(TEMP_FILE_PATH).expect("temp file should be removed")
}

// support for test fixture
// https://stackoverflow.com/a/38254435
struct TestFixture {}

impl TestFixture {
    fn scoped() -> TestFixture {
        let testFixture = TestFixture {};
        testFixture.setUp();
        return testFixture;
    }

    fn setUp(&self) {
        createFile();
    }

    fn tearDown(&self) {
        removeFile();
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        self.tearDown()
    }
}