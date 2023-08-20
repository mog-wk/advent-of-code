use std::fs::File;
use std::io::Read;

use std::time::Instant;

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    File::open("assets/07.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let bench = Instant::now();

    let input: Vec<&str> = input.split('\n').collect();

    let mut signals: [u16; 65535] = [0; 65535];

    let mut state: [bool; 65535] = [false; 65535];

    let mut buffer: Vec<&str> = Vec::new();

    for line in input[0..input.len() - 1].into_iter() {
        let line: Vec<&str> = line.split(' ').collect();
        println!("{:?}", line);
        // separete between 1 input (NOT) and 2 input (AND, OR, XOR) operations
        match line.len() {
            5 => {
                let op = line[1];
                match op {
                    "AND" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            str_to_u16(line[2]).unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] =
                                signals[inp_1 as usize] & signals[inp_2 as usize];
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                        println!("{} {} {:?} {}", inp_1, inp_2, op, signals[out_1 as usize]);
                    }
                    "OR" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            str_to_u16(line[2]).unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] =
                                signals[inp_1 as usize] | signals[inp_2 as usize];
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                        println!("{} {} {:?} {}", inp_1, inp_2, op, signals[out_1 as usize]);
                    }
                    "LSHIFT" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            line[2].parse().unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] = signals[inp_1 as usize] << inp_2;
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                        println!("{} {} {}", inp_1, inp_2, signals[out_1 as usize]);
                    }
                    "RSHIFT" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            line[2].parse().unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] = signals[inp_1 as usize] >> inp_2;
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                    }
                    _ => (),
                }
            }
            4 => {
                let (inp_1, out_1): (u16, u16) =
                    (str_to_u16(line[1]).unwrap(), str_to_u16(line[3]).unwrap());
                if state[inp_1 as usize] != false {
                    signals[out_1 as usize] = !inp_1;
                    state[out_1 as usize] = true;
                } else {
                    buffer.push(line);
                }
                println!("{}", signals[out_1 as usize]);
            }
            3 => {
                for c in line[0].bytes() {
                    match c {
                        b'a'..=b'z' => {
                            let (inp_1, out_1): (u16, u16) =
                                (str_to_u16(line[0]).unwrap(), str_to_u16(line[2]).unwrap());
                            signals[out_1 as usize] = signals[inp_1 as usize];
                        }
                        b'0'..=b'9' => {
                            let (inp_1, out_1): (u16, u16) =
                                (line[0].parse().unwrap(), str_to_u16(line[2]).unwrap());
                            signals[out_1 as usize] = inp_1;
                        }
                        _ => (),
                    }
                    break;
                }
            }
            _ => (),
        }
    }
    for line in buffer.iter() {
        match line.len() {
            5 => {
                let op = line[1];
                match op {
                    "AND" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            str_to_u16(line[2]).unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] =
                                signals[inp_1 as usize] & signals[inp_2 as usize];
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                        println!("{} {} {:?} {}", inp_1, inp_2, op, signals[out_1 as usize]);
                    }
                    "OR" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            str_to_u16(line[2]).unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] =
                                signals[inp_1 as usize] | signals[inp_2 as usize];
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                        println!("{} {} {:?} {}", inp_1, inp_2, op, signals[out_1 as usize]);
                    }
                    "LSHIFT" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            line[2].parse().unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] = signals[inp_1 as usize] << inp_2;
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                        println!("{} {} {}", inp_1, inp_2, signals[out_1 as usize]);
                    }
                    "RSHIFT" => {
                        let (inp_1, inp_2, out_1): (u16, u16, u16) = (
                            str_to_u16(line[0]).unwrap(),
                            line[2].parse().unwrap(),
                            str_to_u16(line[4]).unwrap(),
                        );
                        if state[inp_1 as usize] != false && state[inp_2 as usize] != false {
                            signals[out_1 as usize] = signals[inp_1 as usize] >> inp_2;
                            state[out_1 as usize] = true;
                        } else {
                            buffer.push(line);
                        }
                    }
                    _ => (),
                }
            }
            4 => {
                let (inp_1, out_1): (u16, u16) =
                    (str_to_u16(line[1]).unwrap(), str_to_u16(line[3]).unwrap());
                if state[inp_1 as usize] != false {
                    signals[out_1 as usize] = !inp_1;
                    state[out_1 as usize] = true;
                } else {
                    buffer.push(line);
                }
                println!("{}", signals[out_1 as usize]);
            }
            3 => {
                for c in line[0].bytes() {
                    match c {
                        b'a'..=b'z' => {
                            let (inp_1, out_1): (u16, u16) =
                                (str_to_u16(line[0]).unwrap(), str_to_u16(line[2]).unwrap());
                            signals[out_1 as usize] = signals[inp_1 as usize];
                        }
                        b'0'..=b'9' => {
                            let (inp_1, out_1): (u16, u16) =
                                (line[0].parse().unwrap(), str_to_u16(line[2]).unwrap());
                            signals[out_1 as usize] = inp_1;
                        }
                        _ => (),
                    }
                    break;
                }
            }
            _ => (),
        }
    }
    println!("{}", signals[str_to_u16("0a").unwrap() as usize]);

    println!("{:.2?}", bench.elapsed());
}

fn str_to_u16(s: &str) -> Result<u16, &str> {
    match s.len() {
        2 => {
            let b16 = s.encode_utf16().collect::<Vec<u16>>();
            let b16: u16 = ((b16[0] << 8) + b16[1]).into();
            Ok(b16)
        }
        1 => {
            let s = "0".to_owned() + s;
            let b16 = s.encode_utf16().collect::<Vec<u16>>();
            let b16: u16 = ((b16[0] << 8) + b16[1]).into();
            Ok(b16)
        }
        _ => return Err("invalid length for string"),
    }
}
