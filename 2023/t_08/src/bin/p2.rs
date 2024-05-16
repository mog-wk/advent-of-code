#![allow(unused)]

use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    sync::{atomic::AtomicU32, Arc, Mutex},
    thread::spawn,
};

use itertools::Itertools;
use std::fs::canonicalize;
use std::path::PathBuf;
use tracing::{debug, field::debug, info, Level};
use tracing_subscriber::filter::targets;

// does not work
fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let input_file = canonicalize(PathBuf::from("./input/input.txt")).unwrap();
    let x = parse(input_file.to_str().unwrap()).unwrap();
    let x = process(x.0, x.1, x.2 .0, x.2 .1);
    println!("{:?}", x);
}

fn parse(
    path: &str,
) -> anyhow::Result<(
    String,
    HashMap<String, (String, String)>,
    (Vec<String>, Vec<String>),
)> {
    info!("parsing input...");
    let mut file = File::open(path)?;
    let mut raw = String::new();
    file.read_to_string(&mut raw);

    let mut starts = Vec::with_capacity(4);
    let mut destinations = Vec::with_capacity(4);

    let (directions, nodes) = raw.split_once('\n').unwrap();
    let nodes = &nodes.lines().collect_vec()[1..];
    let nodes: HashMap<String, (String, String)> = nodes
        .into_iter()
        .map(|node| {
            let (mut source, dest) = node.split_once('=').unwrap();
            let source = source.strip_suffix(" ").unwrap();

            if source.ends_with('A') {
                starts.push(source.to_string());
            } else if source.ends_with('Z') {
                destinations.push(source.to_string());
            }

            let ln = dest.len() - 1;
            let mut dest = dest[2..ln].split_once(',').unwrap();
            dest.1 = dest.1.strip_prefix(" ").unwrap();

            (
                source.into(),
                (dest.0.to_string(), dest.1.to_string()).into(),
            )
        })
        .collect();
    debug!(
        "{:?} {:?} {:?} {:?}",
        directions, nodes, starts, destinations
    );

    Ok((
        directions.to_string(),
        nodes.to_owned(),
        (starts, destinations),
    ))
}

fn process(
    directions: String,
    map: HashMap<String, (String, String)>,
    starts: Vec<String>,
    targets: Vec<String>,
) -> u32 {
    info!("beginning process...");
    let directions_len = directions.len();

    let mut count = 0;
    let mut i = 0;

    let mut cur_nodes = Vec::clone(&starts);

    // do until all nodes have 'Z' ( are in their destination)
    'outer: loop {
        // iterator throught all nodes
        for (j, start) in starts.iter().enumerate() {
            let cur_direction = directions
                .get(i..i + 1)
                .unwrap()
                .bytes()
                .collect::<Vec<u8>>()[0];

            debug!(
                "from {:?} of start: {:?} dir: {:?} i: {}",
                cur_nodes[j], start, cur_direction as char, i
            );

            let paths = map.get(&cur_nodes[j]).unwrap();

            match cur_direction {
                b'L' => {
                    cur_nodes[j] = paths.0.clone();
                }
                b'R' => {
                    cur_nodes[j] = paths.1.clone();
                }
                _ => unreachable!(),
            }
        }
        if i >= directions_len - 1 {
            i = 0;
        } else {
            i += 1;
        }

        debug!("END OF LOOP: nodes: {:?} count: {:?}", cur_nodes, count + 1);

        // check if nodes are all in the end of path
        let filter = cur_nodes.iter().filter(|n| n.contains('Z'));

        if filter.collect::<Vec<_>>().len() == starts.len() {
            return count + 1;
        } else {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
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
        let x = process(x.0, x.1, x.2 .0, x.2 .1);

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
        let x = process(x.0, x.1, x.2 .0, x.2 .1);

        assert_eq!(x, 2);
    }

    #[test]
    fn conc_simple() {
        use std::fs::canonicalize;
        use std::path::PathBuf;

        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        let test_file = canonicalize(PathBuf::from("./src/input/test_conc_simple.txt")).unwrap();

        let x = parse(test_file.to_str().unwrap()).unwrap();
        let x = process(x.0, x.1, x.2 .0, x.2 .1);

        assert_eq!(x, 6);
    }
}
