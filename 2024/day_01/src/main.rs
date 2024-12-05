use std::{cmp::min, fs::read_to_string};

fn main() {
    let input = read_to_string("./day_01/src/input.txt")
        .expect("unable to find file\nrun cargo from workspace directory");
    // parse lists
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (
                l.parse::<i32>().expect("l not a number"),
                r.parse::<i32>().expect("r not a number"),
            )
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let ln = min(left_list.len(), right_list.len());

    let mut dif_sum: i32 = 0;
    for i in 0..ln {
        dif_sum += (left_list[i] - right_list[i]).abs();
    }

    println!("{}", dif_sum);
}
