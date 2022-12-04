use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(text: String) -> Self {
        let mut t_sp = text.split("-");
        let start: i32 = t_sp.next().unwrap().parse().unwrap();
        let end: i32 = t_sp.next().unwrap().parse().unwrap();
        Self{start, end}
    }

    fn copy(&self) -> Self {
        Self{start: self.start, end: self.end}
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn contains_number(&self, number: i32) -> bool {
        self.start <= number && self.end >= number
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        if self.contains(other) {
            return Some(other.copy());
        }
        if other.contains(self) {
            return Some(self.copy());
        }
        if self.contains_number(other.end) {
            return Some(Self{start: self.start, end: other.end});
        }
        if self.contains_number(other.start) {
            return Some(Self{start: other.start, end: self.end});
        }
        None
    }
}

#[derive(Debug)]
struct OverlapList {
    list: Vec<Range>
}

impl OverlapList {
    fn new() -> Self {
        Self{list: Vec::new()}
    }

    fn add(&mut self, mut overlap: Range) {
        let mut insert_index = 0;
        while insert_index < self.list.len() {
            if self.list[insert_index].contains(&overlap) {
                return;
            }
            if overlap.end < self.list[insert_index].start {
                break
            }
            if overlap.start > self.list[insert_index].end {
                insert_index += 1;
                continue;
            }
            if self.list[insert_index].contains_number(overlap.end) {
                overlap.end = self.list[insert_index].end;
            }
            if self.list[insert_index].contains_number(overlap.start) {
                overlap.start = self.list[insert_index].start;
            }
            self.list.remove(insert_index);
        }
        self.list.insert(insert_index, overlap);
    }

    fn num_elements(&self) -> usize {
        self.list.iter().map(|range| (range.end - range.start + 1) as usize).sum()
    }
}

fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_sp = line.split(",");
        let range1 = Range::new(line_sp.next().unwrap().to_string());
        let range2 = Range::new(line_sp.next().unwrap().to_string());
        result += (range1.contains(&range2) || range2.contains(&range1)) as usize;
    }
    println!("Part 1: {}", result);
}


fn part2() {
    let reader = read_file();
    let mut overlaps = OverlapList::new();
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_sp = line.split(",");
        let range1 = Range::new(line_sp.next().unwrap().to_string());
        let range2 = Range::new(line_sp.next().unwrap().to_string());
        let overlap = range1.overlap(&range2);
        match overlap {
            Some(range) => {
                overlaps.add(range);
                result += 1;
            },
            None => {},
        }
    }
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
