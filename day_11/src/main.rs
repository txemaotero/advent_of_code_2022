use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


struct Item {
    worry_level: u64,
    worry_level_reminders: HashMap<u64, u64>,
}

impl Item {
    fn new(worry_level: u64) -> Self {
        Self { worry_level, worry_level_reminders: HashMap::new() }
    }

    fn set_reminders(&mut self, check_values: &Vec<u64>) {
        for value in check_values {
            self.worry_level_reminders.insert(*value, self.worry_level % value);
        }
    }

    fn relax(&mut self) {
        self.worry_level = self.worry_level / 3;
    }
}

struct Throw {
    divissible_by: u64,
    monkey_id_true: usize,
    monkey_id_false: usize,
}

impl Throw {
    fn new() -> Self {
        Self {divissible_by: 1, monkey_id_true: 0, monkey_id_false: 0}
    }

    fn to_monkey(&self, item: &Item, real: bool) -> usize {
        let reminder = if real {
            item.worry_level % self.divissible_by as u64
        } else {
            *item.worry_level_reminders.get(&(*&self.divissible_by as u64)).unwrap()
        };
        if reminder == 0 {
            self.monkey_id_true
        } else {
            self.monkey_id_false
        }
    }
}

struct Operation {
    operand: usize,
    second_term: Option<u64>,
}

impl Operation {
    fn new() -> Self {
        Self { operand: 0, second_term: None }
    }

    fn from(operand: &str, second_term: &str) -> Self {
        let operand = if operand == "+" {
            0
        } else if operand == "*" {
            1
        } else {
            panic!("Error")
        };
        let second_term = if second_term == "old" {
            None
        } else {
            Some(second_term.parse().unwrap())
        };
        Self {operand, second_term}
    }

    fn apply(&self, reminders: &mut HashMap<u64, u64>) {
        for (k, v) in reminders.iter_mut() {
            *v = match self.second_term {
                None => {
                    ((*v)*(*v)) % k
                }
                Some(s) => {
                    if self.operand == 0 {
                        ((s % k) + (*v)) % k
                    } else {
                        ((s % k) * (*v)) % k
                    }
                }
            };
        }
    }

    fn apply_real(&self, first_term: u64) -> u64 {
        match self.second_term {
            None => {
                first_term * first_term
            }
            Some(s) => {
                if self.operand == 0 {
                    first_term + s
                } else {
                    first_term * s
                }
            }
        }
    }
}

struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test_throw: Throw,
    inspected_items: u64,
}

impl Monkey {

    fn new() -> Self {
        Self {items: Vec::new(), operation: Operation::new(), test_throw: Throw::new(), inspected_items: 0}
    }

    fn process_item(&mut self, relax: bool) -> Option<(usize, Item)> {
        let mut item = self.items.pop()?;
        if relax {
            item.worry_level = self.operation.apply_real(item.worry_level);
            item.relax();
        } else {
            self.operation.apply(&mut item.worry_level_reminders);
        }
        let to_monkey = self.test_throw.to_monkey(&item, relax);
        self.inspected_items += 1;
        Some((to_monkey, item))
    }
}


fn iterate_monkeys(mut monkeys: Vec<Monkey>, cycles: usize, relax: bool) -> Vec<Monkey> {
    for _ in 0..cycles {
        for monkey_ind in 0..monkeys.len() {
            loop {
                let (monkey_to, item) = match monkeys[monkey_ind].process_item(relax) {
                    None => break,
                    Some(a) => a,
                };
                monkeys[monkey_to].items.push(item);
            }
        }
    }
    monkeys
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}

fn load_monkeys() -> Vec<Monkey> {
    let reader = read_file();
    let mut monkeys = vec![Monkey::new()];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            monkeys.push(Monkey::new());
            continue;
        }
        let line_trim = line.trim();
        if line_trim.starts_with("Starting") {
            let mut items: Vec<_> = line_trim
                .split_once(":").unwrap().1
                .split(",")
                .map(|x| Item::new(x.trim().parse().unwrap()))
                .collect();
            items.reverse();
            monkeys.last_mut().unwrap().items = items;
        } else if line_trim.starts_with("Monkey") {
            continue;
        } else if line_trim.starts_with("Operation") {
            let terms = line_trim.split_once("=").unwrap().1
                .trim()
                .split_whitespace()
                .collect::<Vec<_>>();
            // first term is always old
            monkeys.last_mut().unwrap().operation = Operation::from(terms[1], terms[2]);
        } else if line_trim.starts_with("Test") {
            monkeys.last_mut().unwrap().test_throw.divissible_by = line_trim
                .split_whitespace().last().unwrap().parse().unwrap();
        } else if line_trim.starts_with("If true") {
            monkeys.last_mut().unwrap().test_throw.monkey_id_true = line_trim
                .split_whitespace().last().unwrap().parse().unwrap();
        } else if line_trim.starts_with("If false") {
            monkeys.last_mut().unwrap().test_throw.monkey_id_false = line_trim
                .split_whitespace().last().unwrap().parse().unwrap();
        } else {
            panic!("Wrong line");
        }
    }
    let values: Vec<_> = monkeys.iter()
        .map(|m| m.test_throw.divissible_by)
        .collect();
    for monkey in monkeys.iter_mut() {
        for item in monkey.items.iter_mut() {
            item.set_reminders(&values);
        }
    }
    monkeys
}

fn get_result(monkeys: Vec<Monkey>) -> u64 {
    let mut inspected_items: Vec<u64> = monkeys.iter().map(|m| m.inspected_items).collect();
    inspected_items.sort();
    inspected_items.reverse();
    inspected_items[0] as u64 * inspected_items[1] as u64
}


fn part1() {
    let mut monkeys = load_monkeys();
    monkeys = iterate_monkeys(monkeys, 20, true);
    let result = get_result(monkeys);
    println!("Part 1: {}", result);
}

fn part2() {
    let mut monkeys = load_monkeys();
    monkeys = iterate_monkeys(monkeys, 10000, false);
    let result = get_result(monkeys);
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
