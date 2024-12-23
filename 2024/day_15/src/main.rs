use std::time::Instant;

fn main() {
    let mut f = std::env::args();
    f.next();
    let f = f.next().unwrap();
    let input = std::fs::read_to_string(f).unwrap();

    let (fish_pos, mut warehouse, mov_list) = parse_input(&input);

    let t = Instant::now();
    println!(
        "task 01: {:?} in {} \u{B5}s",
        task_01(fish_pos, &mut warehouse, &mov_list),
        t.elapsed().as_micros()
    );
}

fn task_01(
    mut pos: (usize, usize),
    warehouse: &mut Vec<Vec<char>>,
    mov_list: &Vec<FishMovement>,
) -> u64 {
    for movement in mov_list.iter() {
        let mut box_queue = vec![(pos.0, pos.1)];
        match movement {
            FishMovement::Top => {
                let hit_blank = loop {
                    let (i, j) = box_queue[box_queue.len() - 1];

                    match warehouse[j - 1][i] {
                        '#' => break None,
                        '.' => break Some((i, j - 1)),
                        'O' => box_queue.push((i, j - 1)),
                        _ => panic!("unchecked char in warehouse"),
                    }
                };

                if let Some((mut i, mut j)) = hit_blank {
                    for (x, y) in box_queue.iter().rev() {
                        warehouse[j][i] = warehouse[*y][*x];
                        i = *x;
                        j = *y;
                    }
                    warehouse[pos.1][pos.0] = '.';
                    pos = (pos.0, pos.1 - 1);
                }
            }
            FishMovement::Right => {
                let hit_blank = loop {
                    let (i, j) = box_queue[box_queue.len() - 1];

                    match warehouse[j][i + 1] {
                        '#' => break None,
                        '.' => break Some((i + 1, j)),
                        'O' => box_queue.push((i + 1, j)),
                        _ => panic!("unchecked char in warehouse"),
                    }
                };

                if let Some((mut i, mut j)) = hit_blank {
                    for (x, y) in box_queue.iter().rev() {
                        warehouse[j][i] = warehouse[*y][*x];
                        i = *x;
                        j = *y;
                    }
                    warehouse[pos.1][pos.0] = '.';
                    pos = (pos.0 + 1, pos.1);
                }
            }
            FishMovement::Bot => {
                let hit_blank = loop {
                    let (i, j) = box_queue[box_queue.len() - 1];

                    match warehouse[j + 1][i] {
                        '#' => break None,
                        '.' => break Some((i, j + 1)),
                        'O' => box_queue.push((i, j + 1)),
                        _ => panic!("unchecked char in warehouse"),
                    }
                };

                if let Some((mut i, mut j)) = hit_blank {
                    for (x, y) in box_queue.iter().rev() {
                        warehouse[j][i] = warehouse[*y][*x];
                        i = *x;
                        j = *y;
                    }
                    warehouse[pos.1][pos.0] = '.';
                    pos = (pos.0, pos.1 + 1);
                }
            }
            FishMovement::Left => {
                let hit_blank = loop {
                    let (i, j) = box_queue[box_queue.len() - 1];

                    match warehouse[j][i - 1] {
                        '#' => break None,
                        '.' | '@' => break Some((i - 1, j)),
                        'O' => box_queue.push((i - 1, j)),
                        _ => panic!("unchecked char in warehouse"),
                    }
                };

                if let Some((mut i, mut j)) = hit_blank {
                    for (x, y) in box_queue.iter().rev() {
                        warehouse[j][i] = warehouse[*y][*x];
                        i = *x;
                        j = *y;
                    }
                    warehouse[pos.1][pos.0] = '.';
                    pos = (pos.0 - 1, pos.1);
                }
            }
        }
    }

    gps(warehouse)
}

fn gps(warehouse: &Vec<Vec<char>>) -> u64 {
    let mut bulk = 0;
    for (j, row) in warehouse.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == 'O' {
                bulk += j as u64 * 100 + i as u64;
            }
        }
    }
    bulk
}

pub fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<char>>, Vec<FishMovement>) {
    let mut in_movements = false;

    let mut map = Vec::new();

    let mut fish_pos = None;
    let mut movements = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            in_movements = true;
            continue;
        }
        if !in_movements {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    fish_pos = Some((x, y));
                }
            }
            map.push(line.chars().collect());
        } else {
            for b in line.bytes() {
                let m = match b {
                    b'^' => FishMovement::Top,
                    b'>' => FishMovement::Right,
                    b'v' => FishMovement::Bot,
                    b'<' => FishMovement::Left,
                    _ => panic!("impossible movement"),
                };
                movements.push(m);
            }
        }
    }

    (fish_pos.unwrap(), map, movements)
}

#[derive(Debug)]
pub enum FishMovement {
    Top,
    Right,
    Bot,
    Left,
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
