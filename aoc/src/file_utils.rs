use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_lines(file_path: &str) -> Vec<String> {
    let f = File::open(file_path).expect(format!("Failed to open file: {}", file_path).as_str());
    io::BufReader::new(f).lines().map(|l| l.unwrap()).collect()
}
