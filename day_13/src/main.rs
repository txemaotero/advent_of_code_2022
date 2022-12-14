use std::fs::File;
use std::io::{BufRead, BufReader};


trait Element {
    fn is_smaller(&self, other: Box<dyn Element>) -> bool;
    fn is_equal(&self, other: Box<dyn Element>) -> bool;
    fn len(&self) -> Option<usize>;
    fn element(&self, index: usize) -> Option<Box<dyn Element>>;
    fn is_leaf(&self) -> bool;
    fn value(&self) -> Option<u32>;
    fn clone(&self) -> Box<dyn Element>;
    fn print(&self) -> String;
}

struct ListOfElements {
    elements: Vec<Box<dyn Element>>,
}

impl ListOfElements {
    fn new(elements: Vec<Box<dyn Element>>) -> Self {
        Self { elements }
    }

    fn from_leaf(leaf_value: u32) -> Self {
        Self {
            elements: vec![Box::new(ElementLeaf {value: leaf_value})],
        }
    }

    fn from(text_line: &str) -> Self {
        let mut elements = Self::new(Vec::new());
        let mut depth = 0;
        let mut start_index = 1;
        while start_index < (text_line.len() - 1) {
            let mut ch = text_line.chars().nth(start_index).unwrap();
            if ch == '[' {
                depth += 1;
                let mut end_index = start_index;
                while depth > 0 {
                    end_index += 1;
                    let ch = text_line.chars().nth(end_index).unwrap();
                    if ch == '[' {
                        depth += 1;
                    } else if ch == ']' {
                        depth -= 1;
                    }
                }
                elements.elements.push(Box::new(ListOfElements::from(&text_line[start_index..end_index+1])));
                start_index = end_index;
            } else if ch.is_ascii_digit() {
                // Careful: there may be more than one digit together
                let mut number = String::new();
                while ch.is_ascii_digit() {
                    number.push(ch);
                    start_index += 1;
                    ch = text_line.chars().nth(start_index).unwrap();
                }
                elements.elements.push(Box::new(ElementLeaf {value: number.parse::<u32>().unwrap()}));
                start_index -= 1;
            } else {
            }
            start_index += 1;
        }
        elements
    }
}

impl Element for ListOfElements {
    fn len(&self) -> Option<usize> {
        Some(self.elements.len())
    }

    fn element(&self, index: usize) -> Option<Box<dyn Element>> {
        if index >= self.elements.len() {
            return None
        }
        Some(self.elements[index].clone())
    }

    fn is_leaf(&self) -> bool {
        false
    }

    fn value(&self) -> Option<u32> {
        None
    }

    fn is_smaller(&self, other: Box<dyn Element>) -> bool {
        if other.is_leaf() {
            return self.is_smaller(Box::new(Self::from_leaf(other.value().unwrap())));
        }
        let self_len = self.len().unwrap();
        let other_len = other.len().unwrap();
        let mut index = 0;
        while index < std::cmp::min(self_len, other_len) {
            let self_el = self.element(index).unwrap();
            let other_el = other.element(index).unwrap();
            if self_el.is_equal(other_el.clone()) {
                index += 1;
                continue;
            }
            return self_el.is_smaller(other_el);
        }
        if self_len == other_len {
            panic!("DRAW, not possible");
        }
        return self_len < other_len;
    }

    fn is_equal(&self, other: Box<dyn Element>) -> bool {
        if other.is_leaf() {
            let aux = ListOfElements::from_leaf(other.value().unwrap());
            return self.is_equal(Box::new(aux));
        }

        let self_len = self.len().unwrap();
        let other_len = other.len().unwrap();
        if self_len != other_len {
            return false;
        }
        for i in 0..self_len {
            if !self.element(i).unwrap().is_equal(other.element(i).unwrap()) {
                return false;
            }
        }
        true
    }

    fn clone(&self) -> Box<dyn Element> {
        let mut new = Self::new(Vec::new());
        for index in 0..self.elements.len() {
            new.elements.push(self.elements[index].clone())
        }
        Box::new(new)
    }

    fn print(&self) -> String {
        let mut result = String::new();
        result.push('[');
        for index in 0..self.elements.len() {
            result.push_str(&self.elements[index].print());
            if index < self.elements.len() - 1 {
                result.push(',');
            }
        }
        result.push(']');
        result
    }
}

struct ElementLeaf {
    value: u32
}

impl Element for ElementLeaf {
    fn len(&self) -> Option<usize> {
        None
    }

    fn clone(&self) -> Box<dyn Element> {
        Box::new(Self {value: self.value})
    }

    fn element(&self, index: usize) -> Option<Box<dyn Element>> {
        None
    }

    fn is_leaf(&self) -> bool {
        true
    }

    fn value(&self) -> Option<u32> {
        Some(self.value)
    }

    fn is_smaller(&self, other: Box<dyn Element>) -> bool {
        if other.is_leaf() {
            self.value().unwrap() < other.value().unwrap()
        } else {
            let aux = ListOfElements::from_leaf(self.value);
            aux.is_smaller(other)
        }
    }

    fn is_equal(&self, other: Box<dyn Element>) -> bool {
        if other.is_leaf() {
            self.value().unwrap() == other.value().unwrap()
        } else {
            let aux = ListOfElements::from_leaf(self.value);
            aux.is_equal(other)
        }
    }

    fn print(&self) -> String {
        self.value.to_string()
    }
}

impl std::cmp::PartialEq for ListOfElements {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other.clone())
    }

    fn ne(&self, other: &Self) -> bool {
        !self.is_equal(other.clone())
    }
}


impl std::cmp::Eq for ListOfElements {}


impl std::cmp::PartialOrd for ListOfElements {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_equal(other.clone()) {
            Some(std::cmp::Ordering::Equal)
        } else if self.is_smaller(other.clone()) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}


impl std::cmp::Ord for ListOfElements {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_smaller(other.clone()) {
            std::cmp::Ordering::Less
        } else if self.is_equal(other.clone()) {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut pair: Vec<ListOfElements> = Vec::new();
    let mut result = 0;
    for (li, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            let pc = pair[1].clone();
            if pair[0].is_smaller(pc) {
                result += (li + 1)/3 ;
            }
            pair.clear();
            continue;
        }
        let elements = ListOfElements::from(&line);
        assert_eq!(elements.print(), line);
        pair.push(elements);
    }
    println!("Part 1: {}", result);
}


fn part2() {
    let reader = read_file();
    let mut all_elements: Vec<ListOfElements> = Vec::new();
    all_elements.push(ListOfElements::from("[[2]]"));
    all_elements.push(ListOfElements::from("[[6]]"));

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let elements = ListOfElements::from(&line);
        all_elements.push(elements);
    }
    all_elements.sort();
    let mut result = 1;
    for index in 0..all_elements.len() {
        let current = all_elements[index].clone();
        if current.is_equal(Box::new(ListOfElements::from("[[2]]"))) | current.is_equal(Box::new(ListOfElements::from("[[6]]"))) {
            result *= index + 1;
        }
    }
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
