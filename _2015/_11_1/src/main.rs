
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let input = "vzbxkghb";
    //let input = "abcdefgh";
    let mut password = input.bytes().collect::<Vec<u8>>();
    password.reverse();
    let ans = force_abc(&mut password.clone());
    println!("{}", input);
    println!("{:?} {:?}", password, vec_u8_to_str(&password));
    println!("{:?} {:?}", ans, vec_u8_to_str(&ans));

}

fn vec_u8_to_str(v: &Vec<u8>) -> String {
    let mut s = String::new();
    for b in v.into_iter() {
        s += std::str::from_utf8(&[*b]).unwrap();
    }
    s
}
fn force_abc(password: &mut Vec<u8>) -> Vec<u8> {
    if check_abc(password).is_some() {
        return password.to_vec();
    };

    let itr = 0..3;   

    // find max among first 3 letters
    let mut max: (u8, usize)  = (0, 0);
    for i in itr {
        let b = password[i as usize];
        if b > max.0 {
            max = (b, i as usize);
        }
    }
    // append to form abc
    match max.1 {
        0 => {
            password[1] = (password[0] - 1);
            password[2] = (password[0] - 2);
        },
        1 => {
            password[0] = (password[1] + 1);
            password[2] = (password[1] - 1);
        },
        2 => {
            password[0] = (password[2] + 2);
            password[1] = (password[2] + 1);
        },
        _ => panic!("!!!!!!"),
    }
     password.to_vec()
}

fn padded_increment(password: &mut Vec<u8>) -> Vec<u8> {
    'run: loop {
        let has_abc = check_abc(password);

        match has_abc {
            Some(i) => {
            },
            None => {},
        }
    }
}

fn check_abc(password: &Vec<u8>) -> Option<usize> {
    let mut buffer: (u8, u8) = (password[0], password[1]);
    let len = password.len();
    for i in 2..len {
        let byte = password[i];
        if buffer.0 == byte - 2 && buffer.1 == byte - 1 {
            return Some(i - 2);
        }
        buffer.0 = buffer.1;
        buffer.1 = byte;
    }
    return None
}

fn brute_force(mut password: Vec<u8>) -> String {
    let password = &mut password[..];
    // make little indian
    password.reverse();
    'run: loop {
        // check for overlaps
        for i in 0..password.len()-1 {
            if password[i] > b'z' {
                password[i] = b'a';
                pass_byte_add(&mut password[i + 1], 1);
            }
        }
        let mut byte_buffer: (u8, u8) = (0, 1);
        let mut has_abc = false;
        let mut double_pair: u8 = 0;
        let mut overlap_counter: u8 = 0;
        for b in password.iter_mut() {
            // check consecutive *bits
            if !has_abc {
                if byte_buffer.0 == *b + 2 && byte_buffer.1 == *b + 1 {
                    has_abc = true;
                }
            }
            // check double pair
            if double_pair < 2 {
                if byte_buffer.0 == byte_buffer.1 && overlap_counter == 0{
                    double_pair += 1;
                    overlap_counter = 2;
                }
                if overlap_counter > 0 {
                    overlap_counter -= 1;
                }
            }
            //println!("{} {} {} {:?}", b, has_abc, double_pair, byte_buffer);
            byte_buffer.0 = byte_buffer.1;
            byte_buffer.1 = *b;
            if has_abc && double_pair >= 2 {
                break 'run
            }
        }
        pass_byte_add(&mut password[0], 1);
        println!("{:?}", password);
        //sleep 
        sleep(Duration::from_millis(25));
    }
    password.reverse();
    println!("{:?}", password);
    for p in password {
        print!("{}", *p as char);
    }
    println!();
    "".to_string()
}

fn pass_byte_add(b: &mut u8, i: u8) {
    for _ in 0..i {
        match b {
            // skip i o l
            b'h' | b'n' | b'k' => *b += 2,
            b'a'..=b'z' => *b += 1,
            _ => (),
        }
    }
}
fn pass_byte_sub(b: &mut u8, i: u8) {
    for _ in 0..i {
        match b {
            // skip i o l
            b'j' | b'p' | b'm' => *b -= 2,
            b'a'..=b'z' => *b -= 1,
            _ => (),
        }
    }
}
