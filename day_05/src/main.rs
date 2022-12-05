use std::fs::File;
use std::io::{BufRead, BufReader};


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn initial_state() -> [Vec<char>; 9] {
    [
        vec!['H', 'L', 'R', 'F', 'B', 'C', 'J', 'M'],
        vec!['D', 'C', 'Z'],
        vec!['W', 'G', 'N', 'C', 'F', 'J', 'H'],
        vec!['B', 'S', 'T', 'M', 'D', 'J', 'P'],
        vec!['J', 'R', 'D', 'C', 'N'],
        vec!['Z', 'G', 'J', 'P', 'Q', 'D', 'L', 'W'],
        vec!['H', 'R', 'F', 'T', 'Z', 'P'],
        vec!['G', 'M', 'V', 'L'],
        vec!['J', 'R', 'Q', 'F', 'P', 'G', 'B', 'C']
    ]
}

fn part1() {
    let reader = read_file();
    let mut initial = initial_state();
    for v in initial.iter_mut() {
        v.reverse();
    }
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.starts_with("move") {
            continue;
        }
        let line_sp: Vec<usize> = line
            .split_whitespace()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse().unwrap()).collect();
        for _ in 0..line_sp[0] {
            let value = initial[line_sp[1]-1].pop().unwrap();
            initial[line_sp[2]-1].push(value);
        }
    }
    let mut result = String::new();
    for i in 0..initial.len() {
        result.push(initial[i].pop().unwrap());
    }
    println!("Part 1: {}", result);
}


fn part2() {
    let reader = read_file();
    let mut initial = initial_state();
    // let mut initial = initial_test();
    for v in initial.iter_mut() {
        v.reverse();
    }
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.starts_with("move") {
            continue;
        }
        let line_sp: Vec<usize> = line
            .split_whitespace()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse().unwrap()).collect();
        let len = initial[line_sp[1] - 1].len();
        let mut piece: Vec<char> = initial[line_sp[1]-1].drain(len-line_sp[0]..).collect();
        initial[line_sp[2]-1].append(&mut piece);
    }
    let mut result = String::new();
    for i in 0..initial.len() {
        result.push(initial[i].pop().unwrap());
    }
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
