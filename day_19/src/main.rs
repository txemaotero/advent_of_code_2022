use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: HashMap<String, HashMap<String, usize>>,
    max_costs: HashMap<String, usize>,
}

impl Blueprint {
    fn new(recipe: &str) -> Blueprint {
        let mut costs = HashMap::new();
        let mut max_costs = HashMap::new();
        let (id_part, cost_part) = recipe.split_once(":").unwrap();
        let id = id_part.split_once(" ").unwrap().1.parse::<usize>().unwrap();
        let sentences = cost_part.trim().split(". ");
        for s in sentences {
            let split = s.split_whitespace().collect::<Vec<&str>>();
            let key = split[1].to_string();
            let mut aux_map =  HashMap::new();
            let costs_end = &split[4..];
            let cost_key = costs_end[1].to_string();
            let cost_val = costs_end[0].parse::<usize>().unwrap();
            let aux_cost_val = max_costs.entry(cost_key.clone()).or_insert(cost_val);
            if cost_val > *aux_cost_val {
                *aux_cost_val = cost_val;
            }
            aux_map.insert(cost_key, cost_val);
            if costs_end.len() != 2 {
                let cost_key = costs_end[4].to_string();
                let cost_val = costs_end[3].parse::<usize>().unwrap();
                let aux_cost_val = max_costs.entry(cost_key.clone()).or_insert(cost_val);
                if cost_val > *aux_cost_val {
                    *aux_cost_val = cost_val;
                }
                aux_map.insert(cost_key, cost_val);
            }
            costs.insert(key, aux_map);
        }
        max_costs.insert("geode".to_string(), usize::MAX);

        Blueprint {
            id,
            costs,
            max_costs,
        }
    }

    fn clone(&self) -> Blueprint {
        Blueprint {
            id: self.id,
            costs: self.costs.clone(),
            max_costs: self.max_costs.clone(),
        }
    }

}

struct State {
    storage: HashMap<String, usize>,
    minutes_left: usize,
    blueprint: Blueprint,
    robots: HashMap<String, usize>
}

impl State {
    fn new(blueprint: Blueprint) -> State {
        State {
            storage: HashMap::from([
                                   ("ore".to_string(), 0),
                                   ("clay".to_string(), 0),
                                   ("obsidian".to_string(), 0),
                                   ("geode".to_string(), 0)
            ]),
            minutes_left: 24,
            blueprint,
            robots: HashMap::from([
                                   ("ore".to_string(), 1),
                                   ("clay".to_string(), 0),
                                   ("obsidian".to_string(), 0),
                                   ("geode".to_string(), 0)
            ]),
        }
    }

    fn pass_one_minute(&mut self) {
        self.minutes_left -= 1;
        for (key, value) in self.robots.iter_mut() {
            let store = self.storage.get_mut(key).unwrap();
            *store += *value;
        }
    }

    fn build_robots(&mut self, robots: &HashMap<String, usize>) -> Result<(), ()> {
        let mut total_costs = HashMap::new();
        for (robot_kind, robot_ammount) in robots.iter() {
            let costs = self.blueprint.costs.get(robot_kind).unwrap();
            for (cost_kind, cost_ammount) in costs.iter() {
                let total_cost = total_costs.entry(cost_kind.to_string()).or_insert(0);
                *total_cost += cost_ammount * robot_ammount;
            }
        }
        for (cost_kind, cost_ammount) in total_costs.iter() {
            let store = self.storage.get(cost_kind).unwrap();
            if store < cost_ammount {
                return Err(());
            }
        }
        self.pass_one_minute();
        for (robot_kind, robot_ammount) in robots.iter() {
            let robot = self.robots.get_mut(robot_kind).unwrap();
            *robot += robot_ammount;
        }
        Ok(())
    }

    fn clone(&self) -> State {
        State {
            storage: self.storage.clone(),
            minutes_left: self.minutes_left,
            blueprint: self.blueprint.clone(),
            robots: self.robots.clone(),
        }
    }

    fn valid_after_build(&self) -> bool {
        let mut counter = 0;
        // si con lo que me queda puedo construir un robot de cada tipo, continuar
        for key in ["ore", "clay", "obsidian"].iter() {
            let key = key.to_string();
            let store = self.storage.get(&key).or(Some(&0)).unwrap();
            let max_cost = self.blueprint.max_costs.get(&key).unwrap();
            if store > max_cost {
                counter += 1;
            }
        }
        // println!("counter: {}", counter);
        counter < 3
    }

}


fn get_all_possible_builds(state: &State) -> Vec<HashMap<String, usize>> {
    let mut resources = [0; 4];
    let mut costs = [[0;4]; 4];
    let resource_to_index = HashMap::from([
                                          ("ore".to_string(), 0),
                                          ("clay".to_string(), 1),
                                          ("obsidian".to_string(), 2),
                                          ("geode".to_string(), 3)
    ]);
    for (key, value) in state.storage.iter() {
        let index = resource_to_index.get(key).unwrap();
        resources[*index] = *value;
        let cost = state.blueprint.costs.get(key).unwrap();
        for (cost_key, cost_value) in cost.iter() {
            let cost_index = resource_to_index.get(cost_key).unwrap();
            costs[*index][*cost_index] = *cost_value;
        }
    }
    let mut possible_builds = Vec::new();
    get_all_builds_indexes(&mut possible_builds, &resources, &costs, 0, &mut HashMap::new());
    let result = possible_builds.iter().map(|build| {
        let mut build_map = HashMap::new();
        for (kind_id, ammount) in build.iter() {
            let kind = resource_to_index.iter().find(|(_, v)| **v == *kind_id).unwrap().0;
            build_map.insert(kind.to_string(), *ammount);
        }
        build_map
    }).collect::<Vec<HashMap<String, usize>>>();

    result
}


fn filter_possible_builds(possible_builds: Vec<HashMap<usize, usize>>, resources: &[usize; 4], costs: &[[usize; 4]; 4]) -> Vec<HashMap<usize, usize>> {
    let mut max_costs = [0, 0, 0, usize::MAX];
    for cost in costs.iter() {
        for (i, c) in cost.iter().enumerate() {
            if *c > max_costs[i] {
                max_costs[i] = *c;
            }
        }
    }
    let mut result = Vec::new();
    for build in possible_builds.iter() {
        let mut build_cost = [0, 0, 0, 0];
        for (kind, ammount) in build.iter() {
            for (i, c) in costs[*kind].iter().enumerate() {
                build_cost[i] += c * ammount;
            }
        }
        let cannot_build = (0..4).any(|i| resources[i] >= (max_costs[i] + build_cost[i]));
        if !cannot_build {
            result.push(build.clone());
        }
    }
    result

}

fn get_all_builds_indexes(possible_builds: &mut Vec<HashMap<usize, usize>>,
                          resources: &[usize; 4],
                          costs: &[[usize; 4]; 4],
                          index: usize,
                          aux_build: &mut HashMap<usize, usize>) {
    let mut max_robots = 0;
    let aux_cost = costs[index];
    let mut aux_resources: [isize; 4]= [resources[0] as isize, resources[1] as isize, resources[2] as isize, resources[3] as isize];
    while aux_resources.iter().all(|v| v >= &0) {
        for i in 0..4 {
            aux_resources[i] -= aux_cost[i] as isize;
        }
        max_robots += 1;
    }

    for n_robots in 0..max_robots {
        let mut aux_resources = resources.clone();
        for i in 0..4 {
            aux_resources[i] -= n_robots * aux_cost[i];
        }
        aux_build.insert(index, n_robots);
        if index == 3 {
            possible_builds.push(aux_build.clone());
        } else {
            get_all_builds_indexes(possible_builds, &aux_resources, costs, index + 1, aux_build);
        }
    }
}

fn find_max_geode(mut state: State, max_geode: &mut usize, builds: &HashMap<String, usize>) {
    match state.build_robots(builds) {
        Ok(_) => {},
        Err(_) => panic!("Cannot build robots"),
    };
    // if !state.valid_after_build() {
        // println!("Invalid state, storage {:?}", state.storage);
        // println!("Max costs {:?}", state.blueprint.max_costs);
        // return
    // }

    let to_chech_contains = ["ore", "clay", "obsidian", "geode"].iter().map(|s| HashMap::from([(s.to_string(), 1)])).collect::<Vec<HashMap<String, usize>>>();
    if state.minutes_left == 0 {
        let geode = state.storage.get("geode").unwrap();
        if geode > max_geode {
            *max_geode = *geode;
            println!("New max geode: {}", max_geode);
        }
    } else {
        let possible_builds = get_all_possible_builds(&state);
        if to_chech_contains.iter().all(|e| possible_builds.contains(e)) {
            println!("Applied filter");
            return
        }
        // println!("possible builds: {:?}", possible_builds);
        for build in possible_builds.iter().rev() {
            find_max_geode(state.clone(), max_geode, build);
        }
    }
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    let file = File::open("../example.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    for line in reader.lines() {
        let recipe = line.unwrap();
        let blueprint = Blueprint::new(&recipe[..recipe.len()-1]);
        let state = State::new(blueprint);
        let mut max_geode = 0;
        let id = state.blueprint.id;
        find_max_geode(state, &mut max_geode, &HashMap::new());
        println!("Max geodes {:?}", max_geode);
        result += max_geode * id;
    }
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
