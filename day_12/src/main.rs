use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;


const CHARS_INDEXES: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z'];

struct Node {
    row_index: usize,
    col_index: usize,
    height: u32,
    cost: u32,
    path_length: u32,
}

impl Node {
    fn new(row_index: usize, col_index: usize, height_char: char) -> Node {
        let height = CHARS_INDEXES.iter()
            .position(|&c| c == height_char).unwrap() as u32;
        Node {
            row_index,
            col_index,
            height,
            cost: 0,
            path_length: 0,
        }
    }

    fn clone(&self) -> Node {
        Node {
            row_index: self.row_index,
            col_index: self.col_index,
            height: self.height,
            cost: self.cost,
            path_length: self.path_length,
        }
    }

    fn distance_to(&self, other: &Node) -> u32 {
        let row_diff = (self.row_index as i32 - other.row_index as i32).abs();
        let col_diff = (self.col_index as i32 - other.col_index as i32).abs();
        (row_diff + col_diff) as u32
    }

    fn neighbors(&self, landscape: &Landscape) -> Vec<Node> {
        let mut neighbors = Vec::new();
        if self.row_index > 0 {
            neighbors.push(landscape.map[self.row_index - 1][self.col_index].clone());
        }
        if self.row_index < landscape.n_rows - 1 {
            neighbors.push(landscape.map[self.row_index + 1][self.col_index].clone());
        }
        if self.col_index > 0 {
            neighbors.push(landscape.map[self.row_index][self.col_index - 1].clone());
        }
        if self.col_index < landscape.n_cols - 1 {
            neighbors.push(landscape.map[self.row_index][self.col_index + 1].clone());
        }
        neighbors
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.row_index == other.row_index &&
            self.col_index == other.col_index
    }
}

impl Eq for Node {}


struct Landscape {
    map: Vec<Vec<Node>>,
    n_rows: usize,
    n_cols: usize,
    start: Node,
    end: Node,
}

impl Landscape {

    fn from(filename: &str) -> Landscape {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut map = Vec::new();
        let mut start = Node::new(0, 0, 'a');
        let mut end = Node::new(0, 0, 'a');
        for (row_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut row = Vec::new();
            for (col_index, mut char) in line.chars().enumerate() {
                if char == 'S' {
                    start = Node::new(row_index, col_index, 'a');
                    char = 'a';
                } else if char == 'E' {
                    end = Node::new(row_index, col_index, 'z');
                    char = 'z';
                }
                let node = Node::new(row_index, col_index, char);
                row.push(node);
            }
            map.push(row);
        }

        let n_rows = map.len();
        let n_cols = map[0].len();
        Landscape {
            map,
            n_rows,
            n_cols,
            start,
            end,
        }
    }
}

fn a_star (landscape: &Landscape) -> u32 {
    let mut visited = vec![vec![false; landscape.n_cols]; landscape.n_rows];
    let mut heap = BinaryHeap::new();
    heap.push(landscape.start.clone());
    while let Some(position) = heap.pop() {
        let x = position.col_index;
        let y = position.row_index;
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        if position == landscape.end {
            return position.path_length;
        }

        for mut new_position in position.neighbors(&landscape) {
            let x_new = new_position.col_index;
            let y_new = new_position.row_index;
            if visited[y_new][x_new] {
                continue;
            }
            if (position.height + 1) < new_position.height {
                continue;
            }
            let distance_to_end = new_position.distance_to(&landscape.end);
            new_position.path_length = position.path_length + 1;
            new_position.cost = new_position.path_length + distance_to_end;
            heap.push(new_position);
        }
    }
    0
}


fn part1() {
    let landscape = Landscape::from("../input.txt");
    let cost = a_star(&landscape);
    println!("Part 1: {}", cost);
}


fn part2() {
    let mut fewest_steps = 400;
    let mut landscape = Landscape::from("../input.txt");
    for row in &landscape.map {
        for node in row {
            if node.height != 0 {
                continue;
            }
            landscape.start = node.clone();
            let cost = a_star(&landscape);
            if cost == 0 {
                continue;
            }
            if cost < fewest_steps {
                fewest_steps = cost;
            }
        }
    }
    println!("Part 2: {}", fewest_steps);
}


fn main() {
    part1();
    part2();
}
