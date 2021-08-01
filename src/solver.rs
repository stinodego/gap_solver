use crate::config;
use crate::types::Assignment;
use crate::utils;
use std::collections::{BTreeMap, HashMap};

pub fn solve(config: config::SolverConfig) {
    let mut open_set = init_open_set(&config);
    let mut closed_set: BTreeMap<Assignment, f64> = BTreeMap::new();

    while !open_set.is_empty() {
        // Explore the most promising node
        let current = utils::max_key_by_value(&open_set).unwrap().clone();

        // Move current node from open set to the closet set
        let current_profit = open_set.remove(&current).unwrap();
        closed_set.insert(current, current_profit);

        println!("{:?}", closed_set)
    }
}

fn init_open_set(config: &config::SolverConfig) -> HashMap<Assignment, f64> {
    let mut open_set = HashMap::new();
    let start = config.assigned.clone();
    let start_profit = calc_profit(&start, &config.profit);
    open_set.insert(start, start_profit);
    open_set
}

/// Calculate the total profit for a given assignment
fn calc_profit(assignment: &Assignment, profit_map: &HashMap<(&str, &str), f64>) -> f64 {
    assignment
        .iter()
        .flat_map(|(agent, tasks)| {
            tasks
                .iter()
                .map(move |task| *profit_map.get(&(*agent, *task)).unwrap())
        })
        .sum()
}
