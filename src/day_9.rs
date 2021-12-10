use crate::helpers;

struct Basin {
    point: [u8; 2],
    size: u8,
}

pub fn solve_part(part: u8) -> u32 {
    let map = get_inputs();
    let basins = get_basins(&map);

    let mut result: u32 = 0;

    match part {
        1 => {
            for basin in basins {
                result += get_node_value(&map, &basin.point) as u32 + 1;
            }
        },
        2 => {
            let mut a = Basin {
                point: [0, 0],
                size: 0,
            };
            let mut b = Basin {
                point: [0, 0],
                size: 0,
            };
            let mut c = Basin {
                point: [0, 0],
                size: 0,
            };

            for basin in basins {
                if basin.size > a.size {
                    c = b;
                    b = a;
                    a = basin;
                } else if basin.size > b.size {
                    c = b;
                    b = basin;
                } else if basin.size > c.size {
                    c = basin;
                }
            }

            result = a.size as u32 * b.size as u32 * c.size as u32;
        }
        _ => panic!("Invalid part number"),
    };

    result
}

fn get_basins(map: &Vec<Vec<u8>>) -> Vec<Basin> {
    let low_nodes = get_low_nodes(map);
    let mut basins: Vec<Basin> = Vec::new();

    for node in low_nodes {
        let mut basin = Basin {
            point: [node[0], node[1]],
            size: 1,
        };

        let mut adjacent_nodes: Vec<[u8; 2]> = get_adjacent_nodes(map, &basin.point);
        let mut counter: usize = 0;
        let mut current_node_value = get_node_value(map, &basin.point);

        adjacent_nodes.retain(|n| get_node_value(map, &n) < 9 && get_node_value(map, &n) > current_node_value);
        basin.size += adjacent_nodes.len() as u8;

        loop {
            current_node_value = get_node_value(map, &adjacent_nodes[counter]);
            let adjacent_to_current = get_adjacent_nodes(map, &adjacent_nodes[counter]);

            for adjacent in adjacent_to_current {
                let adjacent_value = get_node_value(map, &adjacent);

                if adjacent_value < 9 && adjacent_value >= current_node_value && !adjacent_nodes.contains(&adjacent) {
                    basin.size += 1;
                    adjacent_nodes.push(adjacent);
                }
            }

            counter += 1;
            if counter == adjacent_nodes.len() {
                break;
            }
        };

        basins.push(basin);
    }

    basins
}

fn get_adjacent_nodes(map: &Vec<Vec<u8>>, point: &[u8; 2]) -> Vec<[u8; 2]> {
    let mut nodes: Vec<[u8; 2]> = Vec::new();
    let row = point[0];
    let col = point[1];

    let map_cols = map[row as usize].len() as u8;
    let map_rows = map.len() as u8;

    if row > 0 {
        nodes.push([row - 1, col]);
    }

    if col < map_cols - 1 {
        nodes.push([row, col + 1]);
    }

    if row < map_rows - 1 {
        nodes.push([row + 1, col]);
    }

    if col > 0 {
        nodes.push([row, col - 1]);
    }

    nodes
}

fn get_inputs() -> Vec<Vec<u8>> {
    let mut inputs: Vec<Vec<u8>> = Vec::new();
    let lines: Vec<String> = helpers::read_inputs_file(9);

    for line in lines {
        let mut row: Vec<u8> = Vec::new();

        for i in 0..line.len() {
            row.push(line.chars()
                .nth(i)
                .unwrap() as u8 - 48);
        }

        inputs.push(row);
    }

    inputs
}

fn get_low_nodes(map: &Vec<Vec<u8>>) -> Vec<[u8; 2]> {
    let mut low_nodes: Vec<[u8; 2]> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let point: [u8; 2] = [row as u8, col as u8];
            let current: u8 = get_node_value(map, &point);
            let is_low_point = get_adjacent_nodes(map, &point).into_iter()
                .all(|point| get_node_value(map, &point) > current);

            if is_low_point {
                low_nodes.push([row as u8, col as u8]);
            }
        }
    }

    low_nodes
}

fn get_node_value(map: &Vec<Vec<u8>>, point: &[u8; 2]) -> u8 {
    map[point[0] as usize][point[1] as usize]
}
