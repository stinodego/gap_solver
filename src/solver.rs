use crate::config::SolverConfig;
use crate::types::Assignment;
use crate::utils;
use std::collections::{BTreeMap, HashMap};

pub fn solve(config: SolverConfig) {
    let mut open_set = init_open_set(&config);
    let mut closed_set: BTreeMap<Assignment, f64> = BTreeMap::new();

    while !open_set.is_empty() {
        // Explore the most promising node
        let current = utils::max_key_by_value(&open_set).unwrap().clone();

        // Move current node from open set to the closet set
        let current_profit = open_set.remove(&current).unwrap();
        closed_set.insert(current, current_profit);

        // Determine next steps
        println!("{:?}", closed_set)
    }
}

/// Initialize set of assignments to explore
fn init_open_set(config: &SolverConfig) -> HashMap<Assignment, f64> {
    let mut open_set = HashMap::new();
    let start = config.assigned.clone();
    let start_profit = calc_profit(&start, config);
    open_set.insert(start, start_profit);
    open_set
}

/// Determine all possible new tasks for an agent for the given assignment
fn possible_tasks(
    agent: &'static str,
    assignment: &Assignment,
    config: &'static SolverConfig,
) -> Vec<&'static str> {
    // Calculate remaining budget
    let budget = calc_agent_budget(agent, assignment, config);
    if budget == 0.0 {
        return Vec::new();
    };

    config
        .tasks
        .iter()
        // Agent cannot be assigned to the same task twice
        .filter(|t| !assignment[agent].contains(*t))
        // Tasks within agent budget
        .filter(|t| config.agent_cost[&(agent, **t)] <= budget)
        // Tasks with enough budget for agent
        .filter(|t| config.agent_cost[&(agent, **t)] <= budget)
        .copied()
        .collect()
}

/// Calculate the total profit for a given assignment
fn calc_profit(assignment: &Assignment, config: &SolverConfig) -> f64 {
    assignment
        .iter()
        .flat_map(|(agent, tasks)| {
            tasks
                .iter()
                .map(move |task| *config.profit.get(&(*agent, *task)).unwrap())
        })
        .sum()
}

/// Calculate the remaining budget for an agent
fn calc_agent_budget(agent: &'static str, assignment: &Assignment, config: &SolverConfig) -> f64 {
    let tasks = assignment.get(agent).unwrap();
    let spent: f64 = tasks.iter().map(|t| config.agent_cost[&(agent, *t)]).sum();
    let total_budget = config.agent_budget[agent];
    total_budget - spent
}

/// Calculate the remaining budget for a task
fn calc_task_budget(task: &'static str, assignment: &Assignment, config: &SolverConfig) -> f64 {
    let agents: Vec<&str> = assignment
        .iter()
        .filter(|(a, tasks)| tasks.contains(task))
        .map(|(k, v)| *k)
        .collect();
    let spent: f64 = agents.iter().map(|a| config.task_cost[&(*a, task)]).sum();
    let total_budget = config.task_budget[task];
    total_budget - spent
}
