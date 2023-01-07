use std::fs::File;
use std::io::{BufRead, BufReader};




struct CyclicVector {
    data: Vec<isize>,
}

impl CyclicVector {
    fn new(data: Vec<isize>) -> CyclicVector {
        CyclicVector {
            data
        }
    }

    fn mix(&mut self) {
        let len = self.data.len();
        let mut positions = (0..len).collect::<Vec<_>>();
        for i in 0..len {
            let xi = self.data[i] % (len - 1) as isize;
            println!("xi: {}", xi);
            if xi == 0 {
                continue;
            }
            let mut global_shift = 0;
            let pi = positions[i];
            if xi < 0 && (pi as isize + xi) <= 0 {
                global_shift -= 1;
                println!("-1");
            } else if xi > 0 && (pi as isize + xi) >= len as isize {
                global_shift += 1;
                println!("+1");
            } else {
                println!("0");
            }
            let direction = xi.signum();
            let is_positive = if direction == 1 {true} else {false};

            // println!("{}", offset);
            let new_pos = self.to_positive_index(pi as isize + xi);
            for j in 0..len {
                if i == j {
                    continue;
                }
                let pj = positions[j];
                // println!("pj: {}, a: {}, b: {}, direction: {} -> {}", pj_with_offset, pi_with_offset, new_pos, direction, self.is_between(pj_with_offset, pi_with_offset, new_pos, is_positive));
                if self.is_between(pj, pi, new_pos, is_positive) {
                    positions[j] = self.to_positive_index(pj as isize + direction * (-1));
                }
            }
            positions[i] = new_pos;
            for j in 0..len {
                positions[j] = self.to_positive_index(positions[j] as isize + global_shift);
            }
            println!("positions: {:?}", &positions);
            println!("After {} moves: {:?}", i +1, self.build_new_data(&positions));
        }
        println!("positions: {:?}", positions);
        // build the new vector
        let new_vec = self.build_new_data(&positions);
        self.data = new_vec;
    }

    fn build_new_data(&self, positions: &Vec<usize>) -> Vec<isize> {
        let mut new_vec = vec![0; self.data.len()];
        for i in 0..self.data.len() {
            new_vec[positions[i]] = self.data[i];
        }
        new_vec
    }

    fn is_between(&self, x: usize, a: usize, b: usize, positive_direction: bool) -> bool {
        if positive_direction {
            if a < b {
                a < x && x <= b
            } else {
                !(b <= x && x < a)
            }
        } else {
            if a < b {
                !(a <= x && x < b)
            } else {
                b <= x && x < a
            }
        }
    }

    fn to_positive_index(&self, index: isize) -> usize {
        let len = self.data.len();
        if index >= 0 && (index as usize) < len {
            return index as usize;
        } else if index >= len as isize {
            return (index % len as isize) as usize;
        }
        let mut new_index = index;
        while new_index < 0 {
            new_index += len as isize;
        }
        new_index as usize
    }

    fn get(&self, index: isize) -> isize {
        if index < 0 {
            self.get(self.data.len() as isize + index)
        } else {
            self.data[index as usize % self.data.len()]
        }
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    let file = File::open("../example.txt").unwrap();
    BufReader::new(file)
}


fn get_vector() -> Vec<isize> {
    let buffer = read_file();
    let mut vector = Vec::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        vector.push(line.parse::<isize>().unwrap());
    }
    vector
}

fn part1() {
    let mut vector = CyclicVector::new(get_vector());
    println!("vector init: {:?}", vector.data);
    vector.mix();
    println!("vector end: {:?}", vector.data);

    let result = vector.get(1000) + vector.get(2000) + vector.get(3000);
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
