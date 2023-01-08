use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap, HashMap};

enum Value {
    Int(i64),
    Operation(String, String, String),
}

struct Monkey {
    name: String,
    value: Value,
}

impl Monkey {
    fn new(name: String, value: Value) -> Monkey {
        Monkey { name, value }
    }

    fn get_value(&self, monkey_dict: &HashMap<String, Monkey>) -> i64 {
        match &self.value {
            Value::Int(i) => *i,
            Value::Operation(monkey1, monkey2, operation) => {
                let monkey1 = monkey_dict.get(monkey1).unwrap();
                let monkey2 = monkey_dict.get(monkey2).unwrap();
                let monkey1_value = monkey1.get_value(monkey_dict);
                let monkey2_value = monkey2.get_value(monkey_dict);
                match operation.as_str() {
                    "*" => monkey1_value * monkey2_value,
                    "+" => monkey1_value + monkey2_value,
                    "-" => monkey1_value - monkey2_value,
                    "/" => monkey1_value / monkey2_value,
                    _ => panic!("Unknown operation"),
                }
            }
        }
    }

    fn inverse(&self, monkey_dict: &HashMap<String, Monkey>, parent_dict: &HashMap<String, String>) -> i64 {
        let parent_name = parent_dict.get(&self.name).unwrap();
        let parent = monkey_dict.get(parent_name).unwrap();
        let (other_child_name, current_is_left) = match &parent.value {
            Value::Operation(monkey1, monkey2, _) => {
                if monkey1 == &self.name {
                    (monkey2, true)
                } else {
                    (monkey1, false)
                }
            },
            _ => panic!("Parent is not an operation"),
        };
        if parent_name == "root" {
            return monkey_dict.get(other_child_name).unwrap().get_value(monkey_dict);
        }
        let other_child = monkey_dict.get(other_child_name).unwrap();
        match &parent.value {
            Value::Int(_) => panic!("Parent is not an operation"),
            Value::Operation(_, _, operation) => {
                let parent_inverse = parent.inverse(monkey_dict, parent_dict);
                let other_child_value = other_child.get_value(monkey_dict);
                if current_is_left {
                    match operation.as_str() {
                        "*" => parent_inverse / other_child_value,
                        "+" => parent_inverse - other_child_value,
                        "-" => parent_inverse + other_child_value,
                        "/" => parent_inverse * other_child_value,
                        _ => panic!("Unknown operation"),
                    }
                } else {
                    match operation.as_str() {
                        "*" => parent_inverse / other_child_value,
                        "+" => parent_inverse - other_child_value,
                        "-" => other_child_value - parent_inverse,
                        "/" => other_child_value / parent_inverse,
                        _ => panic!("Unknown operation"),
                    }
                }
            }
        }
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn parse_monkeys() -> (HashMap<String, Monkey>, HashMap<String, String>) {
    let mut monkey_dict = HashMap::new();
    let mut parent_dict = HashMap::new();
    let file = read_file();
    for line in file.lines() {
        let line = line.unwrap();
        let (monkey_name, value) = line.split_once(": ").unwrap();
        let values: Vec<&str> = value.split(" ").collect();
        if values.len() == 1 {
            let monkey = Monkey::new(monkey_name.to_string(), Value::Int(values[0].parse().unwrap()));
            monkey_dict.insert(monkey_name.to_string(), monkey);
        } else {
            let monkey = Monkey::new(monkey_name.to_string(), Value::Operation(values[0].to_string(), values[2].to_string(), values[1].to_string()));
            monkey_dict.insert(monkey_name.to_string(), monkey);
            parent_dict.insert(values[0].to_string(), monkey_name.to_string());
            parent_dict.insert(values[2].to_string(), monkey_name.to_string());
        }
    }
    (monkey_dict, parent_dict)
}


fn part1() {
    let (monkey_dict, _) = parse_monkeys();
    let root_monkey = monkey_dict.get("root").unwrap();

    println!("Part 1: {}", root_monkey.get_value(&monkey_dict));
}


fn part2() {
    let (monkey_dict, parent_dict) = parse_monkeys();
    let humn_monkey = monkey_dict.get("humn").unwrap();

    println!("Part 2: {}", humn_monkey.inverse(&monkey_dict, &parent_dict));
}


fn main() {
    part1();
    part2();
}
