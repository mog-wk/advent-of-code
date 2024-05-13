#![allow(unused)]

use std::fs::File;
use std::io::Read;

// R1: 535088217

fn main() {
    let mut inp = File::open("./input.txt").unwrap();
    let mut st = String::new();
    inp.read_to_string(&mut st).unwrap();
    std::mem::drop(inp);

    let (seeds, maps) =  preprocess(&st);

    //println!("{:?}", p_1(&seeds, &maps, 8));
    println!("{:?}", p_2(&seeds, &maps, 8));
}

fn p_2(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>, depth: usize) -> u64 {
    // NOTE: bulk way find awswer with less space complexity
    // passes test but crashes the terminal
    let mut seeds_buffer = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let (init, inc) = (seeds[i], seeds[i+1]);
        for j in 0..inc {
            seeds_buffer.push(init+j);
        }
    }
    println!("seeds: {:?}", seeds);
    for (n, map) in maps.into_iter().enumerate() {
        if n > depth {
            break 
        }
        println!("=== layer: {} seeds: {:?}", n, seeds_buffer);

        let mut marked = vec![false; seeds_buffer.len()];

        for instruction in map {
            //println!("{:?}", instruction);

            let dest = instruction[0];
            let source = instruction[1];
            let append = instruction[2];

            // seed is now ranges
            for (i, seed) in seeds_buffer.iter_mut().enumerate() {
                if marked[i] {
                    continue
                }
                let range = (source..=source + append);
                if !range.contains(seed) {
                    continue
                }
                //println!("changing: {}", seed); 

                // NOTE: stupid way to do it, no time rn fix later
                if dest >= source {
                    *seed = *seed + (dest - source);
                 } else {
                    *seed = *seed - (source - dest);
                }
                marked[i] = true;
            }
        }
    }
    seeds_buffer.into_iter().reduce(u64::min).unwrap()
}
fn p_1(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>, depth: usize) -> u64 {
    let mut seeds_buffer = seeds.clone();
    println!("seeds: {:?}", seeds);
    for (n, map) in maps.into_iter().enumerate() {
        if n > depth {
            break 
        }
        println!("=== layer: {} seeds: {:?}", n, seeds_buffer);

        let mut marked = vec![false; seeds_buffer.len()];

        for instruction in map {
            //println!("{:?}", instruction);

            let dest = instruction[0];
            let source = instruction[1];
            let append = instruction[2];

            for (i, seed) in seeds_buffer.iter_mut().enumerate() {
                if marked[i] {
                    continue
                }
                let range = (source..=source + append);
                if !range.contains(seed) {
                    continue
                }
                //println!("changing: {}", seed); 

                // NOTE: stupid way to do it, no time rn fix later
                if dest >= source {
                    *seed = *seed + (dest - source);
                 } else {
                    *seed = *seed - (source - dest);
                }
                marked[i] = true;
            }
        }
    }
    seeds_buffer.into_iter().reduce(u64::min).unwrap()
}

fn preprocess(st: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let sections = st.splitn(10, "\n\n").collect::<Vec<&str>>();

    let seeds = sections[0].split(':').collect::<Vec<&str>>()[1]
        .split('\n')
        .collect::<String>();
    let seeds = seeds
        .trim()
        .split(' ')
        .map(|v| match v.parse::<u64>() {
            Ok(v) => v,
            Err(e) => panic!("failed to convert to u64: {}", e),
        })
        .collect::<Vec<u64>>();

    println!("{:#?}", sections);

    let maps = sections[1..].into_iter().map(|sec| {
        let sec = sec.rsplit(':')
            .next()
            .unwrap()
            .trim()
            .split('\n')
            .collect::<Vec<&str>>();

        sec
            .into_iter()
            .map(|l| {
                l
                    .split(' ')
                    .map(|v| match v.parse::<u64>() {
                        Ok(v) => v,
                        Err(e) => panic!("failed to convert to u64: {}", e),
                    })
                .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>()
    }).collect::<Vec<_>>();


    (seeds, maps)
}

fn type_of<T>(val: T) -> String {
    format!("{:?}", std::any::type_name::<T>())
}

#[cfg(test)]
#[path = "./_tests/preprocess.rs"]
mod tests_preprocess;

#[cfg(test)]
#[path = "./_tests/p_1.rs"]
mod tests_p_1;

#[cfg(test)]
#[path = "./_tests/p_2.rs"]
mod tests_p_2;
