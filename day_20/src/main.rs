use std::fs::File;
use std::io::{BufRead, BufReader};


struct CyclicVector {
    data: Vec<isize>,
}

impl CyclicVector {
    fn new(data: Vec<isize>) -> CyclicVector {
        CyclicVector {
            data,
        }
    }

    fn mix(&mut self, n_times: usize) {
        let len = self.data.len();
        let mut positions = (0..len).collect::<Vec<_>>();
        for _ in 0..n_times {
            for i in 0..len {
                let xi = self.data[i] % (len - 1) as isize;
                if xi == 0 {
                    continue;
                }
                let pi = positions[i];
                let to_move: isize = if (xi < 0) && (pi as isize + xi <= 0) {
                    len as isize - 1 + xi
                } else if (xi > 0) && (pi as isize + xi >= len as isize) {
                    -(len as isize - 1 - xi)
                } else {
                    xi
                };
                let direction = to_move.signum();
                let new_pos = (pi as isize + to_move) as usize;
                for j in 0..len {
                    let pj = positions[j];
                    if j == i {
                        continue;
                    }
                    if self.is_between(pj, pi, new_pos) {
                        positions[j] = (positions[j] as isize - direction) as usize;
                    }
                }
                positions[i] = new_pos;
            }
        }
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

    fn is_between(&self, x: usize, a: usize, b: usize) -> bool {
        if a < b {
            a < x && x <= b
        } else {
            b <= x && x < a
        }
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
    vector.mix(1);

    let zero_index = vector.data.iter().position(|&r| r == 0).unwrap();
    let result: isize = (1..=3).map(|i| vector.get((1000*i + zero_index) as isize)).sum();
    println!("Part 1: {}", result);
}


fn part2() {
    let vector = get_vector();
    let mut vector = CyclicVector::new(vector.iter().map(|&x| x*811589153).collect());
    vector.mix(10);

    let zero_index = vector.data.iter().position(|&r| r == 0).unwrap();
    let result: isize = (1..=3).map(|i| vector.get((1000*i + zero_index) as isize)).sum();

    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
