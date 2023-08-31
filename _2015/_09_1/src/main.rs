use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    File::open("assets/09.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let input = input.split('\n').collect::<Vec<&str>>();

    let graph = format_graph(input);

    let mut data: Vec<(Vec<String>, u32)> = Vec::new();
    for label in graph.keys() {
        data.push(nnh(&graph, Some(label)));
    }
    // part 1
    let mut data: Vec<(Vec<String>, u32)> = Vec::new();
    for label in graph.keys() {
        data.push(prim(&graph, Some(label)));
    }
    let mut min: u32 = 1000;
    for path in data.iter() {
        println!("{:?} => {}", path.0, path.1);
        if path.1 < min {
            min = path.1;
        }
    }
    println!("{}", min);

    // part 2
    let mut data: Vec<(Vec<String>, u32)> = Vec::new();
    for label in graph.keys() {
        data.push(prim_inv(&graph, Some(label)));
    }
    let mut max: u32 = 0;
    for path in data.iter() {
        println!("{:?} => {}", path.0, path.1);
        if path.1 > max {
            max = path.1;
        }
    }
    println!("{}", max);

    
}

// prim s algorithm for minimum spanning trees
fn prim(graph: &HashMap<&str, Vec<(&str, u32)>>, start_node: Option<&str>) -> (Vec<String>, u32) {
    let mut node = start_node;
    let mut distance_travelled: u32 = 0;
    let mut path: Vec<String> = vec![String::from(start_node.unwrap())];
    'run: loop {
        match node {
            Some(k) => {
                let edges = graph.get(k);
                let mut min_node = "";
                let mut min_dist: u32 = 1000;
                if let Some(edges) = edges {
                    for edge in edges {
                        if path.contains(&String::from(edge.0)) { continue }
                        if edge.1 < min_dist {
                            min_dist = edge.1;
                            min_node = edge.0;
                        }
                    }
                }
                path.push(String::from(min_node));
                distance_travelled += min_dist;
                node = if min_node == "" { None } else { Some(min_node) };
            }
            None => break 'run,
        }
    }
    (path[0..path.len() - 1].to_vec(), distance_travelled - 1000)
}

fn prim_inv(graph: &HashMap<&str, Vec<(&str, u32)>>, start_node: Option<&str>) -> (Vec<String>, u32) {
    let mut node = start_node;
    let mut distance_travelled: u32 = 0;
    let mut path: Vec<String> = vec![String::from(start_node.unwrap())];
    'run: loop {
        match node {
            Some(k) => {
                let edges = graph.get(k);
                let mut max_node = "";
                let mut max_dist: u32 = 0;
                if let Some(edges) = edges {
                    for edge in edges {
                        if path.contains(&String::from(edge.0)) { continue }
                        if edge.1 > max_dist {
                            max_dist = edge.1;
                            max_node = edge.0;
                        }
                    }
                }
                path.push(String::from(max_node));
                distance_travelled += max_dist;
                node = if max_node == "" { None } else { Some(max_node) };
            }
            None => break 'run,
        }
    }
    (path[0..path.len() - 1].to_vec(), distance_travelled)
}
fn format_graph(text: Vec<&str>) -> HashMap<&str, Vec<(&str, u32)>> {
    let mut nodes: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
    for line in text[0..text.len() - 1].into_iter() {
        let line = line.split(' ').collect::<Vec<&str>>();
        let (origin, destination, distance): (&str, &str, u32) =
            (line[0], line[2], line[4].trim().parse().unwrap());
        //println!("{} {} {}", origin, destination, distance);
        if nodes.contains_key(origin) {
            nodes
                .get_mut(origin)
                .map(|val| val.push((destination, distance)));
        } else {
            nodes.insert(origin, vec![(destination, distance)]);
        }
        if nodes.contains_key(destination) {
            nodes
                .get_mut(destination)
                .map(|val| val.push((origin, distance)));
        } else {
            nodes.insert(destination, vec![(origin, distance)]);
        }
    }
    nodes
}
fn format_graph_directed(text: Vec<&str>) -> HashMap<&str, Vec<(&str, u32)>> {
    let mut nodes: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();

    for line in text[0..text.len() - 1].into_iter() {
        let line = line.split(' ').collect::<Vec<&str>>();
        let (origin, destination, distance): (&str, &str, u32) =
            (line[0], line[2], line[4].trim().parse().unwrap());
        //println!("{} {} {}", origin, destination, distance);
        if nodes.contains_key(origin) {
            nodes
                .get_mut(origin)
                .map(|val| val.push((destination, distance)));
        } else {
            nodes.insert(origin, vec![(destination, distance)]);
        }
    }
    nodes
}
