use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./day_05/src/test.txt").unwrap();
    let (rules, process_list) = input.split_once("\n\n").unwrap();

    // key should be before all values
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules.lines() {
        let (key, value) = rule.split_once('|').unwrap();

        let arr = rule_map
            .entry(key.parse::<u32>().unwrap())
            .or_insert(Vec::new());

        arr.push(value.parse::<u32>().unwrap());
    }

    //println!("task 01: {}", task_01(rule_map, process_list));
    println!("task 02: {}", task_02(rule_map, process_list));
}

// BROKEN
fn task_02(rule_map: HashMap<u32, Vec<u32>>, process_list: &str) -> u32 {
    let mut valid_ps = 0;
    println!("{:?}", rule_map);
    for process in process_list.lines() {
        println!(">");
        let mut valid = true;
        let mut process: Vec<u32> = process
            .split(',')
            .map(|ps| ps.parse::<u32>().unwrap())
            .collect();
        let process_iter = process.clone();
        for (i, anchor) in process_iter.iter().enumerate() {
            let mut k = i;
            for (j, num) in process_iter.iter().enumerate() {
                match rule_map.get(anchor) {
                    Some(after_ps) => {
                        print!(
                            "({} {}), {} {:?}, {:?} | ",
                            i, j, anchor, process[k], process[j]
                        );
                        if j > i {
                            if !after_ps.contains(num) {
                                valid = false;
                                process.swap(i, j);
                                print!("swap foward ");
                                k = j;
                            }
                        } else if j < i {
                            if after_ps.contains(num) {
                                valid = false;
                                process.swap(i, j);
                                print!("swap backwards ");
                                k = j;
                            }
                        }
                    }
                    None => {}
                }
            }
            println!("::{:?}", process);
        }

        if !valid {
            let mid = process[process.len() / 2];

            println!("{:?} {:?} {}", process_iter, process, mid);
            valid_ps += mid;
        }
    }
    valid_ps
}

fn order_ps(rule_map: HashMap<u32, Vec<u32>>, process: &mut Vec<u32>) {}

// check process validity
fn task_01(rule_map: HashMap<u32, Vec<u32>>, process_list: &str) -> u32 {
    let mut valid_ps = 0;
    println!("{:?}", rule_map);
    for process in process_list.lines() {
        let mut valid = true;
        let process: Vec<u32> = process
            .split(',')
            .map(|ps| ps.parse::<u32>().unwrap())
            .collect();
        for (i, anchor) in process.iter().enumerate() {
            for (j, num) in process.iter().enumerate() {
                match rule_map.get(anchor) {
                    Some(after_ps) => {
                        if j > i {
                            if !after_ps.contains(num) {
                                valid = false;
                                break;
                            }
                        } else if j < i {
                            if after_ps.contains(num) {
                                valid = false;
                                break;
                            }
                        } else {
                            ()
                        }
                    }
                    None => {}
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            let mid = process[process.len() / 2];

            println!("{:?} {}", process, mid);
            valid_ps += mid;
        }
    }
    valid_ps
}
