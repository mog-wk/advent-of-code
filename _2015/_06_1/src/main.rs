// answer => 400410

use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    File::open("assets/06.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut input: Vec<&str> = input.split('\n').collect();

    // false means off true means on
    let mut light_grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    //println!("{:?}", &light_grid[0..10]);

    for line in input.into_iter() {
        let orders: Vec<&str> = line.split(' ').collect();
        println!("{:?}", orders);

        match orders.len() {
            // set
            5 => {
                //let (motion: bool, min: (u32, u32), max: (u32, u32));

                let motion = if orders[1] == "off" { false } else { true };
                let min = orders[2]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let max = orders[4]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                println!("{:?} {:?}", min, max);

                for i in min[0]..=max[0] {
                    for j in min[1]..=max[1] {
                        light_grid[j][i] = motion;
                    }
                }
            }
            // toggle
            4 => {
                let min = orders[1]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let max = orders[3]
                    .split(',')
                    .into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                println!("{:?} {:?}", min, max);

                for i in min[0]..=max[0] {
                    for j in min[1]..=max[1] {
                        light_grid[j][i] = !light_grid[j][i];
                    }
                }
            }
            _ => (),
        }
    }
    let mut light_counter: u32 = 0;
    for light_line in light_grid {
        for light in light_line {
            light_counter += light as u32;
        }
    }

    println!("{}", light_counter);
}
