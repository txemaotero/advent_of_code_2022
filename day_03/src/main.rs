use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn get_letter_value(c: char) -> u32 {
    let letter_values: HashMap<char, u32> = HashMap::from([
        ('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5), ('f', 6), ('g', 7), ('h', 8), ('i', 9),
        ('j', 10), ('k', 11), ('l', 12), ('m', 13), ('n', 14), ('o', 15), ('p', 16), ('q', 17),
        ('r', 18), ('s', 19), ('t', 20), ('u', 21), ('v', 22), ('w', 23), ('x', 24), ('y', 25),
        ('z', 26), ('A', 27), ('B', 28), ('C', 29), ('D', 30), ('E', 31), ('F', 32), ('G', 33),
        ('H', 34), ('I', 35), ('J', 36), ('K', 37), ('L', 38), ('M', 39), ('N', 40), ('O', 41),
        ('P', 42), ('Q', 43), ('R', 44), ('S', 45), ('T', 46), ('U', 47), ('V', 48), ('W', 49),
        ('X', 50), ('Y', 51), ('Z', 52),
    ]);
    *letter_values.get(&c).unwrap()

}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut tracker: HashMap<char, [usize; 2]> = HashMap::new();
        let mut n_elements: usize = 0;
        for (i, c) in line.chars().enumerate() {
            let indexes = tracker.entry(c).or_insert([i, i]);
            indexes[1] = i;
            n_elements += 1;
        }
        result += get_letter_value(get_repeated(tracker, n_elements));
    }
    println!("Part 1: {}", result);
}


fn get_repeated(chars_index: HashMap<char, [usize; 2]>, size: usize) -> char {
    let h_size = size / 2;
    for (k, v) in chars_index.iter() {
        if v[0] < h_size && v[1] >= h_size {
            return *k;
        }
    }
    panic!("BAD");
}

fn part2() {
    let reader = read_file();
    let mut result = 0;
    let mut tracker: HashMap<char, [bool; 3]> = HashMap::new();
    let mut aux_index = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.chars() {
            let indexes = tracker.entry(c).or_insert([false, false, false]);
            indexes[aux_index] = true;
        }
        aux_index += 1;
        if aux_index == 3 {
            result += get_letter_value(get_repeated_lines(&tracker));
            aux_index = 0;
            tracker.clear();
        }
    }
    println!("Part 2: {}", result);
}


fn get_repeated_lines(char_appears: &HashMap<char, [bool; 3]>) -> char {
    for (k, v) in char_appears.iter() {
        if v.iter().all(|&x| x) {
            return *k;
        }
    }
    panic!("BAD");
}


fn main() {
    part1();
    part2();
}
