use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn snafaru_to_decimal(snafaru: Vec<char>) -> isize {
    snafaru
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut result = (5 as isize).pow(i as u32);
            match c {
                '=' => result *= -2,
                '-' => result *= -1,
                '0' => result *= 0,
                '1' => result *= 1,
                '2' => result *= 2,
                _ => panic!("Invalid character"),
            }
            return result;
        })
        .sum()
}

fn decimal_to_snafaru(mut number: isize) -> Vec<char> {
    // lets find max digit. Note that there can be only 1 and 2 at the beginning
    let mut max_digit = 0;
    let mut accum = 0;
    while accum/2 < number {
        max_digit += 1;
        accum += (5 as isize).pow(max_digit as u32);
    }
    let mut result = Vec::new();
    println!("Max digit: {}", max_digit);
    let mut shifted_number = number + (5 as isize).pow(max_digit as u32)/2;
    while max_digit > 0 {
        max_digit -= 1;
        let digit = shifted_number / (5 as isize).pow(max_digit as u32);
        match digit {
            0 => result.push('='),
            1 => result.push('-'),
            2 => result.push('0'),
            3 => result.push('1'),
            4 => result.push('2'),
            _ => panic!("Invalid digit {}", digit),
        };
        shifted_number -= digit * (5 as isize).pow(max_digit as u32);
    }
    result
}

fn part1() {
    let reader = read_file();
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        result += snafaru_to_decimal(line.chars().collect());
    }
    println!("Part 1: {}", decimal_to_snafaru(result).iter().collect::<String>());
}

fn main() {
    part1();
}
