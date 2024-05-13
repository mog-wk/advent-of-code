#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use std::io::Write;

// R1: 533775
// R2: 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_dev = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";

    let time = Instant::now();

    let mut handler = std::io::BufWriter::new(std::io::stdout());

    let mut st = String::new();
    let mut file = File::open("input.txt")?;
    file.read_to_string(&mut st)?;

    //let r1 = p_1(&test_dev, None)?;
    let r2 = p_2(&st, None, &mut handler)?;

    //println!("{}", r1);
    writeln!(&mut handler, "r2: {}", r2);
    writeln!(&mut handler, "{:?}", time.elapsed());

    Ok(())
}

fn p_2(st: &str, ln: Option<usize>, handler: &mut std::io::BufWriter<impl Write>) -> Result<u64, Box<dyn std::error::Error>> {

    let lines = st.split('\n').collect::<Vec<&str>>();
    let lines: Vec<_> = lines.into_iter().map(|l| l.as_bytes()).collect();
    let lines = &lines[0..lines.len() - 1];
    //let lines = &lines[107..=114];

    let ln = match ln {
        Some(v) => v,
        None => lines[0].len() - 1,
    };

    let (mut i, mut j) = (0, 0);
    let mut gear_numbers: Vec<(u32, u32)> = vec![];
    let mut gear_numbers_sum: u64 = 0;

    while j < lines.len() {
        //writeln!(handler, "\n j: {} ====================\n", j);
        while i < ln {
            let cur_byte = lines[j][i];
            //write!(handler, "j: {} i: {} c_b: {}", j, i, cur_byte);
            if cur_byte != b'*' {
                i += 1;
                continue;
            }

            writeln!(handler, "======== Gear: ({}, {}) ========", i, j);
            let mut num = (None, None);
            let mut off_set = (0, None);
            // scan sybols
            for pi in 0..=2 {

                let mut pi = match usize::checked_sub(i + pi, 1) {
                    Some(v) => v,
                    None => continue,
                };
                for pj in 0..=2 {
                    let pi = match off_set.1 {
                        Some(v) => {
                            let mut x = 0;
                            if v == pj {
                                x = pi + off_set.0
                            }
                            x
                        },
                        None => pi,
                    };
                    if pi >= ln {
                        continue;
                    }
                    let pj = match usize::checked_sub(j + pj, 1) {
                        Some(v) => v,
                        None => continue,
                    };
                    if pj >= lines.len() {
                        continue;
                    }
                    let scan_byte = lines[pj][pi];
                    writeln!(handler, "scan byte: {:?}   pi: {}, pj: {}   i: {}, j: {}  off_set: {:?}", scan_byte as char, pi, pj, i, j, off_set);
                    
                    match scan_byte {
                        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                            let mut balance = 0;
                            let mut n = String::new();

                            // look back
                            writeln!(handler, "look back:");
                            let mut i_ind = pi;
                            while lines[pj][i_ind]>= 48 && lines[pj][i_ind]<= 57 {
                                //write!(handler, "num: {:?}, pos: {:?}", lines[pj][i_ind] as char, (i_ind, pj));
                                balance += 1;
                                i_ind = match usize::checked_sub(i_ind, 1) {
                                    Some(v) => v,
                                    None => break,
                                };
                            }
                            for b in (0..balance).rev() {
                                n.push(lines[pj][pi - b] as char);
                            }
                            writeln!(handler, "\nlook foward:");

                            // look foward
                            let mut balance = 0;
                            let mut i_ind = pi;
                            while lines[pj][i_ind] >= 48 && lines[pj][i_ind] <= 57 {
                                //write!(handler, "num: {:?}, pos: {:?}", lines[pj][i_ind] as char, (i_ind, pj));
                                balance += 1;
                                i_ind += 1;
                                if i_ind > ln {
                                    break;
                                }
                            }
                            for b in 1..balance {
                                n.push(lines[pj][pi + b] as char);
                            }
                            //writeln!(handler, "");

                            // shift i
                            off_set.0 = balance-1;
                            off_set.1 = Some(pj);
                            if num.0 == None {
                                num.0 = Some(n.parse::<u32>().unwrap());
                                writeln!(handler, "+ add_1:  {:?} {} {} {} {}", num, i, j, pi, pj);
                                continue;
                            }
                            if num.1 == None {
                                num.1 = Some(n.parse::<u32>().unwrap());
                                writeln!(handler, "+ add_2:  {:?} {} {} {} {}", num, i, j, pi, pj);

                                off_set.0 = 0;
                                off_set.1 = None;

                                let num = (num.0.unwrap(), num.1.unwrap());
                                gear_numbers.push(num);
                                gear_numbers_sum += (num.0 * num.1) as u64;

                                writeln!(handler, "+ push: {:?} {} {} {} {}", num, i, j, pi, pj);

                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
            i += 1;
        }
        j += 1;
        i = 0;
    }

    writeln!(handler, "{:?} {} {}", gear_numbers, gear_numbers_sum, gear_numbers.len());
    handler.flush();
    Ok(gear_numbers_sum)
}

fn p_1(st: &str, ln: Option<usize>) -> Result<u32, Box<dyn std::error::Error>> {
    let lines = st.split('\n').collect::<Vec<&str>>();
    let lines: Vec<_> = lines.into_iter().map(|l| l.as_bytes()).collect();
    let lines = &lines[0..lines.len() - 1];
    //let lines = &lines[134..135];

    let ln = match ln {
        Some(v) => v,
        None => lines[0].len() - 1,
    };

    let (mut i, mut j) = (0, 0);
    let mut part_numbers: Vec<u32> = vec![];
    let mut part_numbers_sum: u32 = 0;

    while j < lines.len() {
        println!("\n j: {} ====================", j);
        while i < ln {
            let mut pad_i = 0;
            let mut n = String::new();
            while lines[j][i + pad_i] >= 48 && lines[j][i + pad_i] <= 57 {
                n.push(lines[j][i + pad_i] as char);
                pad_i += 1;
                if pad_i + i > ln {
                    break;
                }
            }
            println!("j: {} i: {} n: {:04}", j, i, n);
            // scan sybols
            if n.len() > 0 {
                for pi in 0..n.len() + 2 {
                    let pi = match usize::checked_sub(i + pi, 1) {
                        Some(v) => v,
                        None => continue,
                    };
                    if pi >= ln {
                        continue;
                    }
                    for pj in 0..3 {
                        let pj = match usize::checked_sub(j + pj, 1) {
                            Some(v) => v,
                            None => continue,
                        };
                        if pj >= lines.len() {
                            continue;
                        }
                        let scan_byte = lines[pj][pi];
                        print!("{} {} | ", pi, pj);
                        println!("{:?} ", scan_byte as char);
                        match scan_byte {
                            b'*' | b'&' | b'%' | b'#' | b'@' | b'^' | b'$' | b'+' | b'-' | b'/'
                            | b'\\' | b'!' | b'=' | b'_' | b'(' | b')' => {
                                let num = n.parse::<u32>().unwrap();
                                part_numbers.push(num);
                                part_numbers_sum += num;
                            }
                            _ => (),
                        }
                    }
                }
            }
            i += 1;
            i += match usize::checked_sub(pad_i, 1) {
                Some(v) => v,
                None => 0,
            };
        }
        j += 1;
        i = 0;
    }

    println!(
        "{:?} {} {}",
        part_numbers,
        part_numbers_sum,
        part_numbers.len()
    );
    Ok(part_numbers_sum)
}
