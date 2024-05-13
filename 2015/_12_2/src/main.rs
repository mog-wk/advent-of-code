use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    //File::open("assets/12.txt").unwrap().read_to_string(&mut input).unwrap();
    File::open("_12_2/src/ts.txt").unwrap().read_to_string(&mut input).unwrap();
    //let mut input = "[1,{\"c\":\"red\",\"b\":2},3]".to_string();

    let bench = Instant::now();

    let mut cache: HashMap<u32, Vec<String>> = HashMap::new();
    let mut str_buffer = String::new();
    let mut cache_capture: bool = false;

    for c in input.chars() {

    }
    let mut sum: i32 = 0;
    println!("{:}", input);
    for (k, v) in cache {
        println!("{} ===> {:?}", k, v);
        if v.contains(&"red".to_string()) { continue }
        let s: i32 = v.into_iter().map(|n| {
            match n.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            }
        }).sum();
        sum += s;
    }
    println!("{} {:.2?}", sum, bench.elapsed());
}
