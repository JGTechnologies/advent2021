use crate::helpers;

struct Note {
    output_values: [String; 4],
    signal_patterns: [String; 10],
}

impl Note {
    fn new(input: String) -> Self {
        let splits: Vec<&str> = input.split(" | ")
            .collect();
        let inputs: Vec<String> = splits[0].split(' ')
            .into_iter()
            .map(|value| String::from(value))
            .collect();
        let outputs: Vec<String> = splits[1].split(' ')
            .into_iter()
            .map(|value| String::from(value))
            .collect();

        let mut output_values: [String; 4] = Default::default();
        let mut signal_patterns: [String; 10] = Default::default();

        for i in 0..10 {
            signal_patterns[i] = inputs[i].to_string();
        }

        for i in 0..4 {
            output_values[i] = outputs[i].to_string();
        }

        Note {
            output_values,
            signal_patterns,
        }
    }
}

fn count_bits(value: &u8) -> u8 {
    let mut result: u8 = 0;
    let mut current_value = *value;

    while current_value > 0 {
        if current_value & 1 == 1 {
            result += 1;
        }

        current_value = current_value >> 1;
    }

    result
}

fn get_inputs() -> Vec<Note> {
    helpers::read_inputs_file(8)
        .into_iter()
        .map(|line| Note::new(line))
        .collect()
}

pub fn solve_part(part: u8) -> u32 {
    let mut counter: u32 = 0;
    let notes = get_inputs();

    for note in notes {
        let patterns = solve_patterns(&note);

        let thousands_mask: u8 = to_mask(&note.output_values[0]);
        let hundreds_mask: u8 = to_mask(&note.output_values[1]);
        let tens_mask: u8 = to_mask(&note.output_values[2]);
        let ones_mask: u8 = to_mask(&note.output_values[3]);

        let mut thousands: u8 = 0;
        let mut hundreds: u8 = 0;
        let mut tens: u8 = 0;
        let mut ones: u8 = 0;

        for i in 0..patterns.len() {
            if patterns[i] == thousands_mask {
                thousands = i as u8;
            }

            if patterns[i] == hundreds_mask {
                hundreds = i as u8;
            }

            if patterns[i] == tens_mask {
                tens = i as u8;
            }

            if patterns[i] == ones_mask {
                ones = i as u8;
            }
        }

        match part {
            1 => {
                let to_find: [u8; 4] = [1, 4, 7, 8];

                if to_find.contains(&thousands) {
                    counter += 1;
                }

                if to_find.contains(&hundreds) {
                    counter += 1;
                }

                if to_find.contains(&tens) {
                    counter += 1;
                }

                if to_find.contains(&ones) {
                    counter += 1;
                }
            },
            2 => counter += thousands as u32 * 1000 + hundreds as u32 * 100 + tens as u32 * 10 + ones as u32,
            _ => panic!("Invalid part number"),
        }
    }

    counter
}

fn solve_patterns(note: &Note) -> [u8; 10] {
    let mut pattern_masks: [u8; 10] = [0; 10];
    let mut solved_masks: [u8; 10] = [0; 10];

    for i in 0..10 {
        pattern_masks[i] = to_mask(&note.signal_patterns[i]);
    }

    // find the 4 free pattern masks
    for i in 0..pattern_masks.len() {
        let mask = pattern_masks[i];

        match count_bits(&mask) {
            2 => {
                solved_masks[1] = mask;
                pattern_masks[i] = 0;
            },
            3 => {
                solved_masks[7] = mask;
                pattern_masks[i] = 0;
            },
            4 => {
                solved_masks[4] = mask;
                pattern_masks[i] = 0;
            },
            7 => {
                solved_masks[8] = mask;
                pattern_masks[i] = 0;
            },
            _ => { /* happy compiler :D */ }
        }
    }

    // mask 3 is the only 5-length that shares both parts of mask 1
    // mask 6 is the only 6-length that does not share both parts of mask 1
    for i in 0..pattern_masks.len() {
        let mask = pattern_masks[i];

        if mask == 0 {
            continue;
        }

        let num_bits = count_bits(&mask);

        if num_bits == 5 {
            if mask & solved_masks[1] == solved_masks[1] {
                solved_masks[3] = mask;
                pattern_masks[i] = 0;
            }
        } else if num_bits == 6 {
            if mask & solved_masks[1] != solved_masks[1] {
                solved_masks[6] = mask;
                pattern_masks[i] = 0;
            }
        }
    }

    // mask 9 shares all parts of mask 3
    for i in 0..pattern_masks.len() {
        let mask = pattern_masks[i];

        if mask == 0 {
            continue;
        }

        if mask & solved_masks[3] == solved_masks[3] {
            solved_masks[9] = mask;
            pattern_masks[i] = 0;
            break;
        }
    }

    // 0 is the only 6-length remaining and 5 shares all its parts with 9
    for i in 0..pattern_masks.len() {
        let mask = pattern_masks[i];

        if mask == 0 {
            continue;
        }

        if count_bits(&mask) == 6 {
            solved_masks[0] = mask;
            pattern_masks[i] = 0;
        } else if mask & solved_masks[9] == mask {
            solved_masks[5] = mask;
            pattern_masks[i] = 0;
        }
    }

    // 2 is the only one left
    for i in 0..pattern_masks.len() {
        let mask = pattern_masks[i];

        if mask == 0 {
            continue;
        }

        solved_masks[2] = mask;
    }

    solved_masks
}

fn to_mask(value: &String) -> u8 {
    let mut result: u8 = 0;
    let chars: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    for i in 0..chars.len() {
        if value.contains(chars[i]) {
            result |= 1 << i
        }
    }

    result
}
