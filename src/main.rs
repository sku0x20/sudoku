#![allow(non_snake_case)]

use std::{env, fs};
use std::fmt::{Debug, format};
use std::io::Result;
use std::path::Path;

#[allow(dead_code)]
fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    runApp(args)
}

pub fn runApp(args: Vec<String>) -> Result<()> {
    let mut iterator = args.iter();
    let mut filePathString = "";
    while let Some(nextValue) = iterator.next() {
        if nextValue == "--solve" {
            filePathString = iterator.next().expect("--solve should be followed by filePath");
        }
    };

    let filePath = Path::new(filePathString);

    println!("{}", filePath.file_stem().unwrap().to_str().unwrap());

    let solvedFilename = format!("{}_solved.csv", filePath.file_stem().unwrap().to_str().unwrap());
    let solvedPathBuffer = filePath.with_file_name(solvedFilename);

    fs::write( solvedPathBuffer,"test")?;

    Ok(())
}
