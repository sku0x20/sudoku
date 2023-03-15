use std::fs;

pub fn parse(filePath: &str) -> Vec<u8> {
    let fileContent = fs::read_to_string(filePath).expect("unable to read file");

    let mut parser = InternalParser::new();
    parser.parse(fileContent);
    return parser.puzzle;
}

struct InternalParser {
    puzzle: Vec<u8>,
}

impl InternalParser {
    fn new() -> InternalParser {
        InternalParser {
            puzzle: Vec::with_capacity(81),
        }
    }

    fn parse(&mut self, content: String) {
        content.lines()
            .for_each(|line| self.parseLine(line));
    }

    fn parseLine(&mut self, line: &str) {
        line.split(',')
            .for_each(|value| self.puzzle.push(self.parseValue(value)))
    }

    fn parseValue(&self, value: &str) -> u8 {
        value.trim().parse::<u8>().unwrap()
    }
}