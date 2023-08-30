use std::fs::File;
use std::io::Read;

use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let bench = Instant::now();

    let mut input = String::new();
    File::open("assets/08.txt").unwrap().read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split('\n').collect();

    let mut code_cumulative: u32 = 0;
    let mut text_cumulative: u32 = 0;

    for line in input[..input.len()-1].into_iter() {
        let mut code_counter: u32 = 2;
        let mut text_counter: u32 = 0;

        let mut scaped: bool = false;
        let mut skip: u32 = 0;
        for c in line[1..line.len()-1].chars() {
            if skip > 0 {
                skip -= 1;
                continue
            }
            if scaped {
                match c {
                    '\\' | '\"' => {
                        code_counter += 2;
                        text_counter += 1;
                    },
                    'x' => {
                        code_counter += 4;
                        text_counter += 1;
                        skip = 2;
                    }
                    _ => (),
                }
                scaped = false;
            } else {
                match c {
                    '\\' => {
                        scaped = true;
                    },
                    _ => {
                        code_counter += 1;
                        text_counter += 1;
                    },
                }
            }
        }
        code_cumulative += code_counter;
        text_cumulative += text_counter;
        //println!(" | {} {} {}", line, code_counter, text_counter);
    }

    println!("{}", code_cumulative - text_cumulative);
    println!("{:.2?}", bench.elapsed());

}
