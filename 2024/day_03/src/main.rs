use std::fs::canonicalize;

use regex::Regex;

fn main() {
    let mut f = canonicalize(file!()).unwrap();
    f.pop();
    f.push("input.txt");

    let input = std::fs::read_to_string(f).unwrap();

    //println!("task 01: {:>}", task_01(&input));
    println!("task 02: {:>}", task_02(&input));
}

fn task_01(input: &str) -> u32 {
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let mut total = 0;
    for (_, [op_str]) in re.captures_iter(input).map(|c| c.extract()) {
        let n = op_str[4..].split_once(',').unwrap();
        let n = (
            n.0.parse::<u32>().unwrap(),
            n.1[..n.1.len() - 1].parse::<u32>().unwrap(),
        );

        total += n.0 * n.1;

        println!("{:?} {:?}", op_str, n);
    }

    total
}

fn task_02(input: &str) -> u32 {
    let donts: Vec<usize> = Regex::new(r"(don't\(\))")
        .unwrap()
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().start())
        .collect();
    let dos: Vec<usize> = Regex::new(r"(do\(\))")
        .unwrap()
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().start())
        .collect();

    let mut is_enable = true;
    // dos arr tracker
    let mut i = 0;
    // donts arr tracker
    let mut j = 0;

    // every odd index is a enable start and every even index is a disable start
    let mut intervals = vec![0];

    while i < dos.len() && j < donts.len() {
        if is_enable {
            let value = donts[j];
            if value < dos[i] {
                intervals.push(value);
                is_enable = false;
            } else {
                i += 1;
            }
        } else {
            let value = dos[i];
            if value < donts[j] {
                intervals.push(value);
                is_enable = true;
            } else {
                j += 1;
            }
        }
    }

    if i < dos.len() {
        intervals.push(dos[i]);
    }

    if j < donts.len() {
        intervals.push(donts[j]);
    }

    intervals.push(0);

    let muls: Vec<(usize, u32)> = Regex::new(r"(mul\(\d+,\s*\d+\))")
        .unwrap()
        .captures_iter(&input)
        .map(|c| {
            let index = c.get(1).unwrap().start();
            let (_, [ops]) = c.extract();

            let n = ops[4..].split_once(',').unwrap();
            (
                index,
                n.0.parse::<u32>().unwrap() * n.1[..n.1.len() - 1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut bulk = 0;
    j = 0;

    //println!("{:?} {:?} {:?}", intervals, dos, donts);
    println!("{:?}", intervals);
    for (i, n) in muls {
        println!("{:?} {} {}", i, n, j);
        if i > intervals[j] {
            if j < intervals.len() - 1 {
                j += 1;
            }
        }
        if j & 1 == 1 {
            println!("added: {}", n);
            bulk += n;
        }
    }
    bulk
}
