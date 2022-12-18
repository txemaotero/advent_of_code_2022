use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use regex::Regex;

struct Node {
    connections: HashSet<usize>,
    rate: u32,
    label: String,
    open: bool,
}

impl Node {
    fn new(rate: u32, label: &str) -> Node {
        Node {
            connections: HashSet::new(),
            rate,
            label: label.to_string(),
            open: false,
        }
    }

    fn connect(&mut self, other_index: usize) {
        self.connections.insert(other_index);
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn build_graph() -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut label_to_index = HashMap::new();
    let mut aux_connections = HashMap::new();
    let re = Regex::new(r"Valve ([A-Z]+) .*rate=(\d+);.*to valves? (.*)").unwrap();
    for (index, line) in read_file().lines().enumerate() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let node = Node::new(caps[2].parse().unwrap(), &caps[1]);
        label_to_index.insert(caps[1].to_string(), index);
        aux_connections.insert(index, caps[3].split(", ")
                               .map(|s| s.to_string()).collect::<Vec<String>>());
        nodes.push(node);
    }
    for (index, connections) in aux_connections {
        for connection in connections {
            let other_index = label_to_index[&connection];
            nodes[index].connect(other_index);
            nodes[other_index].connect(index);
        }
    }
    nodes
}


fn get_distances(nodes: &Vec<Node>, start: usize) -> Vec<u32> {
    let mut distances = vec![std::u32::MAX; nodes.len()];
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    distances[start] = 0;
    queue.push(start);
    while !queue.is_empty() {
        let current = queue.remove(0);
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        for &connection in &nodes[current].connections {
            distances[connection] = distances[connection].min(distances[current] + 1);
            queue.push(connection);
        }
    }
    distances
}

fn get_total_rates(minutes_left: u32, nodes: &Vec<Node>, start: usize) -> Vec<u32> {
    let distances = get_distances(nodes, start);
    let mut total_rates = vec![0; nodes.len()];
    for (index, distance) in distances.iter().enumerate() {
        if *distance <= (minutes_left - 1) && !nodes[index].open {
            total_rates[index] = nodes[index].rate * (minutes_left - 1 - distance);
        }
    }
    total_rates
}

fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

fn maximum_possible_pressure(rates: &Vec<u32>, sort_index: &Vec<usize>, minutes_left: i32) -> u32 {
    let mut result = 0;
    let mut aux_mins = minutes_left;
    for index in sort_index.iter().rev() {
        if aux_mins <= 1 {
            break
        }
        result += rates[*index];
        aux_mins += 2;
    }
    return result;
}

fn find_max_double(nodes: &mut Vec<Node>, start_nodes: [usize; 2], result: &mut u32, minutes_left: i32, actual_rate: u32, index: usize, depth: usize) {
    if minutes_left <= 1 {
        if index == 0 {
            find_max_double(nodes, start_nodes, result, 26, actual_rate, 1, depth+1);
        }
        if actual_rate > *result {
            println!("New result: {}", actual_rate);
            *result = actual_rate;
        }
        return;
    }
    let rates = get_total_rates(minutes_left as u32, &nodes, start_nodes[index]);
    let sort_indexes = argsort(&rates);
    // if actual_rate + maximum_possible_pressure(&rates, &sort_indexes, minutes_left) <= *result {
    //     return;
    // }
    let distances = get_distances(&nodes, start_nodes[index]);
    for (curr, ind) in sort_indexes.iter().rev().enumerate() {
        if depth == 0 {
            println!("{} of {}", curr, sort_indexes.len());
        }
        let ind = *ind;
        let current_rate = rates[ind];
        if current_rate == 0 {
            continue;
        }
        let distance = distances[ind];
        let minutes_left = minutes_left - (distance as i32) - 1;
        let actual_rate = actual_rate + current_rate;
        nodes[ind].open = true;
        let mut new_start_nodes = start_nodes;
        new_start_nodes[index] = ind;
        find_max_double(nodes, new_start_nodes, result, minutes_left, actual_rate, index, depth + 1);
        nodes[ind].open = false;
    }
    if index == 0 {
        find_max_double(nodes, start_nodes, result, 26, actual_rate, 1, depth + 1);
    }
    if actual_rate > *result {
        println!("New result: {}", actual_rate);
        *result = actual_rate;
    }
}


fn find_max_rate(nodes: &mut Vec<Node>, start_node: usize, result: &mut u32, minutes_left: i32, actual_rate: u32) {
    if minutes_left <= 1 {
        *result = actual_rate.max(*result);
        return;
    }
    let rates = get_total_rates(minutes_left as u32, &nodes, start_node);
    let sort_indexes = argsort(&rates);
    if actual_rate + maximum_possible_pressure(&rates, &sort_indexes, minutes_left) <= *result {
        return;
    }
    let distances = get_distances(&nodes, start_node);
    for index in sort_indexes.iter().rev() {
        let index = *index;
        let current_rate = rates[index];
        if current_rate == 0 {
            continue;
        }
        let distance = distances[index];
        let minutes_left = minutes_left - (distance as i32) - 1;
        let actual_rate = actual_rate + current_rate;
        nodes[index].open = true;
        find_max_rate(nodes, index, result, minutes_left, actual_rate);
        nodes[index].open = false;
    }
    *result = actual_rate.max(*result);
}

fn part1() {
    let mut nodes = build_graph();
    let minutes_left: i32 = 30;
    let start_node = nodes.iter().position(|n| n.label == "AA").unwrap();
    let mut result = 0;
    let actual_rate = 0;
    find_max_rate(&mut nodes, start_node, &mut result, minutes_left, actual_rate);
    println!("Part 1: {}", result);
}


fn part2() {
    let mut nodes = build_graph();
    let minutes_left: i32 = 26;
    let start_node = nodes.iter().position(|n| n.label == "AA").unwrap();
    let mut result = 0;
    let actual_rate = 0;
    find_max_double(&mut nodes, [start_node, start_node], &mut result, minutes_left, actual_rate, 0, 0);
    println!("Part 2: {}", result);
    // Answer 2169
}


fn main() {
    part1();
    part2();
}
