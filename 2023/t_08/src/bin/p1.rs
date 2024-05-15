#![allow(unused)]

use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;
use std::fs::canonicalize;
use std::path::PathBuf;
use tracing::{debug, info, Level};

fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let input_file = canonicalize(PathBuf::from("./input/input.txt")).unwrap();
    let x = parse(input_file.to_str().unwrap()).unwrap();
    let x = process(&x.0, &x.1, "AAA", "ZZZ");
    println!("{:?}", x);
}

fn parse(path: &str) -> anyhow::Result<(String, HashMap<String, (String, String)>)> {
    info!("parsing input...");
    let mut file = File::open(path)?;
    let mut raw = String::new();
    file.read_to_string(&mut raw);

    let (directions, nodes) = raw.split_once('\n').unwrap();
    let nodes = &nodes.lines().collect_vec()[1..];
    let nodes: HashMap<String, (String, String)> = nodes
        .into_iter()
        .map(|node| {
            let (mut source, dest) = node.split_once('=').unwrap();
            let source = source.strip_suffix(" ").unwrap();
            let ln = dest.len() - 1;
            let mut dest = dest[2..ln].split_once(',').unwrap();
            dest.1 = dest.1.strip_prefix(" ").unwrap();

            (
                source.into(),
                (dest.0.to_string(), dest.1.to_string()).into(),
            )
        })
        .collect();
    debug!("{:?} {:?}", directions, nodes);

    Ok((directions.to_string(), nodes.to_owned()))
}

fn process(
    directions: &str,
    map: &HashMap<String, (String, String)>,
    start: &str,
    target: &str,
) -> u32 {
    let directions_len = directions.len();
    let mut i = 0;
    let mut count = 0;

    let mut cur_node = start.to_string();

    loop {
        if &cur_node == target {
            return count;
        }
        count += 1;

        let cur_direction = directions
            .get(i..i + 1)
            .unwrap()
            .bytes()
            .collect::<Vec<u8>>()[0];

        let paths = map.get(&cur_node).unwrap();

        match cur_direction {
            b'L' => {
                cur_node = paths.0.clone();
            }
            b'R' => {
                cur_node = paths.1.clone();
            }
            _ => unreachable!(),
        }

        if i >= directions_len - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use tracing::Level;
    use tracing_subscriber::util::SubscriberInitExt;

    use crate::process;

    use super::parse;

    #[test]
    fn fast() {
        use std::fs::canonicalize;
        use std::path::PathBuf;

        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        let test_file = canonicalize(PathBuf::from("./src/input/test_fast.txt")).unwrap();

        let x = parse(test_file.to_str().unwrap()).unwrap();
        let x = process(&x.0, &x.1, "AAA", "ZZZ");

        assert_eq!(x, 6);
    }

    #[test]
    fn medium() {
        use std::fs::canonicalize;
        use std::path::PathBuf;

        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        let test_file = canonicalize(PathBuf::from("./src/input/test.txt")).unwrap();

        let x = parse(test_file.to_str().unwrap()).unwrap();
        let x = process(&x.0, &x.1, "AAA", "ZZZ");

        assert_eq!(x, 2);
    }
}
