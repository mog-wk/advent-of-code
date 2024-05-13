#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

const NIL: i32 = -2; // sentinel value

// R1: 23678
// R2: 15455663

fn main() {
    // preprocess
    let mut inp = File::open("input.txt").expect("unable to open input file");
    let mut st = String::new();
    inp.read_to_string(&mut st);
    std::mem::drop(inp);

    // split by line
    let st = st.lines().collect::<Vec<&str>>();
    let st = &st[0..st.len()]; // delete trailling ""


    println!("{}", p_2(st));

    //println!("p1: {}\np2: {}", p_1(st), p_2(st));
}

fn p_2(st: &[&str]) -> u32 {
    let mut copy_count: HashMap<u32, u32> = HashMap::new();
    let mut bulk_sum: u32 = 0;
    let mut max = 1_u32;

    for line in st.iter() {
        let game_id: u32 = line
            .split(':').collect::<Vec<&str>>()[0]
            .rsplit(' ').collect::<Vec<&str>>()[0]
            .parse::<u32>().unwrap();
        copy_count.insert(game_id, 1_u32);

        if game_id > max {
            max = game_id;
        }
    }

    for line in st.iter() {
        let (game_id, winning_numbers, round_numbers) = preprocess_line(line);
        println!( "---- {}: {:?} {:?}", game_id, winning_numbers, round_numbers);

        let mut score = 1_u32;
        for num in round_numbers.into_iter() {
            if num == NIL {
                continue;
            }
            if winning_numbers.contains(&num) {
                score += 1;
            }
        }
        for i in 1..score {
            let index = i + game_id;
            if index > max {
                break
            }
            let mult = copy_count.get(&game_id).unwrap().clone();

            println!("added to {} {} times", index, mult);
            copy_count.entry(index).and_modify(|v| *v += mult);
        }
        println!("adding to sum: {}", copy_count.get(&game_id).unwrap());
        bulk_sum += copy_count.get(&game_id).unwrap(); 
    }
    println!("{:?}", copy_count);
    bulk_sum
}

fn p_1(st: &[&str]) -> u32 {
    let mut bulk_sum: u32 = 0;
    for line in st.iter() {
        let (game_id, winning_numbers, round_numbers) = preprocess_line(line);
        println!(
            "---- {}: {:?} {:?}",
            game_id, winning_numbers, round_numbers
        );

        let mut score = 0;
        for num in round_numbers.into_iter() {
            if winning_numbers.contains(&num) {
                score += 1;
                println!("num: {} score: {}", num, score);
            }
        }
        if score > 0 {
            let sum = 2_u32.pow(score - 1);
            println!("added: {}", sum);
            bulk_sum += sum
        }
    }
    bulk_sum
}

fn preprocess_line(line: &str) -> (u32, HashSet<i32>, Vec<i32>) {
    let fill = line.split(':').collect::<Vec<&str>>();
    let game_id = fill[0].rsplit(' ').collect::<Vec<&str>>()[0];
    let fill = fill[1].split('|').collect::<Vec<&str>>();
    let winning_numbers = fill[0]
        .split(' ')
        .map(|s| match s.parse::<i32>() {
            Ok(v) => v,
            Err(e) => NIL,
        })
        .collect::<Vec<i32>>();
    let winning_numbers = parse_to_hashset(winning_numbers);

    let round_numbers = fill[1]
        .split(' ')
        .map(|s| match s.parse::<i32>() {
            Ok(v) => v,
            Err(e) => NIL,
        })
        .collect::<Vec<i32>>();
    (
        game_id.parse::<u32>().unwrap(),
        winning_numbers,
        round_numbers[1..].to_vec(),
    )
}

fn parse_to_hashset<T>(vec: Vec<T>) -> HashSet<T>
where
    T: Sized + Eq + std::hash::Hash + Copy + Into<i32>,
{
    let mut set: HashSet<T> = HashSet::new();
    for el in vec.into_iter() {
        if el.into() != NIL {
            set.insert(el);
        }
    }
    set
}

#[cfg(test)]
#[path = "./_tests/test.rs"]
mod tests;
