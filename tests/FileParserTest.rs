#![allow(non_snake_case)]

#[path = "../src/FileParser.rs"]
mod FileParser;

// give a vec of valid u8s

// for now making FileParse as a helper/util class

#[test]
fn parsesValidStructuredFile(){
    setupFile();

    // let fileParser = FileParser::from(path);
    // fileParser.parse();
    // let string = fileParser.getSudokuVector();

    let string = FileParser::parseFile(path);

    teardownFile();
}

fn setupFile(){

}