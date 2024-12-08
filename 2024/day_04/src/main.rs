fn main() {
    let input: Vec<Vec<u8>> = std::fs::read_to_string("./day_04/src/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    // task 01

    let patten = "XMAS".as_bytes();
    //println!("task 01: {}", task_01(&input, &patten));
    println!("task 02: {}", task_02(&input));
}

// amount o X-MAS
fn task_02(input: &Vec<Vec<u8>>) -> u32 {
    let input_ln = if input.len() > 3 {
        input.len() - 3
    } else {
        panic!("impossible input");
    };
    let line_ln = if input[0].len() > 3 {
        input[0].len() - 3
    } else {
        panic!("inpossible input line")
    };
    let mut x_mas_couter = 0;

    for i in 0..=input_ln {
        for j in 0..=line_ln {
            // continue case center has to be an 'A'
            if input[i + 1][j + 1] != b'A' {
                continue;
            }

            // case for top-left / bot-right
            if !((input[i][j] == b'M' && input[i + 2][j + 2] == b'S')
                || (input[i][j] == b'S' && input[i + 2][j + 2] == b'M'))
                || !((input[i + 2][j] == b'M' && input[i][j + 2] == b'S')
                    || (input[i + 2][j] == b'S' && input[i][j + 2] == b'M'))
            {
                continue;
            }
            x_mas_couter += 1;
        }
    }

    x_mas_couter
}

fn task_01(input: &Vec<Vec<u8>>, patten: &[u8]) -> u32 {
    let input_ln = input.len();
    let line_ln = input[0].len();

    let mut patten_counter = 0;

    for i in 0..input_ln {
        println!("line {}: {:?}", i, input[i]);
        for j in 0..line_ln {
            // stack (position to be compared to, column index, row index
            let mut stack = vec![(None::<Direction>, j, i)];
            //
            let mut k = 0;

            while let Some((dir, vj, vi)) = stack.pop() {
                print!("> ({}, {}) {} | ", j, i, input[i][j] as char);
                print!(
                    "({}, {}) {:?} <{}> {}: ",
                    vj, vi, dir, k, input[vi][vj] as char
                );
                for item in stack.iter() {
                    let (d, i, j) = (&item.0, item.1, item.2);
                    print!("({:?}: {}, {}) {:?}, ", d, i, j, input[i][j] as char);
                }
                print!("\n");
                if input[vi][vj] != patten[k] {
                    k = 1;
                    continue;
                }
                if k == patten.len() - 1 {
                    println!("-> {} in: {:?}, {} {}", patten_counter, dir, vj, vi);
                    patten_counter += 1;
                    k = 1;
                    continue;
                }

                k += 1;

                match dir {
                    Some(dir) => match dir {
                        Direction::RIGHT => {
                            if vj < line_ln - 1 {
                                stack.push((Some(dir), vj + 1, vi));
                            } else {
                                k = 1
                            }
                        }
                        Direction::LEFT => {
                            if vj > 0 {
                                stack.push((Some(dir), vj - 1, vi));
                            } else {
                                k = 1
                            }
                        }
                        Direction::TOP => {
                            if vi > 0 {
                                stack.push((Some(dir), vj, vi - 1));
                            } else {
                                k = 1
                            }
                        }
                        Direction::TOP_RIGHT => {
                            if vi > 0 && vj < line_ln - 1 {
                                stack.push((Some(dir), vj + 1, vi - 1));
                            } else {
                                k = 1
                            }
                        }
                        Direction::TOP_LEFT => {
                            if vi > 0 && vj > 0 {
                                stack.push((Some(dir), vj - 1, vi - 1));
                            } else {
                                k = 1
                            }
                        }
                        Direction::BOTTON => {
                            if vi < input_ln - 1 {
                                stack.push((Some(dir), vj, vi + 1));
                            } else {
                                k = 1
                            }
                        }
                        Direction::BOTTON_RIGHT => {
                            if vi < input_ln - 1 && vj < line_ln - 1 {
                                stack.push((Some(dir), vj + 1, vi + 1));
                            } else {
                                k = 1
                            }
                        }
                        Direction::BOTTON_LEFT => {
                            if vi < input_ln - 1 && vj > 0 {
                                stack.push((Some(dir), vj - 1, vi + 1));
                            } else {
                                k = 1
                            }
                        }
                    },
                    // if no direction is set, add all directions
                    None => {
                        // reset patten ptr
                        k = 1;
                        // add all neighbors to the stack in order
                        if vi < input_ln - 1 {
                            if vj < line_ln - 1 {
                                stack.push((Some(Direction::BOTTON_RIGHT), vj + 1, vi + 1));
                            }
                            stack.push((Some(Direction::BOTTON), vj, vi + 1));
                            if vj > 0 {
                                stack.push((Some(Direction::BOTTON_LEFT), vj - 1, vi + 1));
                            }
                        }

                        if vj < line_ln - 1 {
                            stack.push((Some(Direction::RIGHT), vj + 1, vi));
                        }
                        if vj > 0 {
                            stack.push((Some(Direction::LEFT), vj - 1, vi));
                        }

                        if vi > 0 {
                            if vj < line_ln - 1 {
                                stack.push((Some(Direction::TOP_RIGHT), vj + 1, vi - 1));
                            }
                            stack.push((Some(Direction::TOP), vj, vi - 1));
                            if vj > 0 {
                                stack.push((Some(Direction::TOP_LEFT), vj - 1, vi - 1));
                            }
                        }
                    }
                }
            }
        }
    }

    patten_counter
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Direction {
    RIGHT,
    LEFT,
    TOP,
    TOP_RIGHT,
    TOP_LEFT,
    BOTTON,
    BOTTON_RIGHT,
    BOTTON_LEFT,
}

// I missunderstood the first task and wrote a backtrack algorithm
// I'm not deleting it
#[allow(unused)]
fn back_tracking(input: &Vec<Vec<u8>>, patten: &[u8]) -> u32 {
    let input_ln = input.len();
    let line_ln = input[0].len();

    let mut patten_counter = 0;

    for i in 0..input_ln {
        println!("line: {:?}", input[i]);
        for j in 0..line_ln {
            // stack (position to be compared to, column index, row index
            let mut stack = vec![(0, j, i)];

            while let Some((k, vj, vi)) = stack.pop() {
                // if not in patten continue without add
                if input[vi][vj] != patten[k] {
                    continue;
                }
                print!("> ({}, {}) {} | ", j, i, input[i][j] as char);
                print!("({}, {}) {}: ", vj, vi, input[vi][vj] as char);
                for item in stack.iter() {
                    print!(
                        "({}, {}) {:?}, ",
                        item.0, item.1, input[item.0][item.1] as char
                    );
                }
                print!("\n");
                if k == patten.len() - 1 {
                    patten_counter += 1;
                    println!("added: {}", patten_counter);
                    continue;
                }

                // add all neighbors to the stack in order
                if vi < input_ln - 1 {
                    if vj < line_ln - 1 {
                        stack.push((k + 1, vj + 1, vi + 1));
                    }
                    stack.push((k + 1, vj, vi + 1));
                    if vj > 0 {
                        stack.push((k + 1, vj - 1, vi + 1));
                    }
                }

                if vj < line_ln - 1 {
                    stack.push((k + 1, vj + 1, vi));
                }
                if vj > 0 {
                    stack.push((k + 1, vj - 1, vi));
                }

                if vi > 0 {
                    if vj < line_ln - 1 {
                        stack.push((k + 1, vj + 1, vi - 1));
                    }
                    stack.push((k + 1, vj, vi - 1));
                    if vj > 0 {
                        stack.push((k + 1, vj - 1, vi - 1));
                    }
                }
            }
        }
    }

    patten_counter
}
