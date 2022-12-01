use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    let mut aux_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty(){
            result = std::cmp::max(result, aux_sum);
            aux_sum = 0;
        } else {
            aux_sum += line.parse::<i32>().unwrap();
        }
    }
    result = std::cmp::max(result, aux_sum);
    println!("Part 1: {}", result);
}


fn part2() {
    let reader = read_file();
    let mut ranking = BinaryHeap::new();
    let mut aux_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty(){
            ranking.push(aux_sum);
            aux_sum = 0;
        } else {
            aux_sum += line.parse::<i32>().unwrap();
        }
    }
    ranking.push(aux_sum);
    let mut counter = 0;
    let mut result = 0;
    loop {
        result += ranking.pop().unwrap();
        counter += 1;
        if counter == 3 {
            break
        }
    };
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
