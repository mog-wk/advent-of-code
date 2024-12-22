use std::{
    collections::{HashMap, HashSet},
    io::{stdout, BufWriter, Write},
    time::Instant,
};

fn main() {
    let mut f = std::env::args();
    f.next();

    let f = f.next().unwrap_or("./day_12/src/input.txt".to_string());

    let input = std::fs::read_to_string(f).unwrap();

    let terrain: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let t = Instant::now();
    //println!( "task_01: {} in {} \u{B5}s", task_01(&terrain), t.elapsed().as_micros());
    println!(
        "task_02: {} in {} \u{B5}s",
        task_02(&terrain),
        t.elapsed().as_micros()
    );
}

fn task_02(terrain: &Vec<Vec<char>>) -> u64 {
    let mut writer = BufWriter::new(stdout());

    let row_ln = terrain[0].len();
    let col_ln = terrain.len();

    let calc_area = |i: usize, j: usize| {
        // backtrack towards label
        let mut stack = vec![(i, j)];
        let mut visited = HashSet::new();

        let mut area = 0_u64;

        while stack.len() > 0 {
            let (vi, vj) = stack.pop().unwrap();
            if terrain[vj][vi] != terrain[j][i] {
                continue;
            }
            if !visited.insert((vi, vj)) {
                continue;
            }

            area += 1;

            // add from left of cur if avalilable
            if vi.checked_sub(1).is_some() {
                stack.push((vi - 1, vj));
            }
            // add from bottom of cur if avalilable
            if vj + 1 < col_ln {
                stack.push((vi, vj + 1));
            }

            // add from right of cur if avalilable
            if vi + 1 < row_ln {
                stack.push((vi + 1, vj));
            }

            // add from top of cur if avalilable
            if vj.checked_sub(1).is_some() {
                stack.push((vi, vj - 1));
            }
        }
        (area, visited)
    };

    let mut plots_visited = HashSet::new();

    let mut total_price = 0_u64;

    for j in 0..col_ln {
        for i in 0..row_ln {
            // already in plot map skip
            if plots_visited.contains(&(i, j)) {
                continue;
            }

            let (area, visited) = calc_area(i, j);

            // process visited into array of cols
            let mut shape: Vec<Vec<bool>> = Vec::with_capacity(20);

            let shape_x_len = visited
                .iter()
                .reduce(|acc, e| if acc.0 <= e.0 { e } else { acc })
                .unwrap()
                .0
                + 1;

            for (x, y) in visited {
                let (vi, vj) = (x - i, y - j);

                println!(">{:?}", (vi, vj));
                if shape.len() <= vj {
                    for _ in 0..=vj - j {
                        shape.push(vec![false; shape_x_len]);
                    }
                    shape[vj][vi] = true;
                } else {
                    shape[vj][vi] = true;
                }
                println!("{:?}", shape);
            }

            println!("{:?}", shape);

            println!("{}", terrain[j][i]);

            todo!();

            //write!( writer, "added > {}: {} * {} = {}\n", terrain[j][i], area, area,).unwrap();
            plots_visited.extend(visited);
            total_price += area;
        }
    }

    writer.flush().unwrap();

    total_price
}
fn task_01(terrain: &Vec<Vec<char>>) -> u64 {
    let mut writer = BufWriter::new(stdout());

    let row_ln = terrain[0].len();
    let col_ln = terrain.len();

    let calc_area_and_perimeter = |i: usize, j: usize| {
        // backtrack towards label
        let mut stack = vec![(i, j)];
        let mut visited = HashSet::new();

        let mut area = 0_u64;
        let mut perimeter = 0_u64;

        while stack.len() > 0 {
            let (vi, vj) = stack.pop().unwrap();
            if terrain[vj][vi] != terrain[j][i] {
                continue;
            }
            if !visited.insert((vi, vj)) {
                continue;
            }

            let temp_perimeter = {
                let mut temp_perimeter: i32 = 4;
                // check adjacent nodes

                if vi.checked_sub(1).is_some() {
                    if visited.contains(&(vi - 1, vj)) {
                        temp_perimeter -= 2;
                    }
                }
                if vj + 1 < col_ln {
                    if visited.contains(&(vi, vj + 1)) {
                        temp_perimeter -= 2;
                    }
                }
                if vi + 1 < row_ln {
                    if visited.contains(&(vi + 1, vj)) {
                        temp_perimeter -= 2;
                    }
                }
                if vj.checked_sub(1).is_some() {
                    if visited.contains(&(vi, vj - 1)) {
                        temp_perimeter -= 2;
                    }
                }

                temp_perimeter
            };
            area += 1;

            if temp_perimeter > 0 {
                perimeter += temp_perimeter as u64;
            } else {
                perimeter -= temp_perimeter.abs() as u64;
            }

            // add from left of cur if avalilable
            if vi.checked_sub(1).is_some() {
                stack.push((vi - 1, vj));
            }
            // add from bottom of cur if avalilable
            if vj + 1 < col_ln {
                stack.push((vi, vj + 1));
            }

            // add from right of cur if avalilable
            if vi + 1 < row_ln {
                stack.push((vi + 1, vj));
            }

            // add from top of cur if avalilable
            if vj.checked_sub(1).is_some() {
                stack.push((vi, vj - 1));
            }
        }
        (area, perimeter, visited)
    };

    let mut plots_visited = HashSet::new();

    let mut total_price = 0_u64;

    for j in 0..col_ln {
        for i in 0..row_ln {
            // already in plot map skip
            if plots_visited.contains(&(i, j)) {
                continue;
            }

            let (area, perimeter, visited) = calc_area_and_perimeter(i, j);

            write!(
                writer,
                "added > {}: {} * {} = {}\n",
                terrain[j][i],
                area,
                perimeter,
                area * perimeter
            )
            .unwrap();
            plots_visited.extend(visited);
            total_price += area * perimeter;
        }
    }

    writer.flush().unwrap();

    total_price
}
