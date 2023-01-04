use std::{io::{BufReader, BufRead}, fs::File};

use highs::{Sense, RowProblem, SolvedModel};


fn get_index(kind: &str) -> usize {
    ["ore", "clay", "obsidian", "geode"].iter().position(|&x| x == kind).unwrap()
}


fn parse_line(recipe: &str) -> (usize, [[usize; 4]; 4]) {
    let (id_part, cost_part) = recipe.split_once(":").unwrap();
    let id = id_part.split_once(" ").unwrap().1.parse::<usize>().unwrap();
    let sentences = cost_part.trim().split(". ");
    let mut costs = [[0; 4]; 4];
    for s in sentences {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        let cost_kind_index = get_index(split[1]);

        let costs_end_split = &split[4..];

        let cost_key = get_index(costs_end_split[1]);
        let cost_val = costs_end_split[0].parse::<usize>().unwrap();

        costs[cost_kind_index][cost_key] = cost_val;

        if costs_end_split.len() != 2 {
            let cost_key = get_index(costs_end_split[4]);
            let cost_val = costs_end_split[3].parse::<usize>().unwrap();
            costs[cost_kind_index][cost_key] = cost_val;
        }
    }
    (id, costs)
}


fn solve_problem(var_coefs: &Vec<f64>, constraint_matrix: &Vec<Vec<f64>>, upper_limits: &Vec<f64>) -> SolvedModel {
    let mut pb = RowProblem::default();

    let vars = var_coefs
        .iter()
        .map(|&coef| pb.add_integer_column(coef, 0..=1))
        .collect::<Vec<_>>();

    constraint_matrix
        .iter()
        .zip(upper_limits.iter())
        .for_each(|(const_row, &limit)| {
        let row_factors = const_row
            .iter()
            .zip(vars.iter())
            .map(|(&coef, &var)| (var, coef))
            .collect::<Vec<_>>();
        pb.add_row(..=limit, &row_factors);
    });
    pb.optimise(Sense::Maximise).solve()
}

fn get_problem_input(costs: [[usize; 4]; 4], minutes: usize) -> (Vec<f64>, Vec<Vec<f64>>, Vec<f64>) {
    let mut var_coefs = vec![0.0; minutes*4];
    let mut constraint_matrix = vec![vec![0.0; minutes*4]; minutes*4];
    let mut upper_limits = vec![0.0; minutes*4];

    for constraint_index in 0..minutes*4 {
        var_coefs[constraint_index] = - (costs[constraint_index % 4][3] as f64);
        if constraint_index % 4 == 3 {
            var_coefs[constraint_index] -= (constraint_index / 4) as f64 - minutes as f64;
        } else if constraint_index % 4 == 0 {
            upper_limits[constraint_index] = (constraint_index / 4) as f64;
        }
        for var_index in 0..minutes*4 {
            let gamma = var_index % 4;
            let beta = constraint_index % 4;
            let k = constraint_index / 4 + 1;
            let l = var_index / 4 + 1;
            if l > k {
                continue;
            } else if (l >= k - 1) | (gamma != beta) {
                constraint_matrix[constraint_index][var_index] = costs[gamma][beta] as f64;
            } else if gamma == beta {
                constraint_matrix[constraint_index][var_index] = costs[gamma][beta] as f64 - k as f64 + 1. + l as f64;
            }
        }
    }
    for minute in 0..minutes {
        let mut aux_only_one_robot_const = vec![0.0; minutes*4];
        for kind in 0..4 {
            aux_only_one_robot_const[minute*4 + kind] = 1.0;
        }
        constraint_matrix.push(aux_only_one_robot_const);
        upper_limits.push(1.0);
    }
    (var_coefs, constraint_matrix, upper_limits)
}


fn read_file() -> BufReader<File> {
    let file = File::open("../input.txt").unwrap();
    BufReader::new(file)
}


fn part1() {
    let reader = read_file();
    let mut result = 0;
    for line in reader.lines() {
        let recipe = line.unwrap();
        let (id, costs) = parse_line(&recipe[..recipe.len()-1]);
        let (var_coefs, constraint_matrix, upper_limits) = get_problem_input(costs, 23);

        let solved = solve_problem(&var_coefs, &constraint_matrix, &upper_limits);

        let solution = solved.get_solution();

        let max_geodes: f64 = solution.columns().iter().zip(var_coefs.iter()).map(|(&col, &coef)| col * coef).sum();
        result += (max_geodes as usize) * id;
    }
    println!("Part 1: {}", result);
}


fn part2() {
    let reader = read_file();
    let mut result = 1;
    let mut counter = 0;
    for line in reader.lines() {
        if counter == 3 {
            break;
        }
        counter += 1;
        let recipe = line.unwrap();
        let (_, costs) = parse_line(&recipe[..recipe.len()-1]);
        let (var_coefs, constraint_matrix, upper_limits) = get_problem_input(costs, 31);

        let solved = solve_problem(&var_coefs, &constraint_matrix, &upper_limits);
        let solution = solved.get_solution();

        let max_geodes: f64 = solution.columns().iter().zip(var_coefs.iter()).map(|(&col, &coef)| col * coef).sum();
        result *= max_geodes.round() as usize;
    }
    println!("Part 2: {}", result);
}


fn main() {
    part1();
    part2();
}
