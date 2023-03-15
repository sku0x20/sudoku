use std::fs;

pub fn parse(filePath: &str) -> Vec<u8> {
    let contents = fs::read_to_string(filePath).expect("unable to read file");

    let mut puzzle: Vec<u8> = Vec::with_capacity(81);
    parseLines(&mut puzzle, contents);
    return puzzle;
}

fn parseLines(mut puzzle: &mut Vec<u8>, contents: String) {
    contents.lines()
        .for_each(|line| parseLine(&mut puzzle, line));
}

fn parseLine(puzzle: &mut Vec<u8>, line: &str) {
    line.split(',')
        .for_each(|value| puzzle.push(parseValue(value)))
}

fn parseValue(value: &str) -> u8 {
    value.trim().parse::<u8>().unwrap()
}