use crate::helpers;

#[derive(Clone)]
struct Chunk {
    chars: Vec<char>,
}

#[derive(Clone)]
struct Line {
    chunks: Vec<Chunk>,
}

impl Line {
    fn new(raw: String) -> Self {
        let mut line = Line {
            chunks: Vec::new(),
        };
        let mut chunk = Chunk {
            chars: Vec::new(),
        };

        let open_chars = ['(', '[', '{', '<'];
        let close_chars = [')', ']', '}', '>'];

        let mut chars = raw.chars();
        let mut open_close_counter: u8 = 0;

        // loop over all characters, building chunks and pushing them onto the line
        loop {
            let c = chars.next();

            if !c.is_some() {
                // we've hit the end, push the current chunk
                line.chunks.push(chunk.clone());
                break;
            }

            let char_value = c.unwrap();

            if open_chars.contains(&char_value) {
                open_close_counter += 1;
            } else if close_chars.contains(&char_value) {
                open_close_counter -= 1;
            } else {
                panic!("Invalid character detected");
            }

            chunk.chars.push(char_value);

            if open_close_counter == 0 {
                // all opens have been closed, push the chunk
                line.chunks.push(chunk.clone());
                chunk.chars.clear();
            }
        }

        line
    }
}

fn get_inputs() -> Vec<Line> {
    helpers::read_inputs_file(10)
        .into_iter()
        .map(|line| Line::new(line))
        .collect()
}

pub fn solve_part(part: u8) -> u64 {
    let lines = get_inputs();
    let open_chars = ['(', '[', '{', '<'];
    let close_chars = [')', ']', '}', '>'];

    let mut completion_scores: Vec<u64> = Vec::new();
    let mut total: u64 = 0;

    if part == 0 || part > 2 {
        panic!("Invalid part number");
    }

    for i in 0..lines.len() {
        let line = lines[i].clone();

        'chunk: for chunk in line.chunks {
            let mut opening_chars: Vec<char> = Vec::new();

            for current_char in chunk.chars {
                if open_chars.contains(&current_char) {
                    opening_chars.push(current_char);
                } else {
                    // we have a closing char, get the last open we saw and compare
                    let current_opening = opening_chars.pop()
                        .unwrap();
                    let open_pos = open_chars.iter()
                        .position(|&x| x == current_opening)
                        .unwrap();
                    let close_pos = close_chars.iter()
                        .position(|&x| x == current_char)
                        .unwrap();

                    if open_pos != close_pos {
                        total += match current_char {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Invalid closing char detected"),
                        };

                        break 'chunk;
                    }
                }
            }

            if part == 2 {
                // complete the line and push its score
                let mut completion_score = 0;

                while opening_chars.len() > 0 {
                    let open_char = opening_chars.pop()
                        .unwrap();
                    let i = open_chars.iter()
                        .position(|&x| x == open_char)
                        .unwrap();
                    let close_char = close_chars[i as usize];

                    completion_score *= 5;
                    completion_score += match close_char {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Invalid closing char detected"),
                    };
                }

                if completion_score > 0 {
                    completion_scores.push(completion_score);
                }
            }
        }
    }

    if part == 1 {
        return total;
    }

    // sort the vector, then return the floor of len / 2 to grab the median
    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}
