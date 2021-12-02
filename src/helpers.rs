use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn read_inputs_file(day: u8) -> Vec<String> {
    let suffix: &str = ".txt";
    let mut file_name: String = day.to_string().to_owned();
    file_name.push_str(suffix);

    let inputs_file_path = Path::new("src/inputs").join(&file_name);
    let file = match File::open(&inputs_file_path) {
        Err(_) => panic!("Failed to open inputs file: {}", inputs_file_path.display()),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap())
        .collect()
}
