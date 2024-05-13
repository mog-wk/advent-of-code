#![allow(unused)]
use std::io::Read;
use std::collections::HashMap;
// R: 54081

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./input/01.txt")?;

    let mut st = String::new();
    file.read_to_string(&mut st)?;

    let st: Vec<&str> = st.split('\n').collect();

    //p1()?;
    println!("{}", p2(st)?);
    Ok(())

}

fn p2(st: Vec<&str>) -> std::io::Result<u64> {


    let mut line_cal_values = vec![];
    let number_string_start = vec![b'o', b't', b'f', b's', b'e', b'n'];
    let mut number_map: HashMap<Vec<u8>, u8> = HashMap::new();
    number_map.insert("one".bytes().collect::<Vec<u8>>(), 48 + 1);
    number_map.insert("two".bytes().collect::<Vec<u8>>(), 48 + 2);
    number_map.insert("three".bytes().collect::<Vec<u8>>(), 48 + 3);
    number_map.insert("four".bytes().collect::<Vec<u8>>(), 48 + 4);
    number_map.insert("five".bytes().collect::<Vec<u8>>(), 48 + 5);
    number_map.insert("six".bytes().collect::<Vec<u8>>(), 48 + 6);
    number_map.insert("seven".bytes().collect::<Vec<u8>>(), 48 + 7);
    number_map.insert("eight".bytes().collect::<Vec<u8>>(), 48 + 8);
    number_map.insert("nine".bytes().collect::<Vec<u8>>(), 48 + 9);
    //println!("{:?}", number_map);

    let number_map = number_map;

    for line in st {
        let mut calibration_values = (None, None);
        let line: Vec<u8> = line.bytes().collect();
        let mut i = 0;
        while i < line.len() {

            println!("====== iter: {}", i);
            let mut b = line[i]; 
            if b >= 48 && b <= 57 {
                if calibration_values.0 == None {
                    calibration_values.0 = Some(b);
                    calibration_values.1 = Some(b);
                } else {
                    calibration_values.1 = Some(b);
                }
            } else {
                for val in number_string_start.iter() {
                    if i >= line.len() - 2 {
                        break
                    }
                    if &b == val {
                        println!("==== {} i: {}\nline: {:?}", b as char, i, line);
                        for (k, v) in number_map.iter() {
                            let mut i_offset = 0;
                            let mut j = 0;
                            let mut added_num = false;
                            println!("{:?} {:?}", *v as char, k);
                            //println!("k[j]: {:?} line[i + offset]: {:?}", k[j], line[i + i_offset]);
                            while k[j] == line[i + i_offset] {
                                print!("{} {}+{} | ", k[j] as char, i, i_offset);

                                println!("j: {}", j);
                                // found word
                                if j == k.len()-1 {
                                    println!("\nadding: {:?}", v - 48, );
                                    if calibration_values.0 == None {
                                        calibration_values.0 = Some(*v);
                                        calibration_values.1 = Some(*v);
                                    } else {
                                        calibration_values.1 = Some(*v);
                                    }
                                    if i + i_offset < line.len() {
                                        i += i_offset-1;
                                    }
                                    added_num = true;
                                } else {
                                    j += 1;
                                }
                                i_offset += 1;
                                if i + i_offset >= line.len() {
                                    i_offset = 0;
                                    break
                                }
                            }
                            i_offset = 0;
                            if added_num {
                                break
                            }
                        }
                    }
                }
            }
            //println!("i: {}", i);
            i += 1;
        }

        // skip if no numbers where found
        if calibration_values.0 == None {
            continue
        }

        let calibration_values = (
            calibration_values.0.unwrap(),
            calibration_values.1.unwrap(),
            );
        //println!("cal_val:{:?}", calibration_values);
        let mut sum = String::from_utf8(vec![calibration_values.0, calibration_values.1]).unwrap();

        line_cal_values.push(sum.clone());

        //println!("cal_v: {:?} {:?}", calibration_values, sum);

    }
    println!("{:?}", line_cal_values);

    let mut sum = 0;
    for v in line_cal_values {
        sum += v.parse::<u64>().unwrap();
    }
    Ok(sum)
}

fn p1() -> std::io::Result<()> {
    // dev-test

    let mut file = std::fs::File::open("./input/01.txt")?;
    let mut st = String::new();
    file.read_to_string(&mut st)?;

    let mut line_cal_values = vec![];

    let st = st.split('\n');

    for line in st {
        
        //println!("{:?}", line);
        let mut calibration_values = (None, None);
        for b in line.bytes() {
            if b >= 48 && b <= 57 {
                if calibration_values.0 == None {
                    calibration_values.0 = Some(b);
                    calibration_values.1 = Some(b);
                } else {
                    calibration_values.1 = Some(b);
                }
            }
        }

        if calibration_values.0 == None {
            continue
        }

        let calibration_values = (
            calibration_values.0.unwrap(),
            calibration_values.1.unwrap(),
            );

        //println!("calc_ val: {:?}", calibration_values);
        let mut sum = String::from_utf8(vec![calibration_values.0, calibration_values.1]).unwrap();

        line_cal_values.push(sum.clone());

        //println!("{:?} {:?}", calibration_values, sum);

    }
    //println!("{:?}", line_cal_values);

    let mut sum = 0;
    for v in line_cal_values {
        sum += v.parse::<u64>().unwrap();
    }

    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
#[path = "_tests/p2.rs"]
mod tests;
