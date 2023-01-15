use std::fs::File;
use std::io::{BufRead, BufReader};

const FACES_COORDS: [(usize, usize); 6] = [(0, 2), (0, 1), (1, 1), (2, 1), (2, 0), (3, 0)];


struct Playground {
    grid: Vec<Vec<char>>,
    cols_limits: Vec<[usize; 2]>,
    rows_limits: Vec<[usize; 2]>,
    position: [usize; 2],
    orientation: [isize; 2],
}

impl Playground {
    fn new() -> Playground {
        Playground {
            grid: Vec::new(),
            cols_limits: Vec::new(),
            rows_limits: Vec::new(),
            position: [0, 0],
            orientation: [0, 1],
        }
    }

    fn add_row(&mut self, row: Vec<char>) {

        self.rows_limits.push(self.get_row_limits(&row));
        let row_index = self.grid.len();
        for (col_index, ch) in row.iter().enumerate() {
            if self.cols_limits.len() <= col_index {
                self.cols_limits.push([usize::MAX, 0]);
            }
            if self.grid.len() == 0 && *ch == '.' && self.position[1] == 0 {
                self.position[1] = col_index;
            }
            if *ch == '.' || *ch == '#' {
                self.cols_limits[col_index][1] = row_index;
                if self.cols_limits[col_index][0] == usize::MAX {
                    self.cols_limits[col_index][0] = row_index;
                }
            }
        }
        self.grid.push(row);
    }

    fn get_row_limits(&self, row: &Vec<char>) -> [usize; 2] {
        let mut limits = [0, 0];
        let mut found = false;
        for (col_index, ch) in row.iter().enumerate() {
            if (*ch == '.') || (*ch == '#') {
                if !found {
                    limits[0] = col_index;
                    found = true;
                }
                limits[1] = col_index;
            }
        }
        limits
    }

    fn apply_command(&mut self, command: (usize, char)) {
        let (steps, final_turn) = command;
        for _ in 0..steps {
            match self.move_once() {
                Ok(_) => {},
                Err(_) => break,
            }
        }
        match final_turn {
            'L' => {
                self.orientation = [-self.orientation[1], self.orientation[0]];
            },
            'R' => {
                self.orientation = [self.orientation[1], -self.orientation[0]];
            },
            'N' => {},
            _ => {panic!("Invalid final turn: {}", final_turn)},
        };
    }

    fn move_once(&mut self) -> Result<(), ()> {
        let index = if self.orientation[0] != 0 { 0 } else { 1 };

        // let position = self.position[(index + 1) % 2];
        let position = self.position[index];
        let limits;
        if index == 0 {
            limits = self.cols_limits[self.position[1]];
        } else {
            limits = self.rows_limits[self.position[0]];
        }
        let direction = self.orientation[index];

        let new_position = get_pbc_position(position, direction, limits);
        let element;
        if index == 0 {
            element = self.grid[new_position][self.position[1]];
        } else {
            element = self.grid[self.position[0]][new_position];
        }
        if element == '#' {
            return Err(());
        }
        self.position[index] = new_position;
        Ok(())
    }

    fn get_score(&self) -> usize {
        let or_value = if self.orientation[0] == 1 {
            1
        } else if self.orientation[1] == 1 {
            0
        } else if self.orientation[0] == -1 {
            3
        } else if self.orientation[1] == -1 {
            2
        } else {
            panic!("Invalid orientation: {:?}", self.orientation);
        };
        (self.position[0] + 1)*1000 + (self.position[1] + 1) * 4 + or_value
    }

    fn get_face_id(&self) -> usize {
        let [row, col] = self.position;
        let col_face = col / 50;
        let row_face = row / 50;
        FACES_COORDS.iter().position(|(r, c)| *r == row_face && *c == col_face).unwrap()
    }

    fn get_next_face(&self) -> usize {
        let face_id = self.get_face_id();
        if face_id == 0 {
            if self.orientation[0] == 1 {          // down
                2
            } else if self.orientation[0] == -1 {  // up
                5
            } else if self.orientation[1] == 1 {   // right
                4
            } else if self.orientation[1] == -1 {  // left
                1
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 1 {
            if self.orientation[0] == 1 {          // down
            } else if self.orientation[0] == -1 {  // up
            } else if self.orientation[1] == 1 {   // right
            } else if self.orientation[1] == -1 {  // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 2 {
            if self.orientation[0] == 1 {          // down
            } else if self.orientation[0] == -1 {  // up
            } else if self.orientation[1] == 1 {   // right
            } else if self.orientation[1] == -1 {  // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 3 {
            if self.orientation[0] == 1 {          // down
            } else if self.orientation[0] == -1 {  // up
            } else if self.orientation[1] == 1 {   // right
            } else if self.orientation[1] == -1 {  // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 4 {
            if self.orientation[0] == 1 {          // down
            } else if self.orientation[0] == -1 {  // up
            } else if self.orientation[1] == 1 {   // right
            } else if self.orientation[1] == -1 {  // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 5 {
            if self.orientation[0] == 1 {          // down
            } else if self.orientation[0] == -1 {  // up
            } else if self.orientation[1] == 1 {   // right
            } else if self.orientation[1] == -1 {  // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        }
        let next_face_id = (face_id + 1) % 6;
        next_face_id
    }
}


impl std::fmt::Display for Playground {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, ch) in row.iter().enumerate() {
                if row_index == self.position[0] && col_index == self.position[1] {
                    write!(f, "{}", match self.orientation {
                        [1, 0] => 'v',
                        [0, -1] => '<',
                        [-1, 0] => '^',
                        [0, 1] => '>',
                        _ => panic!("Invalid orientation: {:?}", self.orientation),
                    })?;
                } else {
                    write!(f, "{}", ch)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn get_pbc_position(position: usize, direction: isize, limits: [usize; 2]) -> usize {
    if position == limits[0] && direction == -1 {
        limits[1]
    } else if position == limits[1] && direction == 1 {
        limits[0]
    } else {
        (position as isize + direction) as usize
    }
}

fn read_file() -> BufReader<File> {
    let file = File::open("../example.txt").unwrap();
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn parse_input() -> (Playground, Vec<(usize, char)>) {
    let mut playground = Playground::new();
    let lines = read_file().lines();
    let mut read_commands = false;
    let mut commands = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if read_commands {
            commands = parse_commands(&line);
            break;
        }
        if line.trim().is_empty() {
            read_commands = true;
            continue;
        }
        playground.add_row(line.chars().collect());
    }
    (playground, commands)
}

fn parse_commands(commands: &String) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    let mut aux_num = String::new();
    for ch in commands.chars() {
        if ch.is_numeric() {
            aux_num.push(ch);
        } else {
            result.push((aux_num.parse().unwrap(), ch));
            aux_num.clear();
        }
    }
    if !aux_num.is_empty() {
        result.push((aux_num.parse().unwrap(), 'N'));
    }
    result
}


fn part1() {
    let (mut playground, commands) = parse_input();
    // println!("{}", playground);
    for command in commands {
        // println!("Applying command: {:?}", command);
        playground.apply_command(command);
        // println!("{}", playground);
    }

    println!("Part 1: {}", playground.get_score());
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
