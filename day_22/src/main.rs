use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;


const NCOLS: usize = 150;


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    println!("Part 1: {}", result);
}


fn part2() {
    let reader = read_file();
    let mut result = 0;
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
