use std::fs::File;
use std::io::{BufRead, BufReader};


trait Component {
    fn size(&self) -> u32;
    fn conditional_size(&self) -> u32;
    fn push_large_enough_dirs(&self, possible_dirs: &mut Vec<u32>, space: u32);
    fn name(&self) -> &str;
    fn add(&mut self, component: Box<dyn Component>);
    fn add_at(&mut self, path: &Vec<String>, depth: usize, component: Box<dyn Component>);
    fn print(&self, depth: u32) -> String;
}

struct FileInfo {
    name: String,
    size: u32,
}

impl FileInfo {
    fn new(name: String, size: u32) -> Self {
        Self {name, size}
    }
}

impl Component for FileInfo {
    fn size(&self) -> u32 {
        self.size
    }

    fn conditional_size(&self) -> u32 {
        return 0;
    }

    fn push_large_enough_dirs(&self, possible_dirs: &mut Vec<u32>, space: u32) {
        return;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, component: Box<dyn Component>) {
        panic!("Not possible");
    }

    fn add_at(&mut self, path: &Vec<String>, depth: usize, component: Box<dyn Component>) {
        panic!("Not possible");
    }

    fn print(&self, depth: u32) -> String {
        let mut result = String::new();
        for _ in 0..depth {
            result += "  ";
        }
        result += "* ";
        result += &self.name;
        result += " - ";
        result += &self.size.to_string();
        result += "\n";
        result
    }
}

struct Directory {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {name, components: Vec::new()}
    }

    fn get_large_enough_dir_size(&self, needed_space: u32) -> u32 {
        let mut possible_dirs = Vec::new();
        self.push_large_enough_dirs(&mut possible_dirs, needed_space);
        return *possible_dirs.iter().min().unwrap()
    }
}

impl Component for Directory {
    fn size(&self) -> u32 {
        self.components.iter().map(|comp| comp.size()).sum()
    }

    fn conditional_size(&self) -> u32 {
        let mut size = self.size();
        if size > 100000 {
            size = 0;
        }
        for element in self.components.iter() {
            size += element.conditional_size();
        }
        return size;
    }

    fn push_large_enough_dirs(&self, possible_dirs: &mut Vec<u32>, space: u32) {
        let size = self.size();
        if size >= space {
            possible_dirs.push(size);
        }
        for element in self.components.iter() {
            element.push_large_enough_dirs(possible_dirs, space);
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    fn add_at(&mut self, path: &Vec<String>, depth: usize, component: Box<dyn Component>) {
        if path.len() == depth {
            self.add(component);
        } else {
            for comp in self.components.iter_mut() {
                if comp.name() == path[depth] {
                    comp.add_at(path, depth + 1, component);
                    return;
                }
            }
            println!("{:?}, {}, {}", path, depth, component.name());
            panic!();
        }
    }

    fn print(&self, depth: u32) -> String {
        let mut result = String::new();
        for _ in 0..depth {
            result += "  ";
        }
        result += "- ";
        result += &self.name;
        result += " (";
        result += &self.size().to_string();
        result += "):\n";
        for component in self.components.iter() {
            result += &component.print(depth + 1);
        }
        result
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn build_file_system() -> Directory {
    let reader = read_file();
    let mut file_system = Directory::new(String::from("/"));
    let mut current_path = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "$ cd /" {
            current_path.clear();
        } else if line == "$ ls" {
            continue;
        } else if line == "$ cd .." {
            current_path.pop();
        } else if line.starts_with("$ cd") {
            let new_dir = line.split_whitespace();
            let new_dir = new_dir.last().unwrap();
            current_path.push(String::from(new_dir));
        } else if line.starts_with("dir") {
            let new_component = Directory::new(String::from(line.split_whitespace().last().unwrap()));
            file_system.add_at(&current_path, 0, Box::new(new_component));
        } else {
            let mut line_sp = line.split_whitespace();
            let size: u32 = line_sp.next().unwrap().parse().unwrap();
            let name = line_sp.next().unwrap();
            let new_component = FileInfo::new(name.to_string(), size);
            file_system.add_at(&current_path, 0, Box::new(new_component));
        }
    }
    file_system
}


fn solutions() {
    let file_system = build_file_system();

    let result = file_system.conditional_size();
    println!("Part 1: {}", result);

    let space_to_free = 30000000 - (70000000 - file_system.size());
    let result = file_system.get_large_enough_dir_size(space_to_free);
    println!("Part 2: {}", result);
}


fn main() {
    solutions();
}
