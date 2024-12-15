use std::collections::{HashMap, HashSet};

fn main() {
    let mut f = std::env::args();
    f.next();
    let f = f.next().unwrap_or("./day_08/src/input.txt".to_string());
    let input = std::fs::read_to_string(f).unwrap();

    let city_map: Vec<Vec<u8>> = input
        .lines()
        .into_iter()
        .map(|ln| ln.bytes().collect())
        .collect();
    drop(input);

    let mut antenna_map: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let row_len = city_map[0].len();
    let col_len = city_map.len();

    print!(" ");
    for i in 0..row_len {
        if i < 10 {
            print!("   {i}");
        } else {
            print!("  {i}");
        }
    }
    print!("\n");

    for j in 0..col_len {
        print!("{j:02} ");
        for i in 0..row_len {
            let b = city_map[j][i];
            print!("{:?} ", b as char);

            if b != b'.' && b != b'#' {
                antenna_map
                    .entry(b)
                    .and_modify(|pos_vec| pos_vec.push((i as i32, j as i32)))
                    .or_insert(vec![(i as i32, j as i32)]);
            }
        }
        print!("\n");
    }

    println!("{:?}", antenna_map);
    let row_len = row_len as i32;
    let col_len = col_len as i32;

    //println!("task 01: {}", task_01(&antenna_map, row_len, col_len));
    println!("task 02: {}", task_02(&antenna_map, row_len, col_len));
}
// anti nodes in all nodes of the line
fn task_02(antenna_map: &HashMap<u8, Vec<(i32, i32)>>, row_len: i32, col_len: i32) -> usize {
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    for (freq, antennas) in antenna_map {
        println!("checking: {}", *freq as char);

        for (x, anchor) in antennas.iter().enumerate() {
            for y in x..antennas.len() {
                let antenna = antennas[y];
                if *anchor == antenna {
                    continue;
                }
                println!("== Duo {:?} {:?}", anchor, antenna);

                // calc possible anti_nodes
                let del_x = (anchor.0 - antenna.0).abs();
                let del_y = (anchor.1 - antenna.1).abs();

                let (mut x1, mut y1, mut x2, mut y2) = (anchor.0, anchor.1, antenna.0, antenna.1);

                let mut c1 = 0;
                let mut c2 = 0;

                while check_map_limits(x1, y1, row_len, col_len)
                    || check_map_limits(x2, y2, row_len, col_len)
                {
                    println!(" {:?} {:?}", (x1, y1), (x2, y2));
                    if check_map_limits(x1, y1, row_len, col_len) {
                        println!("> added {:?}", (x1, y1));
                        anti_nodes.insert((x1, y1));
                        c1 += 1;
                    }
                    if check_map_limits(x2, y2, row_len, col_len) {
                        println!("> added {:?}", (x2, y2));
                        anti_nodes.insert((x2, y2));
                        c2 += 1;
                    }
                    (x1, y1, x2, y2) = if anchor.0 < antenna.0 {
                        // \
                        (
                            anchor.0 - del_x * c1,
                            anchor.1 - del_y * c1,
                            antenna.0 + del_x * c2,
                            antenna.1 + del_y * c2,
                        )
                    } else {
                        // /
                        (
                            anchor.0 + del_x * c1,
                            anchor.1 - del_y * c1,
                            antenna.0 - del_x * c2,
                            antenna.1 + del_y * c2,
                        )
                    };
                }
            }
        }
    }

    //check_anti_nodes(&anti_nodes, &city_map);
    anti_nodes.len()
}
fn task_01(antenna_map: &HashMap<u8, Vec<(i32, i32)>>, row_len: i32, col_len: i32) -> usize {
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    for (freq, antennas) in antenna_map {
        println!("checking: {}", *freq as char);

        for (x, anchor) in antennas.iter().enumerate() {
            for y in x..antennas.len() {
                let antenna = antennas[y];
                if *anchor == antenna {
                    continue;
                }
                println!("== Duo {:?} {:?}", anchor, antenna);

                // calc possible anti_nodes
                let del_x = (anchor.0 - antenna.0).abs();
                let del_y = (anchor.1 - antenna.1).abs();

                let (x1, y1, x2, y2) = if anchor.0 < antenna.0 {
                    // \
                    (
                        anchor.0 - del_x,
                        anchor.1 - del_y,
                        antenna.0 + del_x,
                        antenna.1 + del_y,
                    )
                } else {
                    // /
                    (
                        anchor.0 + del_x,
                        anchor.1 - del_y,
                        antenna.0 - del_x,
                        antenna.1 + del_y,
                    )
                };

                println!(" {:?} {:?}", (x1, y1), (x2, y2));
                if check_map_limits(x1, y1, row_len, col_len) {
                    println!("> added {:?}", (x1, y1));
                    anti_nodes.insert((x1, y1));
                }
                if check_map_limits(x2, y2, row_len, col_len) {
                    println!("> added {:?}", (x2, y2));
                    anti_nodes.insert((x2, y2));
                }
            }
        }
    }

    //check_anti_nodes(&anti_nodes, &city_map);
    anti_nodes.len()
}

// returns true if (x, y) is inside the map
fn check_map_limits(x: i32, y: i32, row_len: i32, col_len: i32) -> bool {
    x >= 0 && x < row_len && y >= 0 && y < col_len
}

fn check_anti_nodes(attempt: &HashSet<(i32, i32)>, map: &Vec<Vec<u8>>) {
    let mut aws_pos = HashSet::new();
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            let c = map[j][i];
            if c == b'#' {
                aws_pos.insert((i as i32, j as i32));
            }
        }
    }

    println!("attempt: {:?}", attempt);
    println!("answer: {:?}", aws_pos);
    println!("attempt || answer {:?}", attempt.difference(&aws_pos));
    println!("answer || attempt {:?}", aws_pos.difference(&attempt));
}
