use crate::helpers;

struct Board {
    spaces: [[u8; 5]; 5]
}

impl Board {
    fn new(data: [u8; 25]) -> Self {
        let mut values: [[u8; 5]; 5] = [[0; 5]; 5];

        for i in 0..25 {
            let row = i / 5;
            let col = i % 5;

            values[row][col] = data[i];
        }

        Board {
            spaces: values,
        }
    }

    pub fn is_won(&self) -> bool {
        for row in self.spaces {
            let mut is_winning = true;

            for col in 0..5 {
                if row[col] != 255 {
                    is_winning = false;
                    break;
                }
            }

            if is_winning {
                return true;
            }
        }

        for col in 0..5 {
            let mut is_winning = true;

            for row in 0..5 {
                if self.spaces[row][col] != 255 {
                    is_winning = false;
                    break;
                }
            }

            if is_winning {
                return true;
            }
        }

        false
    }

    fn play(&mut self, call: u8) -> u16 {
        'row: for row in 0..5 {
            for col in 0..5 {
                if self.spaces[row][col] == call {
                    self.spaces[row][col] = 255;
                    break 'row;
                }
            }
        }

        if self.is_won() {
            return self.sum_of_remaining();
        }

        0
    }

    pub fn sum_of_remaining(&self) -> u16 {
        let mut sum: u16 = 0;

        for row in self.spaces {
            for col in 0..5 {
                if row[col] < 255 {
                    sum += row[col] as u16;
                }
            }
        }

        sum
    }
}

fn get_boards(inputs: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut i = 0;

    while i < inputs.len() - 4 {
        if inputs[i].len () > 0 {
            let mut pos: usize = 0;
            let mut values: [u8; 25] = [0; 25];

            for j in 0..5 {
                let splits: Vec<u8> = inputs[i + j].split(' ')
                    .map(|split| {
                        match split.parse::<u8>() {
                            Ok(val) => val,
                            Err(_) => 255,
                        }
                    })
                    .collect();

                for split in splits {
                    if split < 255 {
                        values[pos] = split;
                        pos += 1;
                    }
                }
            }

            boards.push(Board::new(values));
            i += 5;
        }

        i += 1;
    }

    boards
}

fn get_calls(inputs: &Vec<String>) -> Vec<u8> {
    let mut calls: Vec<u8> = Vec::new();

    let splits: Vec<u8> = inputs[0].split(',')
        .map(|split| split.parse::<u8>().unwrap())
        .collect();

    for call in splits {
        calls.push(call);
    }

    calls
}

fn get_inputs() -> Vec<String> {
    helpers::read_inputs_file(4)
        .into_iter()
        .collect()
}

pub fn solve_part(part: u8) -> u16 {
    let mut inputs = get_inputs();
    let calls: Vec<u8> = get_calls(&inputs);

    if part == 0 || part > 2 {
        panic!("Invalid part number");
    }

    // remove the first row of calls and the blank line that follows
    inputs.remove(0);
    inputs.remove(0);

    let mut boards: Vec<Board> = get_boards(&inputs);

    for call in calls {
        for i in 0..boards.len() {
            let result = boards[i].play(call);

            if result > 0 && part == 1 {
                return result * call as u16;
            }
        }

        if part == 2 {
            if boards.len() > 1 {
                boards.retain(|board| !board.is_won());
            } else if boards[0].is_won() {
                return boards[0].sum_of_remaining() * call as u16;
            }
        }
    }

    panic!("Out of calls");
}
