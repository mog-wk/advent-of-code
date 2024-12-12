use std::{collections::HashSet, usize};

fn main() {
    let mut f = std::env::args();
    f.next();
    let f = f.next();
    let input = match f {
        Some(f) => std::fs::read_to_string(&f).unwrap(),
        //None => std::fs::read_to_string("./day_06/src/input.txt").unwrap(),
        None => std::fs::read_to_string("./input.txt").unwrap(),
    };

    //println!("task 01: {}", task_01(&input));
    println!("task 02: {}", task_02(&input));
}

// TODO: refactor into recursion
fn task_02(input: &str) -> u32 {
    // guard position None means guard is off the board
    let mut guard_pos = None;
    // 0 = top; 1 = right; 2 = bot; 3 = left
    let mut guard_dir: u8 = 0;
    let map: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if let Some(j) = line.find('^') {
                guard_pos = Some((j, i))
            };
            line.bytes().collect()
        })
        .collect();

    let mut patrol_map: HashSet<(usize, usize)> = HashSet::new();

    let row_ln = map.len();
    let col_ln = map[0].len();

    println!("start_pos: {:?}", guard_pos.unwrap(),);
    let mut obstructions: u32 = 0;

    // advence patrol loop
    while let Some(pos) = guard_pos {
        let (i, j) = match guard_dir {
            0 => (Some(pos.0), pos.1.checked_sub(1)),
            1 => (Some(pos.0 + 1), Some(pos.1)),
            2 => (Some(pos.0), Some(pos.1 + 1)),
            3 => (pos.0.checked_sub(1), Some(pos.1)),
            _ => panic!("impossible dir"),
        };
        patrol_map.insert(pos);

        // guard is out of the map
        if i.is_none() || j.is_none() || i >= Some(col_ln) || j >= Some(row_ln) {
            println!("guard exit map ");
            guard_pos = None;
        } else {
            // add pos to map

            // set direction and update position
            let (i, j) = (i.unwrap(), j.unwrap());
            let next = map[j][i];

            println!(
                "next {:?} | current_pos: {:?} | current_dir: {}",
                next as char, pos, guard_dir,
            );
            // simulate obstruction
            let mut paradox_dir = (guard_dir + 1) % 4;

            let (para_i, para_j) = match paradox_dir {
                0 => (Some(pos.0), pos.1.checked_sub(1)),
                1 => (Some(pos.0 + 1), Some(pos.1)),
                2 => (Some(pos.0), Some(pos.1 + 1)),
                3 => (pos.0.checked_sub(1), Some(pos.1)),
                _ => panic!("impossible dir"),
            };

            if !(para_i.is_none()
                || para_j.is_none()
                || para_i >= Some(col_ln)
                || para_j >= Some(row_ln))
            {
                let (para_i, para_j) = (para_i.unwrap(), para_j.unwrap());

                let mut paradox_guard_pos = Some((para_i, para_j));

                // start paradox loop

                let mut paradox_patrol = HashSet::new();

                while let Some(paradox_pos) = paradox_guard_pos {
                    let (para_i, para_j) = match paradox_dir {
                        0 => (Some(paradox_pos.0), paradox_pos.1.checked_sub(1)),
                        1 => (Some(paradox_pos.0 + 1), Some(paradox_pos.1)),
                        2 => (Some(paradox_pos.0), Some(paradox_pos.1 + 1)),
                        3 => (paradox_pos.0.checked_sub(1), Some(paradox_pos.1)),
                        _ => panic!("impossible dir"),
                    };

                    if paradox_patrol.contains(&(paradox_pos, paradox_dir)) {
                        // in loop add valid obstruction and break
                        println!(
                            "paradox_obstruction in start: {:?}, end: {:?}; map: {:?}",
                            pos, paradox_pos, paradox_patrol
                        );
                        obstructions += 1;

                        break;
                    }

                    // guard is out of the map
                    if para_i.is_none()
                        || para_j.is_none()
                        || para_i >= Some(col_ln)
                        || para_j >= Some(row_ln)
                    {
                        println!("> guard exit map as paradox");
                        paradox_guard_pos = None;
                    } else {
                        // calculate next step
                        let (para_i, para_j) = (para_i.unwrap(), para_j.unwrap());
                        let next = map[para_j][para_i];
                        println!(
                            "> next {:?} | paradox_pos: {:?} | current_dir: {}",
                            next as char, paradox_pos, paradox_dir
                        );
                        if next == b'#' || (para_i == i && para_j == j) {
                            paradox_dir = (paradox_dir + 1) % 4;
                        } else {
                            paradox_patrol.insert((paradox_pos, paradox_dir));
                            paradox_guard_pos = Some((para_i, para_j));
                        }
                    }
                }
            }

            if next == b'#' {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                guard_pos = Some((i, j));
            }
        }
    }

    obstructions
}

fn paradox_sim(pos: (usize, usize), dir: u8, map: HashSet<(usize, usize)>) {}

fn task_01(input: &str) -> usize {
    // guard position None means guard is off the board
    let mut guard_pos = None;
    // 0 = top; 1 = right; 2 = bot; 3 = left
    let mut guard_dir: u8 = 0;
    let map: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if let Some(j) = line.find('^') {
                guard_pos = Some((j, i))
            };
            line.bytes().collect()
        })
        .collect();

    let mut patrol_map: HashSet<(usize, usize)> = HashSet::new();

    let row_ln = map.len();
    let col_ln = map[0].len();

    println!("start_pos: {:?}", guard_pos.unwrap(),);

    // advence patrol loop
    while let Some(pos) = guard_pos {
        patrol_map.insert(pos);
        let (i, j) = match guard_dir {
            0 => (Some(pos.0), pos.1.checked_sub(1)),
            1 => (Some(pos.0 + 1), Some(pos.1)),
            2 => (Some(pos.0), Some(pos.1 + 1)),
            3 => (pos.0.checked_sub(1), Some(pos.1)),
            _ => panic!("impossible dir"),
        };

        // guard is out of the map
        if i.is_none() || j.is_none() || i >= Some(col_ln) || j >= Some(row_ln) {
            println!("guard exit map ");
            guard_pos = None;
        } else {
            // set direction and update position
            let (i, j) = (i.unwrap(), j.unwrap());
            let next = map[j][i];

            println!(
                "next {:?} | current_pos: {:?} | current_dir: {}",
                next as char,
                guard_pos.unwrap(),
                guard_dir,
            );
            if next == b'#' {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                guard_pos = Some((i, j));
            }
        }
    }

    patrol_map.len()
}
