use crate::helpers;

struct Lanternfish {
    count: u64,
    timer: u8,
}

impl Lanternfish {
    fn new(count: u64) -> Self {
        Lanternfish {
            count,
            timer: 8,
        }
    }
}

pub fn solve_part(part: u8) -> u64 {
    let days_to_run: u16 = match part {
        1 => 80,
        2 => 256,
        _ => panic!("Invalid part number"),
    };

    let mut fish = get_inputs();

    for _ in 0..days_to_run {
        let mut new_fish: u64 = 0;

        for i in 0..fish.len() {
            match fish[i].timer {
                0 => {
                    fish[i].timer = 6;
                    new_fish += fish[i].count;
                },
                _ => fish[i].timer -= 1,
            }
        }

        if new_fish > 0 {
            fish.push(Lanternfish::new(new_fish));
        }
    }

    let mut result: u64 = 0;

    for f in fish {
        result += f.count as u64;
    }

    result
}

fn get_inputs() -> Vec<Lanternfish> {
    let mut counts: [u8; 7] = [0; 7];

    let timers: Vec<u8> = helpers::read_inputs_file(6)[0]
        .split(',')
        .map(|value| {
            match value.parse::<u8>() {
                Ok(timer) => timer,
                Err(_) => panic!("Failed to parse input value"),
            }
        })
        .collect();

    for timer in timers {
        counts[timer as usize] += 1;
    }

    let mut fish: Vec<Lanternfish> = Vec::new();

    for i in 0..counts.len() {
        fish.push(Lanternfish {
            count: counts[i] as u64,
            timer: i as u8,
        })
    }

    fish
}
