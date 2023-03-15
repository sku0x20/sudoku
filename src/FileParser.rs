use std::fs;

pub fn parse(filePath: &str) -> Vec<u8> {
    let mut puzzle: Vec<u8> = Vec::with_capacity(81);
    let contents = fs::read_to_string(filePath).expect("unable to read file");
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        parseLine(&mut puzzle, line);
    };
    return puzzle;
}

fn parseLine(puzzle: &mut Vec<u8>, line: &str) {
    line.split(',')
        .for_each(|value| puzzle.push(parseValue(value)))
}

fn parseValue(value: &str) -> u8 {
    value.trim().parse::<u8>().unwrap()
}