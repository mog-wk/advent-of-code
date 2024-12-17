use std::{collections::HashSet, time::Instant};

fn main() {
    let mut f = std::env::args();
    f.next();

    let f = f.next().unwrap_or("./day_10/src/input.txt".to_string());

    let input = std::fs::read_to_string(f).unwrap();

    let topological_map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .into_iter()
                .map(|b| b - 48)
                .collect::<Vec<u8>>()
        })
        .collect();

    for row in &topological_map {
        for level in row {
            print!("{}", level);
        }
        print!("\n")
    }

    let t = Instant::now();

    println!(
        "task_01: {} in {}_ns",
        task_01(&topological_map, false),
        t.elapsed().as_nanos()
    );

    let t = Instant::now();
    println!(
        "task_02: {} in {}_ns",
        task_01(&topological_map, true),
        t.elapsed().as_nanos()
    );
}

fn task_01(topological_map: &Vec<Vec<u8>>, value_rating: bool) -> u64 {
    let col_len = topological_map.len();
    // FIX: zero len map indexing
    let row_len = topological_map[0].len();
    let mut score_bulk: u64 = 0;

    for j in 0..col_len {
        for i in 0..row_len {
            let level = topological_map[j][i];
            let mut score = 0;
            if level == 0 {
                let mut found_trail = HashSet::new();
                get_trailheads(
                    topological_map,
                    row_len,
                    col_len,
                    None,
                    (i, j),
                    &mut found_trail,
                    &mut score,
                    value_rating,
                );

                score_bulk += score;

                //println!("\n{:?}", found_trail);
            }
        }
    }

    score_bulk
}

fn get_trailheads(
    topological_map: &Vec<Vec<u8>>,
    row_len: usize,
    col_len: usize,
    prev_value: Option<u8>,
    (i, j): (usize, usize),
    found_trail: &mut HashSet<(usize, usize)>,
    score: &mut u64,
    rating: bool,
) {
    let value = topological_map[j][i];

    if let Some(prev_value) = prev_value {
        if value != prev_value + 1 {
            return;
        }
    }

    //print!("{} {} ({}) > ", i, j, topological_map[j][i]);
    if value == 9 {
        if rating {
            //println!(">> added {} {}", i, j);
            *score += 1;
            return;
        } else {
            if found_trail.insert((i, j)) {
                //println!(">> added {} {}", i, j);
                *score += 1;
                return;
            }
        }
    }

    // scan all adjacent locations (top, right, bottom, left
    if j.checked_sub(1).is_some() {
        get_trailheads(
            topological_map,
            row_len,
            col_len,
            Some(value),
            (i, j - 1),
            found_trail,
            score,
            rating,
        );
    }
    if i < row_len - 1 {
        get_trailheads(
            topological_map,
            row_len,
            col_len,
            Some(value),
            (i + 1, j),
            found_trail,
            score,
            rating,
        );
    }

    if j < col_len - 1 {
        get_trailheads(
            topological_map,
            row_len,
            col_len,
            Some(value),
            (i, j + 1),
            found_trail,
            score,
            rating,
        );
    }
    if i.checked_sub(1).is_some() {
        get_trailheads(
            topological_map,
            row_len,
            col_len,
            Some(value),
            (i - 1, j),
            found_trail,
            score,
            rating,
        );
    }
}
