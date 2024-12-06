use std::{collections::HashMap, fs::read_to_string};

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

    println!("task 01: {:>8}", task_01(&left_list, &right_list));
    println!("task 02: {:>8}", task_02(&left_list, &right_list));
}

fn task_01(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    // error checks length
    assert!(left_list.len() == right_list.len());

    let mut dif_sum: i32 = 0;
    // iterates lists and add the difference
    for i in 0..left_list.len() {
        dif_sum += (left_list[i] - right_list[i]).abs();
    }

    dif_sum
}

// assumes both lists are sorted
fn task_02(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut dif_sum: i32 = 0;

    let mut cache: HashMap<i32, i32> = HashMap::new();

    let mut ptr = 0;
    for i in 0..left_list.len() {
        //println!("{:?} {:?} {}", cache, left_list[i], ptr);
        let dif = match cache.get(&left_list[i]) {
            Some(v) => *v,
            None => {
                let mut j = ptr;
                let mut c = 0;
                // find the next instance of the left value, skips dummy values
                if j < right_list.len() {
                    while right_list[j] != left_list[i] {
                        j += 1;
                        if j >= right_list.len() - 1 {
                            break;
                        }
                    }

                    // find the amount of numbers equal to the left value
                    if j < right_list.len() {
                        while right_list[j + c] == left_list[i] {
                            c += 1;
                            if c + j >= right_list.len() - 1 {
                                break;
                            }
                        }
                    }
                }

                // if there is no instance of the left number in the right list
                if j == right_list.len() - 1 {
                    cache.insert(left_list[i], 0);
                    0
                } else {
                    ptr = j + c;
                    let v = (c as i32 * left_list[i]).abs();
                    cache.insert(left_list[i], v);
                    v
                }
            }
        };
        dif_sum += dif;
    }

    dif_sum
}
