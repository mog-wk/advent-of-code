use std::{fs::File, io::Read};

pub fn parse(path: &str) -> Vec<(u32, u32)> {
    let mut file = File::open(path).unwrap();
    let mut raw = String::new();
    //let raw = include_str!("../inputs/input_1.txt");
    let _ = file.read_to_string(&mut raw).unwrap();
    let raw: Vec<_> = raw.split('\n').collect();
    let raw = raw[..raw.len() - 1].to_vec();
    let raw: Vec<_> = raw
        .into_iter()
        .map(|st: &str| st.split_once(':').unwrap().1)
        .map(|st: &str| st.strip_prefix(' ').unwrap())
        .collect();

    let mut time_dist: Vec<u32> = Vec::with_capacity(4);
    for line in raw.iter() {
        let mut num = "".to_string();
        for x in line.chars() {
            match x {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    num.push(x);
                }
                ' ' => {
                    if num.len() > 0 {
                        time_dist.push(num.parse().unwrap());
                        num.clear();
                    }
                    ()
                }
                _ => unreachable!(),
            }
        }
        // last interation not reacheble
        time_dist.push(num.parse().unwrap());
    }
    let mut res = Vec::with_capacity(4);

    for i in 0..time_dist.len() / 2 {
        let j = time_dist.len() / 2 + i;
        res.push((time_dist[i], time_dist[j]));
    }

    res
}
