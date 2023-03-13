use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRECTIONS_TO_LOOK: [[[isize; 2]; 3]; 4] = [
    [[-1, -1], [0, -1], [1, -1]], // North
    [[-1, 1], [0, 1], [1, 1]],    // South
    [[-1, 1], [-1, 0], [-1, -1]], // West
    [[1, 1], [1, 0], [1, -1]],    // East
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    y: isize,
    x: isize,
}

impl Position {
    fn is_neighbour(&self, other: &Position) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff <= 1 && y_diff <= 1
    }

    fn clone(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn parse_input() -> Vec<Position> {
    // Returns a vec with the coordinates of the points
    let lines = read_file().lines();
    let mut result = Vec::new();
    for (row, line) in lines.enumerate() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                result.push(Position {
                    x: col as isize,
                    y: row as isize,
                });
            }
        }
    }
    result
}

fn move_ground(ground: Vec<Position>, round: usize) -> Result<Vec<Position>, String> {
    // Find proposals
    let mut proposed = Vec::new();
    let mut elf_index = Vec::new();
    for (index, elf) in ground.iter().enumerate() {
        let neighbours = find_neighbours(&ground, elf, index);
        if neighbours.len() == 0 {
            continue;
        }
        for dir_index in 0..4 {
            let directions = DIRECTIONS_TO_LOOK[(dir_index + round) % 4];
            let mut found = false;
            for direction in directions {
                let new_position = Position {
                    x: elf.x + direction[0],
                    y: elf.y + direction[1],
                };
                if neighbours.contains(&new_position) {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            let proposal = Position {
                x: elf.x + directions[1][0],
                y: elf.y + directions[1][1],
            };
            let sort_index = match proposed.binary_search(&proposal) {
                Ok(sort_index) => sort_index,
                Err(sort_index) => sort_index,
            };
            proposed.insert(sort_index, proposal);
            elf_index.insert(sort_index, index);
            break;
        }
    }
    // Remove bad proposals
    let (proposed, elf_index) = remove_duplicates(proposed, elf_index);

    if proposed.len() == 0 {
        return Err("No proposals".to_string());
    }
    return Ok(apply_moves(ground, proposed, elf_index));
}

fn remove_duplicates(
    mut proposed: Vec<Position>,
    mut elf_index: Vec<usize>,
) -> (Vec<Position>, Vec<usize>) {
    // Removes all the duplicates in the proposed vec
    if proposed.len() == 0 {
        return (proposed, elf_index);
    }
    let mut index = 0;
    while index < proposed.len() - 1 {
        if proposed[index] == proposed[index + 1] {
            let last_index = index + 1;
            while proposed[index] == proposed[last_index] {
                proposed.remove(last_index);
                elf_index.remove(last_index);
                if last_index == proposed.len() {
                    break;
                }
            }
            proposed.remove(index);
            elf_index.remove(index);
        } else {
            index += 1;
        }
    }
    (proposed, elf_index)
}

fn apply_moves(
    ground: Vec<Position>,
    proposed: Vec<Position>,
    elf_index: Vec<usize>,
) -> Vec<Position> {
    let mut new_ground = Vec::with_capacity(ground.len());
    for (index, elf) in ground.iter().enumerate() {
        let to_change_index = elf_index.iter().position(|&x| x == index);
        let to_insert = match to_change_index {
            Some(to_change_index) => proposed[to_change_index].clone(),
            None => elf.clone()
        };
        let sort_index = match new_ground.binary_search(&to_insert) {
            Ok(sort_index) => sort_index,
            Err(sort_index) => sort_index,
        };
        new_ground.insert(sort_index, to_insert);
    }
    new_ground
}

fn find_neighbours(ground: &Vec<Position>, point: &Position, index: usize) -> Vec<Position> {
    // Returns a vec with all the neighbours (in all directions) of the point found in the ground.
    // The idex is the index of the point in ground
    let mut result = Vec::with_capacity(8);
    if index != 0 {
        let mut left_index = index - 1;
        while ground[left_index].y >= point.y - 1 {
            if ground[left_index].is_neighbour(&point) {
                result.push(ground[left_index].clone());
            }
            if left_index == 0 {
                break;
            }
            left_index -= 1;
        }
    }
    if index != ground.len() - 1 {
        let mut right_index = index + 1;
        while ground[right_index].y <= point.y + 1 {
            if ground[right_index].is_neighbour(&point) {
                result.push(ground[right_index].clone());
            }
            right_index += 1;
            if right_index == ground.len() {
                break;
            }
        }
    }
    result
}

fn get_ground_limits(ground: &Vec<Position>) -> ((isize, isize), (isize, isize)) {
    let mut xlims = (ground[0].x, ground[0].x);
    let mut ylims = (ground[0].y, ground[0].y);
    for point in ground.iter() {
        if point.x < xlims.0 {
            xlims.0 = point.x;
        }
        if point.x > xlims.1 {
            xlims.1 = point.x;
        }
        if point.y < ylims.0 {
            ylims.0 = point.y;
        }
        if point.y > ylims.1 {
            ylims.1 = point.y;
        }
    }
    (xlims, ylims)
}

fn empty_tiles(ground: &Vec<Position>) -> usize {
    let (xlims, ylims) = get_ground_limits(ground);
    return ((xlims.1 - xlims.0 + 1) * (ylims.1 - ylims.0 + 1)) as usize - ground.len();
}

fn print_ground(ground: &Vec<Position>) {
    let (xlims, ylims) = get_ground_limits(ground);
    for y in ylims.0..ylims.1 + 1 {
        for x in xlims.0..xlims.1 + 1 {
            let point = Position { x, y };
            if ground.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1() {
    let mut ground = parse_input();
    for round in 0..10 {
        ground = move_ground(ground, round).unwrap();
    }
    println!("Part 1: {}", empty_tiles(&ground));
}

fn part2() {
    let mut ground = parse_input();
    let mut round = 0;
    while let Ok(new_ground) = move_ground(ground, round) {
        ground = new_ground;
        round += 1;
    }
    println!("Part 2: {}", round+1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn remove_duplicates() {
        let proposed = vec![
            super::Position { x: 1, y: 1 },
            super::Position { x: 1, y: 1 },
            super::Position { x: 2, y: 2 },
        ];
        let elf_index = vec![0, 1, 2];
        let (proposed, elf_index) = super::remove_duplicates(proposed, elf_index);
        assert_eq!(proposed, vec![super::Position { x: 2, y: 2 }]);
        assert_eq!(elf_index, vec![2]);

        let proposed = vec![
            super::Position { x: 1, y: 1 },
            super::Position { x: 2, y: 1 },
            super::Position { x: 2, y: 2 },
            super::Position { x: 2, y: 2 },
        ];
        let elf_index = vec![0, 1, 2, 3];
        let (proposed, elf_index) = super::remove_duplicates(proposed, elf_index);
        assert_eq!(
            proposed,
            vec![
                super::Position { x: 1, y: 1 },
                super::Position { x: 2, y: 1 }
            ]
        );
        assert_eq!(elf_index, vec![0, 1]);
    }
}

fn main() {
    part1();
    part2();
}
