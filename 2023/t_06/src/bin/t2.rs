#![allow(unused)]

mod parser_2;

fn main() {
    let r1 = task_1();
    println!("{:?}", r1);
}

fn task_1() -> u32 {
    let races = parser_2::parse("inputs/input_1.txt");
    println!("{:?}", races);
    let mut attempt_num = vec![];
    for (final_time, final_dist) in races {
        let mut passed = vec![];
        for speed in 0..final_time {
            let dist = speed * (final_time - speed);
            if dist > final_dist {
                passed.push(dist);
            }
        }
        attempt_num.push(passed.len());
    }
    attempt_num.into_iter().reduce(|c, v| c * v).unwrap() as u32
}
