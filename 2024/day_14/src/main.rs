use std::time::Instant;

pub type RobotList = Vec<(i32, i32, i32, i32)>;

fn main() {
    let mut f = std::env::args();
    f.next();
    let f = f.next().unwrap();
    let input = std::fs::read_to_string(f).unwrap();

    let mut robots = parse_input(&input);

    let t = Instant::now();
    println!(
        "task_01: {:?} in {} \u{B5}s",
        task_01(&mut robots, 100, 101, 103),
        t.elapsed().as_micros()
    );
}

pub fn task_01(robots: &mut RobotList, secs: u32, width: i32, height: i32) -> i32 {
    for _ in 0..secs {
        for (x, y, vx, vy) in robots.iter_mut() {
            let del_x = *x + *vx;

            *x = if del_x < 0 {
                width + del_x
            } else if del_x >= width {
                del_x % width
            } else {
                del_x
            };

            let del_y = *y + *vy;

            *y = if del_y < 0 {
                height + del_y
            } else if del_y >= height {
                del_y % height
            } else {
                del_y
            };
        }
    }

    let mut quads = [0; 4];

    let half_width = width / 2;
    let half_height = height / 2;

    for (x, y, _, _) in robots {
        if *x == half_width || *y == half_height {
            continue;
        }

        if *x < half_width {
            if *y < half_height {
                quads[0] += 1;
            } else {
                quads[2] += 1;
            }
        } else {
            if *y < half_height {
                quads[1] += 1;
            } else {
                quads[3] += 1;
            }
        }
    }

    quads.iter().fold(1, |acc, v| acc * v)
}

pub fn parse_input(input: &str) -> RobotList {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(' ').unwrap();

            let (x, y) = pos.split_once(',').unwrap();

            let x = &x[2..];

            let (vx, vy) = vel.split_once(',').unwrap();

            let vx = vx.split_once('=').unwrap().1;

            (
                x.parse().unwrap(),
                y.parse().unwrap(),
                vx.parse().unwrap(),
                vy.parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
