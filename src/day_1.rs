use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn solve_part(part: u8) -> u16 {
    let inputs: Vec<u16>;

    if part == 1 {
        inputs = get_raw_inputs();
    } else if part == 2 {
        inputs = get_part_2_inputs();
    } else {
        panic!("Invalid part number");
    }

    let mut counter: u16 = 0;
    let mut previous_input: u16 = u16::MAX;

    for input in inputs {
        if input > previous_input {
            counter += 1;
        }

        previous_input = input;
    }

    counter
}

fn get_part_2_inputs() -> Vec<u16> {
    let mut inputs: Vec<u16> = Vec::new();
    let raw_inputs = get_raw_inputs();

    for i in 0..raw_inputs.len() - 2 {
        let sum = raw_inputs[i] + raw_inputs[i + 1] + raw_inputs[i + 2];
        inputs.push(sum);
    }

    inputs
}

fn get_raw_inputs() -> Vec<u16> {
    let inputs_file_path = Path::new("src/inputs/1.txt");
    let file = match File::open(&inputs_file_path) {
        Err(_) => panic!("Failed to open inputs file"),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().parse::<u16>()
        .unwrap())
        .collect()
}
