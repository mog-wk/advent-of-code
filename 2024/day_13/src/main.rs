fn main() {
    let mut f = std::env::args();
    f.next();
    let x = f.next().unwrap();
    let y = f.next().unwrap();

    //let input = std::fs::read_to_string(f).unwrap();

    let g = extended_gcd(x.parse().unwrap(), y.parse().unwrap());
    println!("{:?}", g);
}

fn extended_gcd(a: i32, b: i32) -> Option<(i32, i32, i32)> {
    if a == 0 {
        return Some((b, 1, 0));
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a).unwrap();
    let x = y1 - (b / a) * x1;
    let y = x1;

    return Some((gcd, x, y));
}
