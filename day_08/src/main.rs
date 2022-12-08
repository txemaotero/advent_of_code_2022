use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


struct TreeRow {
    highest: u32,
    visible_right: Vec<(usize, u32)>,
}

impl TreeRow {
    fn new() -> Self {
        Self {highest: 0, visible_right: Vec::new()}
    }

    fn add_tree(&mut self, index: usize, height: u32) -> bool {
        if self.visible_right.len() == 0 {
            self.highest = height;
            self.visible_right.push((index, height));
            return true;
        }
        if height >= self.highest {
            let result = height > self.highest;
            self.highest = height;
            self.visible_right = vec![(index, height)];
            return result;
        }
        self.add_visible_right(index, height);
        return false;
    }

    fn add_visible_right(&mut self, index: usize, height: u32) {
        let mut index_to_add = 0;
        for (i, (_, tree_height)) in self.visible_right.iter().enumerate() {
            if tree_height <= &height {
                index_to_add = i;
                break;
            }
        }
        if index_to_add > 0 {
            self.visible_right.truncate(index_to_add);
        }
        self.visible_right.push((index, height));
    }

    fn get_right_visibles(&self) -> Vec<usize> {
        self.visible_right.iter().map(|t| t.0).collect()
    }

    fn clear(&mut self) {
        self.highest = 0;
        self.visible_right.clear();
    }
}

struct Forest {
    current_row_index: usize,
    current_row: TreeRow,
    columns: Vec<TreeRow>,
    visible_trees: HashSet<(usize, usize)>,
}

impl Forest {
    fn new() -> Self {
        Self {
            current_row_index: 0,
            current_row: TreeRow::new(),
            columns: Vec::new(),
            visible_trees: HashSet::new(),
        }
    }

    fn add_tree(&mut self, row_index: usize, column_index: usize, height: u32) {
        if row_index != self.current_row_index {
            self.collect_last_row();
            self.current_row_index = row_index;
            self.current_row.clear();
        }
        if self.columns.len() <= column_index {
            self.columns.push(TreeRow::new());
        }
        let mut added = self.current_row.add_tree(column_index, height);
        added |= self.columns[column_index].add_tree(row_index, height);
        if added {
            self.visible_trees.insert((row_index, column_index));
        }
    }

    fn collect_last_row(&mut self) {
        for col_ind in self.current_row.get_right_visibles().iter() {
            self.visible_trees.insert((self.current_row_index, *col_ind));
        }
    }

    fn collect_columns(&mut self) {
        for (col_ind, tree_row) in self.columns.iter().enumerate() {
            for row_ind in tree_row.get_right_visibles().iter() {
                self.visible_trees.insert((*row_ind, col_ind));
            }
        }
    }

    fn number_of_visible_trees(&self) -> usize {
        self.visible_trees.len()
    }
}

fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut forest = Forest::new();
    for (row_ind, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col_ind, c) in line.chars().enumerate() {
            let height: u32 = c.to_digit(10).unwrap();
            forest.add_tree(row_ind, col_ind, height);
        }
    }
    forest.collect_last_row();
    forest.collect_columns();
    println!("Part 1: {}", forest.number_of_visible_trees());
}

struct SpotLocator {
    heights: Vec<Vec<u32>>,
    distances: Vec<Vec<[u32; 4]>>,
    n_rows: usize,
    n_cols: usize,
}

impl SpotLocator{
    fn from(heights: Vec<Vec<u32>>) -> Self {
        let n_rows = heights.len();
        let n_cols = heights[0].len();
        let distances = vec![vec![[0; 4]; n_cols]; n_rows]; // (left, right, top, bottom)
        Self { heights, distances, n_rows, n_cols }
    }

    fn calculate_distances(&mut self) {
        for row_ind in 0..self.n_rows {
            for col_ind in 0..self.n_cols {
                self.update_element(row_ind, col_ind);
            }
        }

    }

    fn update_left_right(&mut self, row_ind: usize, col_ind: usize) {
        if col_ind == 0 {
            return
        }
        // left
        self.distances[row_ind][col_ind][0] = 1;
        let mut aux_ind = 1;
        while self.heights[row_ind][col_ind] > self.heights[row_ind][col_ind-aux_ind] {
            aux_ind += 1;
            if aux_ind > col_ind {
                break;
            }
            self.distances[row_ind][col_ind][0] += 1;
        }
        // right
        aux_ind = 1;
        let mod_col_ind = self.n_cols - 1 - col_ind;
        self.distances[row_ind][mod_col_ind][1] = 1;
        while self.heights[row_ind][mod_col_ind] > self.heights[row_ind][mod_col_ind+aux_ind] {
            aux_ind += 1;
            if aux_ind > col_ind {
                break;
            }
            self.distances[row_ind][mod_col_ind][1] += 1;
        }
    }

    fn update_top_bottom(&mut self, row_ind: usize, col_ind: usize) {
        if row_ind == 0 {
            return;
        }
        // top
        self.distances[row_ind][col_ind][2] = 1;
        let mut aux_ind = 1;
        while self.heights[row_ind][col_ind] > self.heights[row_ind-aux_ind][col_ind] {
            aux_ind += 1;
            if aux_ind > row_ind {
                break;
            }
            self.distances[row_ind][col_ind][2] += 1;
        }
        // bottom
        aux_ind = 1;
        let mod_row_ind = self.n_rows - 1 - row_ind;
        self.distances[mod_row_ind][col_ind][3] = 1;
        while self.heights[mod_row_ind][col_ind] > self.heights[mod_row_ind+aux_ind][col_ind] {
            aux_ind += 1;
            if aux_ind > row_ind {
                break;
            }
            self.distances[mod_row_ind][col_ind][3] += 1;
        }
    }

    fn update_element(&mut self, row_ind: usize, col_ind: usize) {
        self.update_left_right(row_ind, col_ind);
        self.update_top_bottom(row_ind, col_ind);
    }

    fn find_max_fov(&self) -> u32 {
        let mut max_prod = 0;
        for row in self.distances.iter() {
            for elem in row.iter() {
                let test = elem.iter().product();
                max_prod = std::cmp::max(max_prod, test);
            }
        }
        max_prod
    }
}

fn part2() {
    let reader = read_file();
    let mut data: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line_vector: Vec<u32> = line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect();
        data.push(line_vector);
    }
    let mut spot_locator = SpotLocator::from(data);
    spot_locator.calculate_distances();

    println!("Part 2: {}", spot_locator.find_max_fov());
}


fn main() {
    part1();
    part2();
}
