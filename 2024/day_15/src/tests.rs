use super::*;

#[test]
fn simple_01() {
    let input = std::fs::read_to_string("./src/tests/simple.txt").unwrap();
    let (pos, mut warehouse, mov_list) = parse_input(&input);

    assert!(task_01(pos, &mut warehouse, &mov_list) == 2028)
}

#[test]
fn only_up() {
    let input = std::fs::read_to_string("./src/tests/only_up.txt").unwrap();
    let (fish, mut map, mov) = parse_input(&input);

    for row in map.iter() {
        println!("{:?}", row);
    }

    let _ = task_01(fish, &mut map, &mov);

    println!("{:?}\n==\n{:?}", fish, mov);
}
