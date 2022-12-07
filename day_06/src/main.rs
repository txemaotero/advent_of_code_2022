use std::fs::File;
use std::io::{BufRead, BufReader};


const LEN_BLOCK: usize = 14;


struct BlockCheck {
    block: [char; LEN_BLOCK],
    index: usize
}

impl BlockCheck {
    fn new() -> Self {
        Self {block: ['a'; LEN_BLOCK], index: 0}
    }

    fn add_char(&mut self, element: char) -> bool {
        if self.index < LEN_BLOCK-1 {
            self.block[self.index] = element;
            self.index += 1;
            return false;
        }
        let index_to_replace = self.index % LEN_BLOCK;
        self.block[index_to_replace] = element;
        self.index += 1;
        return all_different(&self.block);
    }
}


fn all_different(char_array: &[char; LEN_BLOCK]) -> bool {
    for i in 0..LEN_BLOCK {
        for j in 0..i {
            if char_array[i] == char_array[j] {
                return false
            }
        }
    }
    true
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let line = reader.lines().next().unwrap().unwrap();
    let mut chars = line.chars();
    let mut block_check = BlockCheck::new();
    while !block_check.add_char(chars.next().unwrap()) {
    }
    println!("Part 1: {}", block_check.index);
}


fn main() {
    part1();
}
