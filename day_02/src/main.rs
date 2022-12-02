use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    let encoder = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
        ("A", 1), // Piedra
        ("B", 2), // Papel
        ("C", 3)  // Tijeras
    ]);
    let result_encoder = HashMap::from([
                               (1, 6),
                               (-1, 0),
                               (0, 3)
    ]);
    for line in reader.lines() {
        let mut line = line.unwrap().split_whitespace();
        let other = encoder.get(line.next().unwrap()).unwrap();
        let mine = encoder.get(line.next().unwrap()).unwrap();
        result += result_encoder.get((mine - other)

    };
    println!("Part 1: {}", result);
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
