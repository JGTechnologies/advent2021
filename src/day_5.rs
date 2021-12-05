use crate::helpers;

struct VentLine {
    x_1: u16,
    y_1: u16,
    x_2: u16,
    y_2: u16,
}

impl VentLine {
    fn new(input: String) -> Self {
        let from_to: Vec<&str> = input.split(" -> ")
            .collect();
        let from: Vec<u16> = from_to[0].split(',')
            .map(|split| split.parse::<u16>()
            .unwrap())
            .collect();
        let to: Vec<u16> = from_to[1].split(',')
            .map(|split| split.parse::<u16>()
            .unwrap())
            .collect();

        VentLine {
            x_1: from[1],
            y_1: from[0],
            x_2: to[1],
            y_2: to[0],
        }
    }

    pub fn get_points(&self) -> Vec<[u16; 2]> {
        let mut current_x: u16 = self.x_1;
        let mut current_y: u16 = self.y_1;

        let mut points: Vec<[u16; 2]> = Vec::new();

        loop {
            points.push([current_x, current_y]);

            if current_x == self.x_2 && current_y == self.y_2 {
                return points;
            }

            if current_x < self.x_2 {
                current_x += 1;
            } else if current_x > self.x_2 {
                current_x -= 1;
            }

            if current_y < self.y_2 {
                current_y += 1;
            } else if current_y > self.y_2 {
                current_y -= 1;
            }
        }
    }
}

pub fn solve_part(part: u8) -> u16 {
    if part == 0 || part > 2 {
        panic!("Invalid part number");
    }

    let mut inputs = get_inputs();

    if part == 1 {
        inputs.retain(|input| input.x_1 == input.x_2 || input.y_1 == input.y_2);
    }

    let max_pos = get_max_pos(&inputs);
    let mut vents: Vec<Vec<u8>> = Vec::new();

    for _ in 0..max_pos + 1 {
        let mut x: Vec<u8> = Vec::new();

        for _ in 0..max_pos + 1 {
            x.push(0);
        }

        vents.push(x);
    }

    for input in inputs {
        for point in input.get_points() {
            let x = point[0] as usize;
            let y = point[1] as usize;

            vents[x][y] += 1;
        }
    }

    let mut counter: u16 = 0;
    for i in 0..max_pos + 1 {
        for j in 0..max_pos + 1 {
            let x = i as usize;
            let y = j as usize;

            if vents[x][y] > 1 {
                counter += 1;
            }
        }
    }

    counter
}

fn get_inputs() -> Vec<VentLine> {
    helpers::read_inputs_file(5)
        .into_iter()
        .map(|input| VentLine::new(input))
        .collect()
}

fn get_max_pos(vent_lines: &Vec<VentLine>) -> u16 {
    let mut largest_pos: u16 = 0;

    for vent_line in vent_lines {
        if vent_line.x_1 > largest_pos {
            largest_pos = vent_line.x_1;
        }

        if vent_line.y_1 > largest_pos {
            largest_pos = vent_line.y_1;
        }

        if vent_line.x_2 > largest_pos {
            largest_pos = vent_line.x_2;
        }

        if vent_line.y_2 > largest_pos {
            largest_pos = vent_line.y_2;
        }
    }

    largest_pos
}
