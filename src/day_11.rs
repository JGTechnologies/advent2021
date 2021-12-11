use crate::helpers;

fn get_inputs() -> Vec<Vec<u8>> {
    let mut inputs: Vec<Vec<u8>> = Vec::new();

    for line in helpers::read_inputs_file(11) {
        let mut values: Vec<u8> = Vec::new();
        let mut chars = line.chars();

        loop {
            let c = chars.next();

            if !c.is_some() {
                inputs.push(values);
                break;
            }

            let value = c.unwrap();

            values.push(value.to_digit(10).unwrap() as u8);
        }
    }

    inputs
}

pub fn solve_part(part: u8) -> u16 {
    let mut octopi = get_inputs();
    let mut total_flashes: u16 = 0;
    let mut steps_taken: u16 = 0;

    loop {
        // increase energy for each octopus
        for row in 0..octopi.len() {
            for col in 0..octopi[row].len() {
                octopi[row][col] += 1;
            }
        }

        // loop over all octopi, flashing whichever ones are ready until all flashes are done
        let mut flash_occurred = true;

        while flash_occurred {
            flash_occurred = false;

            for row in 0..octopi.len() {
                for col in 0..octopi[row].len() {
                    let octopus = octopi[row][col];

                    if octopus > 9 && octopus < 255 {
                        // mark octopi that flashed by setting them to 255
                        octopi[row][col] = 255;
                        flash_occurred = true;
                        total_flashes += 1;

                        if row > 0 {
                            // N
                            if octopi[row - 1][col] < 255 {
                                octopi[row - 1][col] += 1;
                            }

                            // NW
                            if col > 0 {
                                if octopi[row - 1][col - 1] < 255 {
                                    octopi[row - 1][col - 1] += 1;
                                }
                            }

                            // NE
                            if col < octopi[0].len() - 1 {
                                if octopi[row - 1][col + 1] < 255 {
                                    octopi[row - 1][col + 1] += 1;
                                }
                            }
                        }

                        if row < octopi.len() - 1 {
                            // S
                            if octopi[row + 1][col] < 255 {
                                octopi[row + 1][col] += 1;
                            }

                            // SW
                            if col > 0 {
                                if octopi[row + 1][col - 1] < 255 {
                                    octopi[row + 1][col - 1] += 1;
                                }
                            }

                            // SE
                            if col < octopi[0].len() - 1 {
                                if octopi[row + 1][col + 1] < 255 {
                                    octopi[row + 1][col + 1] += 1;
                                }
                            }
                        }

                        // W
                        if col > 0 {
                            if octopi[row][col - 1] < 255 {
                                octopi[row][col - 1] += 1;
                            }
                        }

                        // E
                        if col < octopi[0].len() - 1 {
                            if octopi[row][col + 1] < 255 {
                                octopi[row][col + 1] += 1;
                            }
                        }
                    }
                }
            }
        }

        let mut all_flashed = true;

        // reset all flashed octopi to 0 energy
        for row in 0..octopi.len() {
            for col in 0..octopi[row].len() {
                if octopi[row][col] == 255 {
                    octopi[row][col] = 0;
                } else {
                    all_flashed = false;
                }
            }
        }

        steps_taken += 1;

        match part {
            1 => {
                if steps_taken == 100 {
                    return total_flashes;
                }
            },
            2 => {
                if all_flashed {
                    return steps_taken;
                }
            },
            _ => panic!("Invalid part"),
        }
    }
}
