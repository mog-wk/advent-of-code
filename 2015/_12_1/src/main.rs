
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    File::open("assets/12.txt").unwrap().read_to_string(&mut input).unwrap();

    let bench = Instant::now();

    let mut cache: Vec<i32> = Vec::new();
    let mut val_str = String::with_capacity(4);
    let mut cache_capture: bool = false;

    for c in input.chars() {
        if cache_capture {
            match c {
                // end of string
                ',' | '}' | ']' => {
                    cache_capture = false;
                    match val_str.parse() {
                        Ok(v) => cache.push(v),
                        Err(e) => println!("{}", e),
                    }
                    val_str.clear();
                },
                // string content
                '0'..='9' | '-' => {
                    val_str += &c.to_string();
                }
                _ => cache_capture = false,
            }
        }
        if c == ':' || c == ',' || c == '[' {
            cache_capture = true;
        }
    }
    println!("{:?} {:.2?}", cache.into_iter().sum::<i32>(), bench.elapsed());
}
