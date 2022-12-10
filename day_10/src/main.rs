use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;


fn read_file() -> BufReader<File> {
    let file = File::open("../example.txt").unwrap();
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    let mut cycle = 0;
    let mut value = 1;
    for line in reader.lines() {
        let line = line.unwrap();
        cycle += 1;
        if ((cycle - 20) % 40) == 0 {
            result += cycle*value;
        }
        if !line.starts_with("noop") {
            let new_val: i32 = line.split_whitespace()
                .last().unwrap()
                .parse().unwrap();
            cycle += 1;
            if ((cycle - 20) % 40) == 0 {
                result += cycle*value;
            }
            value += new_val;
        }
    }
    println!("Part 1: {}", result);
}

struct Screen {
    pixels: [[bool; 40]; 6],
    crt: i32,
    cycle: u32,
}

impl Screen {
    fn new() -> Self {
        let mut result = Self {pixels: [[false; 40]; 6], crt: 1, cycle: 0};
        result.activate_pixel();
        result
    }

    fn add_new_command(&mut self, command: String) {
        if command.starts_with("noop") {
            self.noop();
        } else {
            let new_val: i32 = command.split_whitespace()
                .last().unwrap()
                .parse().unwrap();
            self.add_value(new_val);
        }
    }

    fn noop(&mut self) {
        self.cycle += 1;
        self.activate_pixel();
    }

    fn add_value(&mut self, value: i32) {
        self.noop();
        self.crt += value;
        self.noop();
    }

    fn activate_pixel(&mut self) {
        let col = self.cycle % 40;
        let row = self.cycle / 40;
        let diff = (col as i32 - self.crt).abs();
        if diff <= 1 {
            self.pixels[row as usize][col as usize] = true;
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let mut result = String::new();
        for row in self.pixels {
            for col in row {
                if col {
                    result.push_str("#");
                } else {
                    result.push_str(".");
                }
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}


fn part2() {
    let reader = read_file();
    let mut result = Screen::new();
    for line in reader.lines() {
        let line = line.unwrap();
        result.add_new_command(line);
    }
    println!("{}", result);
}


fn main() {
    part1();
    part2();
}
