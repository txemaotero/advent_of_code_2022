use std::fs::File;
use std::io::{BufRead, BufReader};


struct RockMap {
    // x are columns, y are rows
    map: Vec<Vec<usize>>,
    n_cols: usize,
    n_rows: usize,
    min_x: isize,
    min_y: usize,
    n_sand: usize,
}

impl RockMap {
    fn from(rock_cords: &Vec<[usize; 2]>, min_lim: [usize; 2], max_lim: [usize; 2], floor: bool) -> RockMap {
        let n_cols = max_lim[0] - min_lim[0] + 3;
        let mut n_rows = max_lim[1] - min_lim[1] + 3;
        let min_x = min_lim[0];
        let min_y = min_lim[1];
        let mut map = vec![vec![0; n_cols]; n_rows];
        for [x, y] in rock_cords {
            map[y - min_y + 1][x - min_x + 1] = 1;
        }
        if floor {
            map.push(vec![1; n_cols]);
            n_rows += 1;
        }
        let n_sand = 0;
        RockMap { map, n_cols, n_rows, min_x: min_x as isize, min_y, n_sand }
    }

    fn drop_sand(&mut self) -> bool {
        let mut current = [1, (500 - self.min_x) as usize + 1];
        loop {
            if current[0] == self.n_rows - 1 {
                // falls out of the map
                return false;
            }
            if self.map[current[0]][current[1]] == 2 {
                return false;
            }
            if current[1] == 0 {
                for i in 0..self.n_rows {
                    let val = self.map[i][0];
                    self.map[i].insert(0, val);
                }
                self.n_cols += 1;
                current[1] += 1;
                self.min_x -= 1;
                continue;
            }
            if current[1] == self.n_cols - 1 {
                for i in 0..self.n_rows {
                    let val = self.map[i][self.n_cols - 1];
                    self.map[i].push(val);
                }
                self.n_cols += 1;
                continue;
            }
            if self.map[current[0] + 1][current[1]] == 0 {
                current[0] += 1;
                continue;
            }
            if self.map[current[0] + 1][current[1] - 1] == 0 {
                current[0] += 1;
                current[1] -= 1;
                continue;
            }
            if self.map[current[0] + 1][current[1] + 1] == 0 {
                current[0] += 1;
                current[1] += 1;
                continue;
            }
            self.map[current[0]][current[1]] = 2;
            self.n_sand += 1;
            return true;
        }
    }
}

impl std::fmt::Display for RockMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.map {
            for col in row {
                if *col == 1 {
                    write!(f, "#")?;
                } else if *col == 2 {
                    write!(f, "o")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn read_all_rocks() -> Vec<[usize; 2]> {
    let mut rocks = Vec::new();
    let mut x_lims = [usize::MAX, 0];
    let mut y_lims = [usize::MAX, 0];
    for line in read_file().lines() {
        let line = line.unwrap();
        let corners: Vec<[usize; 2]> = line
            .split(" -> ")
            .into_iter()
            .map(|corner| {
                let mut corner = corner.split(",");
                let x = corner.next().unwrap().parse().unwrap();
                let y = corner.next().unwrap().parse().unwrap();
                x_lims[0] = x_lims[0].min(x);
                x_lims[1] = x_lims[1].max(x);
                y_lims[0] = y_lims[0].min(y);
                y_lims[1] = y_lims[1].max(y);
                [x, y]
            })
            .collect();
        for index in 0..corners.len() - 1 {
            let current = corners[index];
            let next = corners[index + 1];
            for step_x in next[0].min(current[0])..next[0].max(current[0]) {
                rocks.push([step_x+1, current[1]]);
            }
            for step_y in next[1].min(current[1])..next[1].max(current[1]) {
                rocks.push([current[0], step_y+1]);
            }
            rocks.push(current);
            rocks.push(next);
        }
    }
    rocks.push([x_lims[0], y_lims[0]]);
    rocks.push([x_lims[1], y_lims[1]]);
    rocks
}

fn part1() {
    let mut rocks = read_all_rocks();
    let max_point = rocks.pop().unwrap();
    let mut min_point = rocks.pop().unwrap();
    min_point[1] = 0;
    let mut rock_map = RockMap::from(&rocks, min_point, max_point, false);
    while rock_map.drop_sand() {}
    println!("Part 1: {}", rock_map.n_sand);
}


fn part2() {
    let mut rocks = read_all_rocks();
    let max_point = rocks.pop().unwrap();
    let mut min_point = rocks.pop().unwrap();
    min_point[1] = 0;
    let mut rock_map = RockMap::from(&rocks, min_point, max_point, true);
    while rock_map.drop_sand() { }

    println!("Part 2: {}", rock_map.n_sand);
}


fn main() {
    part1();
    part2();
}
