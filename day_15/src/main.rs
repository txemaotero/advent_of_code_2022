use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


#[derive(Debug)]
struct Sensor {
    position: [i32; 2],
    closest_becon: [i32; 2],
}

impl Sensor {
    fn from_line(line: &str) -> Self {
        let re = Regex::new(r"x=\s*(-?\d+),\s*y=(-?\d+):.*x=\s*(-?\d+),\s*y=(-?\d+)").unwrap();
        // let re = Regex::new(r"x=\s*(-?\d+).*").unwrap();
        let caps = re.captures(line).unwrap();
        let position = [caps[1].parse().unwrap(), caps[2].parse().unwrap()];
        let closest_becon = [caps[3].parse().unwrap(), caps[4].parse().unwrap()];
        Sensor { position, closest_becon }
    }

    fn distance_to_beacon(&self) -> i32 {
        let x = self.position[0] - self.closest_becon[0];
        let y = self.position[1] - self.closest_becon[1];
        x.abs() + y.abs()
    }

    fn cross_y_line(&self, y: i32) -> bool {
        let dist = self.distance_to_beacon();
        if y > self.position[1] {
            return y <= self.position[1] + dist;
        }
        y >= self.position[1] - dist
    }

    fn interest_range(&self, y: i32, remove_beacon: bool) -> Option<[i32; 2]> {
        if !self.cross_y_line(y) {
            return None;
        }
        let dist = self.distance_to_beacon();
        let range_semi = dist - (y - self.position[1]).abs();
        let mut left = self.position[0] - range_semi;
        let mut right = self.position[0] + range_semi;
        if self.closest_becon[1] == y && remove_beacon {
            if self.closest_becon[0] < self.position[0] {
                left +=1;
            } else if self.closest_becon[0] > self.position[0] {
                right -=1;
            } else {
                return None;
            }
        }
        Some([left, right])
    }
}

fn range_union(range1: [i32; 2], range2: [i32; 2]) -> Option<[i32; 2]> {
    if overlaps(range1, range2) {
        Some([range1[0].min(range2[0]), range1[1].max(range2[1])])
    } else {
        None
    }
}

fn overlaps(range1: [i32; 2], range2: [i32; 2]) -> bool {
    range1[0] <= range2[1] && range2[0] <= range1[1]
}

fn add_range_to_list(range: [i32; 2], list: &mut Vec<[i32; 2]>) {
    let mut i = 0;
    while i < list.len() {
        if let Some(new_range) = range_union(range, list[i]) {
            list.remove(i);
            add_range_to_list(new_range, list);
            return;
        }
        i += 1;
    }
    list.push(range);
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn range_is_full(ranges: &Vec<[i32; 2]>, target: &[i32; 2]) -> bool {
    for range in ranges {
        if is_contained(target, range) {
            return true;
        }
    }
    false
}

fn is_contained(target: &[i32; 2], range: &[i32; 2]) -> bool {
    target[0] >= range[0] && target[1] <= range[1]
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    let mut ranges = Vec::new();
    for line in reader.lines() {
        let sensor = Sensor::from_line(&line.unwrap());
        if let Some(range) = sensor.interest_range(2000000, true) {
            add_range_to_list(range, &mut ranges);
        }
    }
    for range in ranges {
        result += range[1] - range[0] + 1;
    }
    println!("Part 1: {}", result);
}

fn get_empty_point(sensors: &Vec<Sensor>, y_coord: i32) -> Option<[i32; 2]> {
    let mut ranges = Vec::new();
    for sensor in sensors {
        if let Some(range) = sensor.interest_range(y_coord, false) {
            add_range_to_list(range, &mut ranges);
        }
        if range_is_full(&ranges, &[0, 4000000]) {
            return None;
        }
    }
    for range in &ranges {
        if (range[1] > 0) & (range[1] < 4000000) {
            return Some([range[1] + 1, y_coord]);
        }
    }
    panic!("No empty point found");
}

fn part2() {
    let reader = read_file();
    let mut sensors = Vec::new();
    for line in reader.lines() {
        sensors.push(Sensor::from_line(&line.unwrap()));
    }
    for y_coord in 0..=4000000 {
        let empty_point = get_empty_point(&sensors, y_coord);
        if let Some(point) = empty_point {
            println!("Part 2: {:?}", 4000000*(point[0] as i64) + (point[1] as i64));
            return
        }
    }
    println!("Not empty find");
}


fn main() {
    part1();
    part2();
}
