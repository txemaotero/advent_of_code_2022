use std::fs::File;
use std::io::{BufRead, BufReader};

const FACES_COORDS: [[usize; 2]; 6] = [[3, 0], [3, 1], [2, 1], [0, 2], [1, 1], [1, 2]];

const EDGE_TRANSITION: [[[usize; 2]; 4]; 6] = [
    [[3, 2], [1, 3], [2, 3], [4, 3]],
    [[3, 1], [5, 1], [2, 0], [0, 1]],
    [[1, 2], [5, 0], [4, 0], [0, 2]],
    [[5, 2], [1, 0], [0, 0], [4, 2]],
    [[2, 2], [5, 3], [3, 3], [0, 3]],
    [[2, 1], [1, 1], [3, 0], [4, 1]],
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
        (self.position[0] + 1) * 1000 + (self.position[1] + 1) * 4 + or_value
    }

    fn get_face_id(&self) -> usize {
        let [row, col] = self.position;
        let col_face = col / 50;
        let row_face = row / 50;
        FACES_COORDS
            .iter()
            .position(|(r, c)| *r == row_face && *c == col_face)
            .unwrap()
    }

    fn get_next_face(&self) -> usize {
        let face_id = self.get_face_id();
        if face_id == 0 {
            if self.orientation[0] == 1 {
                // down
                return 2;
            } else if self.orientation[0] == -1 {
                // up
                return 5;
            } else if self.orientation[1] == 1 {
                // right
                return 4;
            } else if self.orientation[1] == -1 {
                // left
                return 1;
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 1 {
            if self.orientation[0] == 1 { // down
            } else if self.orientation[0] == -1 { // up
            } else if self.orientation[1] == 1 { // right
            } else if self.orientation[1] == -1 { // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 2 {
            if self.orientation[0] == 1 { // down
            } else if self.orientation[0] == -1 { // up
            } else if self.orientation[1] == 1 { // right
            } else if self.orientation[1] == -1 { // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 3 {
            if self.orientation[0] == 1 { // down
            } else if self.orientation[0] == -1 { // up
            } else if self.orientation[1] == 1 { // right
            } else if self.orientation[1] == -1 { // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 4 {
            if self.orientation[0] == 1 { // down
            } else if self.orientation[0] == -1 { // up
            } else if self.orientation[1] == 1 { // right
            } else if self.orientation[1] == -1 { // left
            } else {
                panic!("Invalid orientation: {:?}", self.orientation);
            }
        } else if face_id == 5 {
            if self.orientation[0] == 1 { // down
            } else if self.orientation[0] == -1 { // up
            } else if self.orientation[1] == 1 { // right
            } else if self.orientation[1] == -1 { // left
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

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Grid {
        Grid { grid: vec![] }
    }

    fn add_row(&mut self, row: Vec<char>) {
        self.grid.push(row);
    }
}

struct CubicPlayground {
    faces: [Grid; 6],
    // conections[i][j] = [k, l, r] => Face i, side j is connected to face k, side l and r 90
    // degrees rotations are needed to match the sides.
    position: [usize; 2], // [row, col]
    orientation: [isize; 2],
    _side: usize,
    // Sides
    // 0: bottom
    // 1: right
    // 2: top
    // 3: left
}

impl CubicPlayground {
    fn new(
        faces: [Grid; 6],
        position: [usize; 2],
        orientation: [isize; 2],
    ) -> CubicPlayground {
        let _side = faces[0].grid.len();
        CubicPlayground {
            faces,
            position,
            orientation,
            _side,
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
        // Usar que para saltar de uno a otro se haría primero como si estuviesen contiguos y luego
        // se plicarían las rotaciones necesarias para ir a la cara correcta. Con el movimiento al
        // contiguo cambio de lado l a (l+2)%4. Después con cada rotación cambio de l a (l+1)%4. y
        // las coordenadas (i, j) (ya en el contiguo) se transforman en (S-j, i).
        let new_position, new_orientation = self.get_new_state();

        let element = self.faces[self.position[0]].grid[self.position[1]][self.position[2]];
        if element == '#' {
            return Err(());
        }
        self.position = new_position;
        self.orientation = new_orientation;
        Ok(())
    }

    fn get_new_state(&self) -> ([usize; 2], [isize; 2]) {
        let [current_row, current_col] = self.position;
        let [current_orientation_x, current_orientation_y] = self.orientation;
        let current_face_coords = [current_row/self._side, current_col/self._side];
        let current_face = FACES_COORDS.iter().position(|&x| x == current_face_coords).unwrap();
        let [shifted_row, shifted_col] = [current_row % self._side, current_col % self._side];
        let (new_position, new_orientation);
        match self.get_limit_side(shifted_row, shifted_col) {
            Some(limit_id) => {
                if limit_id == 0 && current_orientation_y == 1 {
                    let [new_face, new_edge] = EDGE_TRANSITION[current_face][limit_id];
                    let edge_diff = (limit_id as isize - new_edge as isize + 4) % 4;
                    // 3 (rigth)
                    // 1 (left)
                    // 2 (opposite)
                    // 0 (same)
                    if edge_diff == 0 {
                        new_orientation = [-current_orientation_y, -current_orientation_x];
                        new_position = [current_row, self._side - 1 - current_col];
                    } else if edge_diff == 2 {
                        new_orientation = [current_orientation_y, current_orientation_x];
                        new_position = [self._side - 1 - current_row, current_col];
                    } else if edge_diff == 1 {
                        new_orientation = [current_orientation_y, current_orientation_x];
                        new_position = [self._side - 1 - current_row, current_col];
                    } else if edge_diff == 3 {
                    }

                }
            },
            None => {
                new_position= [
                    (current_row as isize + current_orientation_x) as usize,
                    (current_col as isize + current_orientation_y) as usize,
                ];
                new_orientation  = [current_orientation_x, current_orientation_y];
            }
        }
        return (new_position, new_orientation);
    }


    fn get_limit_side(&self, row: usize, col: usize) -> Option<usize> {
        if row == 0 {
            Some(2)
        } else if row == self._side - 1 {
            Some(0)
        } else if col == 0 {
            Some(3)
        } else if col == self._side - 1 {
            Some(1)
        } else {
            None
        }
    }
}

// cube
// [0, 1, 1]
// [0, 1, 0]
// [1, 1, 0]
// [1, 0, 0]

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
