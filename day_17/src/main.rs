use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const WIDTH: usize = 7;


#[derive(Debug)]
struct CyclicBuffer<T> {
    buffer: Vec<T>,
    position: usize,
}

impl <T> CyclicBuffer<T> {
    fn new(buffer: Vec<T>) -> CyclicBuffer<T> {
        CyclicBuffer {
            buffer,
            position: 0,
        }
    }

    fn next(&mut self) -> &T {
        let value = &self.buffer[self.position];
        self.position = (self.position + 1) % self.buffer.len();
        value
    }
}

struct Piece {
    left_position: usize,
    bottom_limit: usize,
    heights: Vec<usize>,
    start_heights: Vec<usize>,
    id: usize,
}

impl Piece {
    fn horizontal() -> Self {
        Piece {
            left_position: 2,
            bottom_limit: 0,
            heights: vec![1, 1, 1, 1],
            start_heights: vec![0, 0, 0, 0],
            id: 0,
        }
    }

    fn vertical() -> Self {
        Piece {
            left_position: 2,
            bottom_limit: 0,
            heights: vec![4],
            start_heights: vec![0],
            id: 1,
        }
    }

    fn cross() -> Self {
        Piece {
            left_position: 2,
            bottom_limit: 0,
            heights: vec![1, 3, 1],
            start_heights: vec![1, 0, 1],
            id: 2,
        }
    }

    fn square() -> Self {
        Piece {
            left_position: 2,
            bottom_limit: 0,
            heights: vec![2, 2],
            start_heights: vec![0, 0],
            id: 3,
        }
    }

    fn l_shape() -> Self {
        Piece {
            left_position: 2,
            bottom_limit: 0,
            heights: vec![1, 1, 3],
            start_heights: vec![0, 0, 0],
            id: 4,
        }
    }

    fn clone(&self) -> Self {
        Piece {
            left_position: self.left_position,
            bottom_limit: self.bottom_limit,
            heights: self.heights.clone(),
            start_heights: self.start_heights.clone(),
            id: self.id
        }
    }

    fn set_position(&mut self, left_position: usize, bottom_limit: usize) {
        self.left_position = left_position;
        self.bottom_limit = bottom_limit;
    }

    fn move_down(&mut self) {
        self.bottom_limit -= 1;
    }

    fn parts_index(&self) -> Vec<[usize; 2]> {
        let mut parts_index = Vec::new();
        for column in self.left_position..(self.left_position + self.heights.len()) {
            let local_column = column - self.left_position;
            let start = self.bottom_limit + self.start_heights[local_column];
            let end = start + self.heights[local_column];
            for row in start..end {
                parts_index.push([row, column]);
            }
        }
        parts_index
    }
}

struct Well {
    height_offset: u64,
    layers: Vec<[bool; WIDTH]>,
    seen_disp: HashMap<[i32; 5], i32>,
}

impl std::fmt::Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for layer in self.layers.iter().rev() {
            write!(f, "|")?;
            for &cell in layer.iter() {
                if cell {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        for _ in 0..(WIDTH + 2) {
            write!(f, "-")?;
        }
        Ok(())
    }
}

impl Well {
    fn new() -> Self {
        Well {
            height_offset: 0,
            layers: vec![[false; WIDTH]; 7],
            seen_disp: HashMap::new(),
        }
    }

    fn get_initial_disp(&mut self, commands: &mut CyclicBuffer<i32>, piece: &Piece) -> i32 {
        let key = [piece.id as i32, *commands.next(), *commands.next(), *commands.next(), *commands.next()];
        if let Some(&disp) = self.seen_disp.get(&key) {
            return disp;
        }
        let mut total_disp = 2;
        for index in 1..5 {
            if key[index] == -1 {
                if total_disp > 0 {
                    total_disp -= 1;
                }
            } else {
                if (total_disp + piece.heights.len()) < WIDTH {
                    total_disp += 1;
                }
            }
        }
        self.seen_disp.insert(key, total_disp as i32);
        total_disp as i32
    }

    fn next_command(&self, commands: &mut CyclicBuffer<i32>, piece: &mut Piece) {
        let cmd = *commands.next() as isize;
        if self.can_move_piece(piece, [0, cmd]) {
            let new_position = piece.left_position as isize + cmd;
            piece.left_position = new_position as usize;
        }
    }

    fn can_move_piece(&self, piece: &Piece, direction: [isize; 2]) -> bool {
        for [row, col] in piece.parts_index() {
            let new_row = row as isize + direction[0];
            let new_col = col as isize + direction[1];
            if new_row < 0 || new_col < 0 || new_col as usize >= WIDTH || self.layers[new_row as usize][new_col as usize] {
                return false;
            }
        }
        return true;
    }

    fn get_min_height(&self) -> usize {
        let mut aux_seen_index = [-1; WIDTH];
        for (layer_ind, layer) in self.layers.iter().enumerate() {
            for i in 0..WIDTH {
                if layer[i] {
                    aux_seen_index[i] = layer_ind as i32;
                }
            }
        }
        (*aux_seen_index.iter().min().unwrap() + 1) as usize
    }

    fn get_max_height(&self) -> usize {
        for (height, layer) in self.layers.iter().enumerate() {
            if layer.iter().all(|&x| !x) {
                return height;
            }
        }
        self.layers.len()
    }

    fn add_piece(&mut self, piece: &mut Piece, commands: &mut CyclicBuffer<i32>) {
        let initial_left = self.get_initial_disp(commands, piece);
        piece.set_position(initial_left as usize, self.get_max_height());
        while self.can_move_piece(&piece, [-1, 0]) {
            piece.move_down();
            self.next_command(commands, piece);
        }
        self.update_profile(piece);
    }


    fn update_profile(&mut self, piece: &Piece) {
        for i in 0..WIDTH {
            if i >= piece.left_position && i < (piece.left_position + piece.heights.len()) {
                let index = i - piece.left_position;
                let start_height = piece.start_heights[index] + piece.bottom_limit;
                let end_height = piece.heights[index] + start_height;
                for j in start_height..end_height {
                    self.layers[j][i] = true;
                }
            }
        }
        let min_height = self.get_min_height();
        if min_height > 0 {
            self.height_offset += min_height as u64;
            for _ in 0..min_height {
                self.layers.remove(0);
            }
        }
        let missing_layers = 7 - (self.layers.len() - self.get_max_height());
        for _ in 0..missing_layers {
            self.layers.push([false; WIDTH]);
        }
    }

    fn total_height(&self) -> u64 {
        self.get_max_height() as u64 + self.height_offset
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn get_pieces() -> CyclicBuffer<Piece> {
    let pieces = vec![
        Piece::horizontal(),
        Piece::cross(),
        Piece::l_shape(),
        Piece::vertical(),
        Piece::square()
    ];
    CyclicBuffer::new(pieces)
}

fn parse_commands() -> CyclicBuffer<i32> {
    let reader = read_file();
    CyclicBuffer::new(reader.lines().next().unwrap().unwrap().chars()
                      .map(|c| {
                          if c == '<' {
                              -1
                          } else {
                              1
                          }
                      }).collect::<Vec<i32>>())
}

fn part1() {
    let mut commands = parse_commands();
    let mut pieces = get_pieces();
    let mut well = Well::new();

    for _ in 0..2022 {
        let mut piece = pieces.next().clone();
        well.add_piece(&mut piece, &mut commands);
    }
    println!("Part 1: {}", well.total_height());
}

fn part2() {
    let mut commands = parse_commands();

    let mut pieces = [
        Piece::horizontal(),
        Piece::cross(),
        Piece::l_shape(),
        Piece::vertical(),
        Piece::square()
    ];
    let mut index = 0;
    let mut well = Well::new();
    let mut memory: HashMap<(usize, usize), HashMap<Vec<[bool; WIDTH]>, (u64, u64)>> = HashMap::new();
    let total_pieces = 1000000000000u64;

    for i in 0..total_pieces {
        well.add_piece(&mut pieces[index], &mut commands);
        let key1 = (pieces[index].id, commands.position);
        let entry = memory.entry(key1).or_insert(HashMap::new());
        let key2 = well.layers.clone();
        match entry.get(&key2) {
            Some(&val) => {
                let remaining = total_pieces - i;
                let pieces_interval = i - val.0;
                let current_height = well.total_height();
                let height_interval = current_height - val.1;
                let mut height = val.1 + height_interval * ((remaining / pieces_interval) + 1);
                let missing = remaining % pieces_interval;
                for new_ind in 0..(missing) {
                    well.add_piece(&mut pieces[((new_ind+ 1 + index as u64 )%5) as usize], &mut commands);
                }
                height += well.total_height() - current_height - 1;
                println!("Part 2: {}", height);
                return;
            },
            None => {
                entry.insert(key2, (i, well.total_height()));
            }
        }

        index = (index + 1) % 5;
    }
    println!("Part 2: {}", well.total_height());
}


fn main() {
    part1();
    part2();
}
