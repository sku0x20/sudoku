#![allow(non_snake_case)]

use std::{env, fs};
use std::io::Result;
use std::path::Path;

#[allow(dead_code)]
fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    runApp(args)
}

pub fn runApp(args: Vec<String>) -> Result<()> {
    let inputFilePath = getInputFilePath(&args);
    let fileName = getFileNameWithoutExtension(inputFilePath);

    let solvedFilename = format!("{}_solved.csv", fileName);
    let solvedPathBuffer = inputFilePath.with_file_name(solvedFilename);

    fs::write( solvedPathBuffer,"test")?;

    Ok(())
}

fn getInputFilePath(args: &Vec<String>) -> &Path {
    let mut iterator = args.iter();
    let mut filePathString = "";
    while let Some(nextValue) = iterator.next() {
        if nextValue == "--solve" {
            filePathString = iterator.next().expect("--solve should be followed by filePath");
        }
    };
    let inputFilePath = Path::new(filePathString);
    inputFilePath
}

fn getFileNameWithoutExtension(inputFilePath: &Path) -> &str {
    inputFilePath.file_stem().unwrap().to_str().unwrap()
}
