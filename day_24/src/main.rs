use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Blizard {
    direction: (isize, isize),
}

impl Blizard {
    fn new(direction: char) -> Blizard {
        return match direction {
            '^' => Blizard { direction: (0, -1) },
            'v' => Blizard { direction: (0, 1) },
            '<' => Blizard { direction: (-1, 0) },
            '>' => Blizard { direction: (1, 0) },
            _ => panic!("Unknown direction: {}", direction),
        };
    }
}

#[derive(Debug)]
struct Spot {
    position: (usize, usize),
    blizards: Vec<Blizard>,
    is_floor: bool,
}

impl Spot {
    fn new(position: (usize, usize), is_floor: bool) -> Spot {
        Spot {
            position,
            blizards: Vec::new(),
            is_floor,
        }
    }

    fn add_blizard(&mut self, blizard: Blizard) {
        self.blizards.push(blizard);
    }
}

struct Grid {
    spots: Vec<Vec<Spot>>,
    n_rows: usize,
    n_cols: usize,
    current_position: (usize, usize),
    start_position: (usize, usize),
    end_position: (usize, usize),
}

impl Grid {
    fn new(
        spots: Vec<Vec<Spot>>,
        start_position: (usize, usize),
        end_position: (usize, usize),
    ) -> Grid {
        let n_rows = spots.len();
        let n_cols = spots[0].len();
        Grid {
            spots,
            n_rows,
            n_cols,
            current_position: start_position,
            start_position,
            end_position,
        }
    }

    fn min_distance_to_end(&self, x: usize, y: usize) -> usize {
        let (x_end, y_end) = self.end_position;
        let x_diff = (x_end as isize - x as isize).abs() as usize;
        let y_diff = (y_end as isize - y as isize).abs() as usize;
        return x_diff + y_diff;
    }

    fn get_empty_neighbors(&self, x: usize, y: usize, time: usize) -> Vec<(usize, usize)> {
        let mut all_neighbors = self.get_neighbors(x, y);
        all_neighbors
            .into_iter()
            .filter(|(x, y)| !self.will_be_occupied(*x, *y, time))
            .collect()
    }

    fn will_be_occupied(&self, x: usize, y: usize, time: usize) -> bool {
        // Special case for the start position
        if y == 0{
            return false;
        }
        // See if comes from right
        let target_x = (x as isize - 1 + time as isize) % (self.n_cols as isize - 2) + 1;
        if self.spots[y][target_x as usize]
            .blizards
            .contains(&Blizard::new('<'))
        {
            return true;
        }
        // See if comes from left
        let mut target_x = x as isize - 1 - time as isize;
        while target_x < 0 {
            target_x += self.n_cols as isize - 2;
        }
        target_x = target_x % (self.n_cols as isize - 2) + 1;
        if self.spots[y][target_x as usize]
            .blizards
            .contains(&Blizard::new('>'))
        {
            return true;
        }
        // no blizards mooving vertically in the first column
        if x == 1 {
            return false;
        }
        // See if comes from bottom
        let target_y = (y as isize - 1 + time as isize) % (self.n_rows as isize - 2) + 1;
        if self.spots[target_y as usize][x]
            .blizards
            .contains(&Blizard::new('^'))
        {
            return true;
        }
        // See if comes from top
        let mut target_y = y as isize - 1 - time as isize;
        while target_y < 0 {
            target_y += self.n_rows as isize - 2;
        }
        target_y = target_y % (self.n_rows as isize - 2) + 1;
        if self.spots[target_y as usize][x]
            .blizards
            .contains(&Blizard::new('v'))
        {
            return true;
        }
        return false;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(5);
        neighbors.push((x, y));
        if x > 0 && !self.spots[y][x - 1].is_floor {
            neighbors.push((x - 1, y));
        }
        if x < self.n_cols - 1 && !self.spots[y][x + 1].is_floor {
            neighbors.push((x + 1, y));
        }
        if y > 0 && !self.spots[y - 1][x].is_floor {
            neighbors.push((x, y - 1));
        }
        if y < self.n_rows - 1 && !self.spots[y + 1][x].is_floor {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
}

fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn load_grid() -> Grid {
    let reader = read_file();
    let mut spots = Vec::new();
    let start_position = (1, 0);
    let mut end_position = (0, 0);
    for (row, line) in reader.lines().enumerate() {
        let mut row_spots = Vec::new();
        for (col, c) in line.unwrap().chars().enumerate() {
            let spot = match c {
                '#' => Spot::new((row, col), true),
                '.' => Spot::new((row, col), false),
                d => {
                    let mut spot = Spot::new((row, col), false);
                    spot.add_blizard(Blizard::new(d));
                    spot
                }
            };
            row_spots.push(spot);
            if col > 0 && row > 0 && row_spots[col - 1].is_floor && !row_spots[col].is_floor {
                end_position = (col, row);
            }
        }
        spots.push(row_spots);
    }
    Grid::new(spots, start_position, end_position)
}

struct PathNode {
    col_index: usize,
    row_index: usize,
    minute: usize,
    distance_to_end: usize,
}

impl PathNode {
    fn new(col_index: usize, row_index: usize, minute: usize, distance_to_end: usize) -> PathNode {
        PathNode {
            col_index,
            row_index,
            minute,
            distance_to_end,
        }
    }
    fn minimum_time(&self) -> usize {
        self.minute + self.distance_to_end
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.minimum_time().cmp(&self.minimum_time())
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.minimum_time() == other.minimum_time()
    }
}
impl Eq for PathNode {}

fn a_star(grid: &Grid, start_time: usize) -> usize {
    // Finds the minimum ammount of steps to cross the grid
    let mut visited = vec![vec![vec![]; grid.n_cols]; grid.n_rows];
    let mut heap = BinaryHeap::new();
    heap.push(PathNode::new(
        grid.start_position.0,
        grid.start_position.1,
        start_time,
        grid.min_distance_to_end(grid.start_position.0, grid.start_position.1),
    ));
    while let Some(path_node) = heap.pop() {
        let x = path_node.col_index;
        let y = path_node.row_index;
        if visited[y][x].contains(&path_node.minute) {
            continue;
        }
        visited[y][x].push(path_node.minute);
        if (x, y) == grid.end_position {
            return path_node.minute;
        }

        for new_position in grid.get_empty_neighbors(x, y, path_node.minute + 1) {
            let (x_new, y_new) = new_position;
            if visited[y_new][x_new].contains(&(path_node.minute + 1)) {
                continue;
            }
            let distance_to_end = grid.min_distance_to_end(x_new, y_new);
            heap.push(PathNode::new(x_new, y_new, path_node.minute + 1, distance_to_end));
        }
    }
    0
}

fn part1() {
    let grid = load_grid();
    let result = a_star(&grid, 0);
    println!("Part 1: {}", result);
}

fn part2() {
    let mut grid = load_grid();
    let go = a_star(&grid, 0);

    let aux_end = grid.end_position;
    grid.end_position = grid.start_position;
    grid.start_position = aux_end;
    let back = a_star(&grid, go) - go;

    let aux_end = grid.end_position;
    grid.end_position = grid.start_position;
    grid.start_position = aux_end;
    let go2 = a_star(&grid, go+back) - go - back;

    println!("Part 2: {}", go + back + go2);
}

fn main() {
    part1();
    part2();
}
