use crate::helpers;

pub fn solve_part(part: u8) -> u32 {
    let inputs = get_inputs();

    let min_position: u16 = *inputs.iter().min().unwrap();
    let max_position: u16 = *inputs.iter().max().unwrap();

    let mut best_fuel_usage: u32 = u32::MAX;

    for i in min_position..max_position + 1 {
        let fuel_usage = get_fuel_usage(&inputs, i, part);

        if fuel_usage < best_fuel_usage {
            best_fuel_usage = fuel_usage;
        }
    }

    best_fuel_usage
}

fn get_fuel_usage(positions: &Vec<u16>, target_position: u16, part: u8) -> u32 {
    let mut fuel_used: u32 = 0;

    for position in positions {
        let steps_needed = if position > &target_position {
            (position - target_position) as u32
        } else {
            (target_position - position) as u32
        };

        if part == 1 {
            fuel_used += steps_needed;
        } else if part == 2 {
            fuel_used += (u32::pow(steps_needed, 2) + steps_needed) / 2;
        } else {
            panic!("Invalid part number");
        }
    }

    fuel_used
}

fn get_inputs() -> Vec<u16> {
    helpers::read_inputs_file(7)[0]
        .split(",")
        .map(|input| {
            match input.parse::<u16>() {
                Ok(value) => value,
                Err(_) => panic!("Failed to parse input value"),
            }
        })
        .collect()
}
