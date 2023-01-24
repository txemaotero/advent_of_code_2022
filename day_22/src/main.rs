use std::fs::File;
use std::io::{BufRead, BufReader};

const FACES_COORDS: [[usize; 2]; 6] = [[0, 2], [0, 1], [1, 1], [3, 0], [2, 1], [2, 0]];

const EDGE_TRANSITION: [[[usize; 2]; 4]; 6] = [
    [[2, 1], [4, 1], [3, 0], [1, 1]],
    [[2, 2], [0, 3], [3, 3], [5, 3]],
    [[4, 2], [0, 0], [1, 0], [5, 2]],
    [[0, 2], [4, 0], [5, 0], [1, 2]],
    [[3, 1], [0, 1], [2, 0], [5, 1]],
    [[3, 2], [4, 3], [2, 3], [1, 3]],
];

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
                Ok(_) => {}
                Err(_) => break,
            }
        }
        match final_turn {
            'L' => {
                self.orientation = [-self.orientation[1], self.orientation[0]];
            }
            'R' => {
                self.orientation = [self.orientation[1], -self.orientation[0]];
            }
            'N' => {}
            _ => {
                panic!("Invalid final turn: {}", final_turn)
            }
        };
    }

    fn move_once(&mut self) -> Result<(), ()> {
        let index = if self.orientation[0] != 0 { 0 } else { 1 };

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
        (self.position[0] + 1) * 1000 + (self.position[1] + 1) * 4 + or_value
    }

}

impl std::fmt::Display for Playground {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, ch) in row.iter().enumerate() {
                if row_index == self.position[0] && col_index == self.position[1] {
                    write!(
                        f,
                        "{}",
                        match self.orientation {
                            [1, 0] => 'v',
                            [0, -1] => '<',
                            [-1, 0] => '^',
                            [0, 1] => '>',
                            _ => panic!("Invalid orientation: {:?}", self.orientation),
                        }
                        )?;
                } else {
                    write!(f, "{}", ch)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct CubicPlayground {
    grid: Vec<Vec<char>>,
    position: [usize; 2], // [row, col]
    orientation: [isize; 2],
    side_length: usize,
    // Sides
    // 0: bottom
    // 1: right
    // 2: top
    // 3: left
}

impl CubicPlayground {
    fn new(
        grid: Vec<Vec<char>>,
        position: [usize; 2],
        orientation: [isize; 2],
        side: usize,
        ) -> CubicPlayground {
        CubicPlayground {
            grid,
            position,
            orientation,
            side_length: side,
        }
    }

    fn set_initial_state(&mut self) {
        self.orientation = [0, 1];
        for (index, char) in self.grid[0].iter().enumerate() {
            if *char == '.' {
                self.position = [0, index];
                break;
            }
        }
    }

    fn apply_command(&mut self, command: (usize, char)) {
        let (steps, final_turn) = command;
        for _ in 0..steps {
            match self.move_once() {
                Ok(_) => {}
                Err(_) => break,
            }
        }
        match final_turn {
            'L' => {
                self.orientation = [-self.orientation[1], self.orientation[0]];
            }
            'R' => {
                self.orientation = [self.orientation[1], -self.orientation[0]];
            }
            'N' => {}
            _ => {
                panic!("Invalid final turn: {}", final_turn)
            }
        };
    }

    fn move_once(&mut self) -> Result<(), ()> {
        let (new_position, new_orientation) = self.get_new_state();

        let element = self.grid[new_position[0]][new_position[1]];
        if element == '#' {
            return Err(());
        }
        self.position = new_position;
        self.orientation = new_orientation;
        Ok(())
    }

    fn get_new_state(&self) -> ([usize; 2], [isize; 2]) {
        let [current_row, current_col] = self.position;
        let current_face_coords = [current_row/self.side_length, current_col/self.side_length];
        let mut current_face = FACES_COORDS.iter().position(|&x| x == current_face_coords).unwrap();
        let [shifted_row, shifted_col] = [current_row % self.side_length, current_col % self.side_length];
        let mut new_position;
        let mut new_orientation = self.orientation;
        let aux_last = (self.side_length - 1) as isize;
        match self.get_limit_side(shifted_row, shifted_col) {
            Some(limit_id) => {
                let [new_face, new_edge] = EDGE_TRANSITION[current_face][limit_id];
                let edge_diff = (limit_id as isize - new_edge as isize + 6) % 4;
                current_face = new_face;
                // 0 (no rotation)
                // 1 (-90)
                // 2 (-180)
                // 3 (-270)
                for _ in 0..edge_diff {
                    new_orientation = [new_orientation[1], -new_orientation[0]];
                }
                if edge_diff == 0 {
                    new_position = [
                        (shifted_row as isize - aux_last * self.orientation[0]) as usize,
                        (shifted_col as isize - aux_last * self.orientation[1]) as usize
                    ];
                } else if edge_diff == 1 {
                    new_position = [
                        (shifted_col as isize - self.orientation[1] * aux_last) as usize,
                        (aux_last as usize - shifted_row) * self.orientation[1].abs() as usize + shifted_row * self.orientation[0].abs() as usize
                    ];
                } else if edge_diff == 2 {
                    new_position = [
                        (self.orientation[0] * shifted_row as isize + self.orientation[1].abs()*(aux_last - shifted_row as isize)) as usize,
                        (self.orientation[1] * shifted_col as isize + self.orientation[0].abs()*(aux_last - shifted_col as isize)) as usize
                    ];
                } else if edge_diff == 3 {
                    new_position = [
                        (self.orientation[0] * (aux_last - shifted_col as isize)) as usize + self.orientation[1].abs() as usize * shifted_col,
                        shifted_row
                    ];
                } else {
                    panic!("Invalid edge_diff: {}", edge_diff);
                }
                let [face_row, face_col] = FACES_COORDS[current_face];
                new_position = [
                    new_position[0] + face_row * self.side_length,
                    new_position[1] + face_col * self.side_length
                ];
            }
            None => {
                new_position= [
                    (current_row as isize + self.orientation[0]) as usize,
                    (current_col as isize + self.orientation[1]) as usize,
                ];
            }
        }
        return (new_position, new_orientation);
    }


    fn get_limit_side(&self, row: usize, col: usize) -> Option<usize> {
        if row == 0 && self.orientation[0] == -1 {
            Some(2)
        } else if row == self.side_length - 1 && self.orientation[0] == 1 {
            Some(0)
        } else if col == 0 && self.orientation[1] == -1 {
            Some(3)
        } else if col == self.side_length - 1 && self.orientation[1] == 1 {
            Some(1)
        } else {
            None
        }
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
        (self.position[0] + 1) * 1000 + (self.position[1] + 1) * 4 + or_value
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

fn parse_input_part2() -> (CubicPlayground, Vec<(usize, char)>) {
    let mut grid = Vec::new();
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
        grid.push(line.chars().collect());
    }
    let playground = CubicPlayground::new(grid, [0, 0], [0, 1], 50);
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
    for command in commands {
        playground.apply_command(command);
    }
    println!("Part 1: {}", playground.get_score());
}

fn part2() {
    let (mut playground, commands) = parse_input_part2();
    playground.set_initial_state();
    for command in commands {
        playground.apply_command(command);
    }
    println!("Part 2: {}", playground.get_score());
}

fn main() {
    part1();
    part2();
}
