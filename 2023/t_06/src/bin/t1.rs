#![allow(unused)]

mod parser_1;

fn main() {
    let r1 = task_1("inputs/input_1.txt");
    println!("{:?}", r1);
}

fn task_1(path: &str) -> u32 {
    let races = crate::parser_1::parse(path);
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

#[cfg(test)]
mod tests {
    #[test]
    fn exp() {
        use super::task_1;
        let res = task_1("inputs/exp.txt");
        assert_eq!(res, 288);
    }
}
