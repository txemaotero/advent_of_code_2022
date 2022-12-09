use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    let mut result: HashSet<[i32; 2]> = HashSet::new();
    result.insert([0, 0]);

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, ammount) = line.split_once(" ").unwrap();
        let ammount: u32 = ammount.parse().unwrap();
        let index = if (direction == "L") | (direction == "R") {
            0
        } else {
            1
        };
        let magnitude = if (direction == "U") | (direction == "R") {
            1
        } else {
            -1
        };
        for _ in 0..ammount {
            head[index] += magnitude;
            tail = move_tail(&head, tail);
            result.insert(tail);
        }
    }

    println!("Part 1: {}", result.len());
}

fn move_tail(head: &[i32; 2], mut tail: [i32; 2]) -> [i32; 2] {
    let diff_x = head[0] - tail[0];
    let diff_y = head[1] - tail[1];
    if (diff_x.abs() <= 1) & (diff_y.abs() <= 1) {
        return tail;
    }
    tail[0] += diff_x.signum();
    tail[1] += diff_y.signum();
    return tail;
}

fn part2() {
    let reader = read_file();

    let mut knots: [[i32; 2]; 10] = [[0, 0]; 10];

    let mut result: HashSet<[i32; 2]> = HashSet::new();
    result.insert([0, 0]);

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, ammount) = line.split_once(" ").unwrap();
        let ammount: u32 = ammount.parse().unwrap();
        let index = if (direction == "L") | (direction == "R") {
            0
        } else {
            1
        };
        let magnitude = if (direction == "U") | (direction == "R") {
            1
        } else {
            -1
        };
        for _ in 0..ammount {
            knots[0][index] += magnitude;
            for i in 1..10 {
                knots[i] = move_tail(&knots[i-1], knots[i]);
            }
            result.insert(knots[9]);
        }
    }

    println!("Part 2: {}", result.len());
}


fn main() {
    part1();
    part2();
}
