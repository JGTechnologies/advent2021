use crate::helpers;

pub fn solve_part(part: u8) -> u64 {
    let inputs = get_inputs();
    let most_common_bitmask = get_most_common_bitmask(&inputs);
    let flipped_bitmask = flip_all_bits(&most_common_bitmask);

    if part == 1 {
        most_common_bitmask as u64 * flipped_bitmask as u64
    } else if part == 2 {
        let o2 = find_first_input_meeting_mask(&inputs, &most_common_bitmask, 11, false);
        let co2_scrubber = find_first_input_meeting_mask(&inputs, &flipped_bitmask, 11, true);

        o2 * co2_scrubber
    } else {
        panic!("Invalid part number");
    }
}

fn find_first_input_meeting_mask(inputs: &Vec<u16>, mask: &u16, step: usize, invert_bitmask: bool) -> u64 {
    let mut next_inputs: Vec<u16> = Vec::new();
    let target_bit = mask & (1 << step) > 0;

    for input in inputs {
        let bit = input & (1 << step) > 0;

        if bit == target_bit {
            next_inputs.push(*input);
        }
    }

    let mut new_mask = get_most_common_bitmask(&next_inputs);

    if invert_bitmask {
        new_mask = flip_all_bits(&new_mask);
    }

    if next_inputs.len() == 1 {
        return next_inputs[0] as u64;
    }

    return find_first_input_meeting_mask(&next_inputs, &new_mask, step - 1, invert_bitmask);
}

fn flip_all_bits(bitmask: &u16) -> u16 {
    let mut result = !bitmask;

    // unset the 4 high bits since inputs are only 12 bits
    result &= 0b1111_1111_1111u16;

    result
}

fn get_inputs() -> Vec<u16> {
    helpers::read_inputs_file(3)
        .into_iter()
        .map(|input| u16::from_str_radix(&input, 2)
        .unwrap())
        .collect()
}

fn get_most_common_bitmask(inputs: &Vec<u16>) -> u16 {
    let mut bitmask: u16 = 0;

    for i in 0..12 {
        let mut counter: i16 = 0;

        for input in inputs {
            let is_zero = input & (1 << i) == 0;

            if is_zero {
                counter -= 1;
            } else {
                counter += 1;
            }
        }

        if counter >= 0 {
            bitmask |= 1 << i;
        }
    }

    bitmask
}
