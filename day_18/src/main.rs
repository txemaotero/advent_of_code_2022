use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

// This ensures all the points inside the grid
const MAX_SIZE: usize = 22;

struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn contains(&self, value: i32) -> bool {
        self.start <= value && value <= self.end
    }

    fn contains_interval(&self, other: &Interval) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }

    fn union(&self, other: &Self) -> Option<Self> {
        if self.contains_interval(other) {
            return Some(self.clone());
        } else if other.contains_interval(self) {
            return Some(other.clone());
        } else if self.contains(other.start) {
            return Some(Interval::new(self.start, other.end));
        } else if self.contains(other.end) {
            return Some(Interval::new(other.start, self.end));
        } else if self.end + 1 == other.start {
            return Some(Interval::new(self.start, other.end));
        } else if other.end + 1 == self.start {
            return Some(Interval::new(other.start, self.end));
        } else {
            return None;
        }
    }

    fn clone(&self) -> Self {
        Self::new(self.start, self.end)
    }
}

fn add_interval_to_vec(intervals: &mut Vec<Interval>, interval: Interval) {
    let mut i = 0;
    while i < intervals.len() {
        if let Some(union) = intervals[i].union(&interval) {
            intervals[i] = union;
            let mut j = i + 1;
            while j < intervals.len() {
                if let Some(union) = intervals[i].union(&intervals[j]) {
                    intervals[i] = union;
                    intervals.remove(j);
                } else {
                    j += 1;
                }
            }
            return;
        }
        i += 1;
    }
    intervals.push(interval);
}

fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn get_lattice() -> [[[bool; MAX_SIZE]; MAX_SIZE]; MAX_SIZE] {
    let reader = read_file();
    let mut lattice = [[[false; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];
    for line in reader.lines() {
        let line = line.unwrap();
        let xyz: Vec<_> = line.split(",")
            .map(|s| 1 + s.parse::<usize>().unwrap())
            .collect();
        lattice[xyz[0]][xyz[1]][xyz[2]] = true;
    }
    lattice
}

fn get_sights() -> HashMap<String, HashMap<i32, HashMap<i32, Vec<Interval>>>> {
    let reader = read_file();
    let mut max_vals = vec![0, 0, 0];
    let mut min_vals = vec![100, 100, 100];
    let mut sights: HashMap<String, HashMap<i32, HashMap<i32, Vec<Interval>>>> = HashMap::new();
    sights.insert("xy".to_string(), HashMap::new());
    sights.insert("xz".to_string(), HashMap::new());
    sights.insert("yz".to_string(), HashMap::new());
    for line in reader.lines() {
        let line = line.unwrap();
        let xyz: Vec<_> = line.split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        for i in 0..3 {
            max_vals[i] = max_vals[i].max(xyz[i]);
            min_vals[i] = min_vals[i].min(xyz[i]);
        }

        let intervals = sights.get_mut("xy").unwrap()
            .entry(xyz[0])
            .or_insert(HashMap::new())
            .entry(xyz[1])
            .or_insert(Vec::new());
        add_interval_to_vec(intervals, Interval::new(xyz[2], xyz[2]));
        let intervals = sights.get_mut("xz").unwrap()
            .entry(xyz[0])
            .or_insert(HashMap::new())
            .entry(xyz[2])
            .or_insert(Vec::new());
        add_interval_to_vec(intervals, Interval::new(xyz[1], xyz[1]));
        let intervals = sights.get_mut("yz").unwrap()
            .entry(xyz[1])
            .or_insert(HashMap::new())
            .entry(xyz[2])
            .or_insert(Vec::new());
        add_interval_to_vec(intervals, Interval::new(xyz[0], xyz[0]));
    }
    println!("Max: {:?}", max_vals);
    println!("Min: {:?}", min_vals);
    sights
}

fn part1() {
    let sights = get_sights();
    let result = sights.iter()
        .map(|(_, sight)| sight.iter()
            .map(|(_, sight)| sight.iter()
                .map(|(_, intervals)| 2* intervals.len())
                .sum::<usize>())
            .sum::<usize>())
        .sum::<usize>();
    println!("Part 1: {}", result);
}

fn get_neighbours(indexes: [usize; 3]) -> Vec<[usize; 3]> {
    let mut neighbours = Vec::new();
    for i in 0..3 {
        let x = indexes[i];
        if x > 0 {
            let mut aux = indexes.clone();
            aux[i] = x - 1;
            neighbours.push(aux);
        }
        if x < MAX_SIZE - 1 {
            let mut aux = indexes.clone();
            aux[i] = x + 1;
            neighbours.push(aux);
        }
    }
    neighbours
}

fn part2() {
    let lattice = get_lattice();
    let mut result = 0;
    let mut missing_points = vec![[0, 0, 0]];
    let mut visited = HashSet::new();
    while missing_points.len() != 0 {
        let current = missing_points.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        let neighbours = get_neighbours(current);
        for neighbour in neighbours {
            if lattice[neighbour[0]][neighbour[1]][neighbour[2]] {
                result += 1;
            } else {
                missing_points.push(neighbour);
            }
        }
    }
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
