// answer => 15343601

use std::fs::File;
use std::io::Read;

use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    File::open("assets/06.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let bench = Instant::now();

    let input = input.split('\n').collect::<Vec<&str>>();

    let mut light_grid: [[u8; 1000]; 1000] = [[0_u8; 1000]; 1000];

    let mut light_counter: u32 = 0;

    for line in input[..].into_iter() {
        let orders = line.split(' ').collect::<Vec<&str>>();

        match orders.len() {
            5 => {
                let motion: bool = if orders[1] == "off" { false } else { true };
                let min: Vec<usize> = orders[2]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect();
                let max: Vec<usize> = orders[4]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect();

                for i in min[0]..=max[0] {
                    for j in min[1]..=max[1] {
                        if motion {
                            light_grid[j][i] += 1_u8;
                            light_counter += 1;
                        } else {
                            if light_grid[j][i] != 0 {
                                light_grid[j][i] -= 1_u8;
                                light_counter -= 1;
                            }
                        }
                    }
                }
            }
            4 => {
                let min: Vec<usize> = orders[1]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect();
                let max: Vec<usize> = orders[3]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect();
                for i in min[0]..=max[0] {
                    for j in min[1]..=max[1] {
                        light_grid[j][i] += 2_u8;
                        light_counter += 2
                    }
                }
            }
            _ => (),
        }
    }
    println!("{} | {:.2?}", light_counter, bench.elapsed());
}
