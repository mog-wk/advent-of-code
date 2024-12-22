use super::*;

#[test]
fn default_01() {
    let input = std::fs::read_to_string("src/tests/default.txt").unwrap();

    let mut robots = parse_input(&input);

    //for r in robots.iter() { println!("{:?}", r); }

    let aws = task_01(&mut robots, 100, 11, 7);

    println!("====");
    //for r in robots.iter() { println!("{:?}", r); }
    assert_eq!(aws, 12);
}

#[test]
fn simple_01() -> () {
    let input = std::fs::read_to_string("src/tests/simple.txt").unwrap();

    let mut robots = parse_input(&input);

    for r in robots.iter() {
        println!("{:?}", r);
    }

    task_01(&mut robots, 5, 11, 7);

    println!("====");
    for r in robots.iter() {
        println!("{:?}", r);
    }
}
