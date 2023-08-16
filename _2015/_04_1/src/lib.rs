/// implementation of md5 hash


/// DISCLAIMER: MD5 is considered cryptographcally
/// broken as it is vulnerable to collision attacks.
/// DO NOT use in digital signatures 
//links: 
// https://www.kb.cert.org/vuls/id/836068
// https://dl.acm.org/doi/10.1109/word_cIS.2009.214
// https://datatracker.ietf.org/doc/html/rfc6151
// https://dev.to/mdimovich/build-md5-from-scratch-with-rust-563e
// https://github.com/mdimovich/rusty_md5/blob/main/src/lib.rs#L116
pub fn md5(text: &str) -> String {
    let bench = std::time::Instant::now();

    let text_vec: Vec<u8> = text.bytes().collect();
    // 1. define padding bits
    let text_len = (text_vec.len() as u64) << 3;
    let mut pad_to = (text_len as i32) * -1;

    while pad_to < 64 {
        pad_to += 512;
    }
    pad_to -= 64;
    pad_to >>= 3;

    let mut bits = vec![];
    bits.extend(text_vec);
    bits.push(0b1000_0000);
    for _i in 1..pad_to {
        bits.push(0b0000_0000);
    }
    bits.extend(split_u64_to_u8_array(text_len));
    //println!("{:?} {}", bits, text_len);

    // 3. initialize Mword_d buffer
    let mut word_a: u32 = 0x67452301u32;
    let mut word_b: u32 = 0xefcdab89u32;
    let mut word_c: u32 = 0x98badcfeu32;
    let mut word_d: u32 = 0x10325476u32;


    let f = |x: u32, y: u32, z: u32| (x&y) | (!x & z);
    let g = |x: u32, y: u32, z: u32| (x&z) | (!z & y);
    let h = |x: u32, y: u32, z: u32| x ^ y ^ z;
    let i = |x: u32, y: u32, z: u32| y ^ (x | !z);

    let table = get_sine_table();

    for chunk in bits.chunks_exact_mut(64) {

        let word_aa = word_a;
        let word_bb = word_b;
        let word_cc = word_c;
        let word_dd = word_d;

        let x = convert_vec_u8_to_vec_u32(chunk);

        // round 1
        let result = round_one_operations(word_a, word_b, word_c, word_d, &table, &x, Box::new(f));
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        let result = round_two_operations(word_a, word_b, word_c, word_d, &table, &x, Box::new(g));
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        let result = round_three_operations(word_a, word_b, word_c, word_d, &table, &x, Box::new(h));
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        let result = round_four_operations(word_a, word_b, word_c, word_d, &table, &x, Box::new(i));
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        word_a = word_a.wrapping_add(word_aa);
        word_b = word_b.wrapping_add(word_bb);
        word_c = word_c.wrapping_add(word_cc);
        word_d = word_d.wrapping_add(word_dd);
    }
    //println!("{:.2?}", bench.elapsed());
    format!("{:08x}{:08x}{:08x}{:08x}", word_a.swap_bytes(), word_b.swap_bytes(), word_c.swap_bytes(), word_d.swap_bytes())
}
fn bit_padding(input: &str) -> Vec<u8> {
    let mut input_vector: Vec<u8> = input.bytes().collect();
    let bit_length: u64 = (input.len() as u64) * 8u64; // todo - add support for > 2^64 bit size

    // 128_u8 is the equivalent of padding 1 as an unsigned 8-bit integer
    // with lower-order bits first
    input_vector.push(128_u8);
    //check if bit length % 512 is 448 (64 less than 512)
    while (input_vector.len() * 8) % 512 != 448 {
        input_vector.push(0_u8); // push in another 8-bit 0 padded value until the correct
                                 // result is reached;
    }

    let length_bits_as_u8_array = split_u64_to_u8_array(bit_length);
    input_vector.extend(length_bits_as_u8_array);

    return input_vector;
}
fn split_u64_to_u8_array(s: u64) -> [u8; 8] {
    let u8_array = [
        s as u8,
        (s >> 8) as u8,
        (s >> 16) as u8,
        (s >> 24) as u8,
        (s >> 32) as u8,
        (s >> 40) as u8,
        (s >> 48) as u8,
        (s >> 56) as u8,
    ];
    return u8_array;
}
fn convert_vec_u8_to_vec_u32(v: &mut [u8]) -> Vec<u32> {
    let mut x: Vec<u32> = Vec::new();

    for i in 0..v.len() {
        let c = i as u32 % 4;
        if c == 3 {
            let arr: [u8; 4] = [v[i-3], v[i-2], v[i-1], v[i]]; 
            x.push(u32::from_ne_bytes(arr));
        }
    }
    x
}

fn round_one_operations(mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, x: &Vec<u32>, f: Box<fn(u32, u32, u32) -> u32>) -> [u32; 4] {
    macro_rules! round1 {
        ($a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(f($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round1!(a, b, c, d, 0, 7, 1);
    round1!(d, a, b, c, 1, 12, 2);
    round1!(c, d, a, b, 2, 17, 3);
    round1!(b, c, d, a, 3, 22, 4);

    round1!(a, b, c, d, 4, 7, 5);
    round1!(d, a, b, c, 5, 12, 6);
    round1!(c, d, a, b, 6, 17, 7);
    round1!(b, c, d, a, 7, 22, 8);

    round1!(a, b, c, d, 8, 7, 9);
    round1!(d, a, b, c, 9, 12, 10);
    round1!(c, d, a, b, 10, 17, 11);
    round1!(b, c, d, a, 11, 22, 12);

    round1!(a, b, c, d, 12, 7, 13);
    round1!(d, a, b, c, 13, 12, 14);
    round1!(c, d, a, b, 14, 17, 15);
    round1!(b, c, d, a, 15, 22, 16);

    [a, b, c, d]
}
fn round_two_operations(mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, x: &Vec<u32>, f: Box<fn(u32, u32, u32) -> u32>) -> [u32; 4] {
    macro_rules! round2 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(f($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round2!(a, b, c, d, 1, 5, 17);
    round2!(d, a, b, c, 6, 9, 18);
    round2!(c, d, a, b, 11, 14, 19);
    round2!(b, c, d, a, 0, 20, 20);

    round2!(a, b, c, d, 5, 5, 21);
    round2!(d, a, b, c, 10, 9, 22);
    round2!(c, d, a, b, 15, 14, 23);
    round2!(b, c, d, a, 4, 20, 24);

    round2!(a, b, c, d, 9, 5, 25);
    round2!(d, a, b, c, 14, 9, 26);
    round2!(c, d, a, b, 3, 14, 27);
    round2!(b, c, d, a, 8, 20, 28);

    round2!(a, b, c, d, 13, 5, 29);
    round2!(d, a, b, c, 2, 9, 30);
    round2!(c, d, a, b, 7, 14, 31);
    round2!(b, c, d, a, 12, 20, 32);

    return [a, b, c, d];
}
fn round_three_operations(mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, x: &Vec<u32>, f: Box<fn(u32, u32, u32) -> u32>) -> [u32; 4] {
    macro_rules! round3 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr  ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(f($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round3!(a, b, c, d, 5, 4, 33);
    round3!(d, a, b, c, 8, 11, 34);
    round3!(c, d, a, b, 11, 16, 35);
    round3!(b, c, d, a, 14, 23, 36);

    round3!(a, b, c, d, 1, 4, 37);
    round3!(d, a, b, c, 4, 11, 38);
    round3!(c, d, a, b, 7, 16, 39);
    round3!(b, c, d, a, 10, 23, 40);

    round3!(a, b, c, d, 13, 4, 41);
    round3!(d, a, b, c, 0, 11, 42);
    round3!(c, d, a, b, 3, 16, 43);
    round3!(b, c, d, a, 6, 23, 44);

    round3!(a, b, c, d, 9, 4, 45);
    round3!(d, a, b, c, 12, 11, 46);
    round3!(c, d, a, b, 15, 16, 47);
    round3!(b, c, d, a, 2, 23, 48);

    return [a, b, c, d];
}
fn round_four_operations(mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, x: &Vec<u32>, f: Box<fn(u32, u32, u32) -> u32>) -> [u32; 4] {
    macro_rules! round4 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(f($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round4!(a, b, c, d, 0, 6, 49);
    round4!(d, a, b, c, 7, 10, 50);
    round4!(c, d, a, b, 14, 15, 51);
    round4!(b, c, d, a, 5, 21, 52);

    round4!(a, b, c, d, 12, 6, 53);
    round4!(d, a, b, c, 3, 10, 54);
    round4!(c, d, a, b, 10, 15, 55);
    round4!(b, c, d, a, 1, 21, 56);

    round4!(a, b, c, d, 8, 6, 57);
    round4!(d, a, b, c, 15, 10, 58);
    round4!(c, d, a, b, 6, 15, 59);
    round4!(b, c, d, a, 13, 21, 60);

    round4!(a, b, c, d, 4, 6, 61);
    round4!(d, a, b, c, 11, 10, 62);
    round4!(c, d, a, b, 2, 15, 63);
    round4!(b, c, d, a, 9, 21, 64);

    return [a, b, c, d];
}
fn get_sine_table() -> Vec<u32> {
    let mut t: Vec<u32> = Vec::new();
    t.push(0x0000_0000);

    for i in 1..=64 {
        t.push(get_sine_eval(i))
    }
    t
}

fn get_sine_eval(i: u32) -> u32 {
    let x = i as f64;
    let sin_eval = x.sin().abs();

    // 2^32 * abs sine of i
    ( 4294967296.0 * sin_eval ) as u32
}
