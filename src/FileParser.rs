use std::fs;

pub fn parse(filePath: &str) -> Vec<u8> {
    let mut puzzle: Vec<u8> = Vec::with_capacity(81);
    let contents = fs::read_to_string(filePath).expect("unable to read file");
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let mut spit = line.split(',');
        while let Some(value) = spit.next() {
            puzzle.push(value.trim().parse::<u8>().unwrap());
        }
    };
    return puzzle;
}