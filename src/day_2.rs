use crate::helpers;

struct Command {
    amount: u32,
    instruction: String,
}

impl Command {
    fn new(line: String) -> Self {
        let splits: Vec<&str> = line.split(' ').collect();

        Command {
            amount: splits[1].parse::<u32>().unwrap(),
            instruction: String::from(splits[0]),
        }
    }
}

struct Position {
    depth: u32,
    distance: u32,
}

pub fn solve_part(part: u8) -> u32 {
    let commands = get_commands();
    let position: Position;

    if part == 1 {
        position = get_part_1_position(commands);
    } else if part == 2 {
        position = get_part_2_position(commands);
    } else {
        panic!("Invalid part number");
    }

    position.distance * position.depth
}

fn get_part_1_position(commands: Vec<Command>) -> Position {
    let mut position = Position {
        depth: 0,
        distance: 0,
    };

    for command in commands {
        if command.instruction == "forward" {
            position.distance += command.amount;
        } else if command.instruction == "up" {
            position.depth -= command.amount;
        } else if command.instruction == "down" {
            position.depth += command.amount;
        } else {
            panic!("Unhandled instruction");
        }
    }

    position
}

fn get_part_2_position(commands: Vec<Command>) -> Position {
    let mut aim: u32 = 0;
    let mut position = Position {
        depth: 0,
        distance: 0,
    };

    for command in commands {
        if command.instruction == "forward" {
            position.distance += command.amount;
            position.depth += aim * command.amount;
        } else if command.instruction == "up" {
            aim -= command.amount;
        } else if command.instruction == "down" {
            aim += command.amount;
        } else {
            panic!("Unhandled instruction");
        }
    }

    position
}

fn get_commands() -> Vec<Command> {
    helpers::read_inputs_file(2)
        .into_iter()
        .map(|input| Command::new(input))
        .collect()
}
