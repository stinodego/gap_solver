use crate::assignment::Assignment;
use crate::config::SolverConfig;
use crate::utils;
use std::collections::HashMap;

pub fn solve<'a>(config: &'a SolverConfig) -> Vec<Assignment<'a>> {
    let mut open_set: HashMap<Assignment, i64> = init_open_set(config);
    let mut closed_set: HashMap<Assignment, i64> = HashMap::new();
    let mut finished_set: Vec<Assignment> = Vec::new();

    while !open_set.is_empty() {
        // Explore the most promising node
        let current = utils::max_key_by_value(&open_set).unwrap().clone();

        // Remove current node from open set
        let current_profit = open_set.remove(&current).unwrap();

        // Determine all possible next assignments
        match expand_node(&current, config, &mut open_set, &closed_set) {
            Ok(_) => {}
            Err(_) => {
                finished_set.push(current.clone());
            }
        }

        // Move node to the closed set
        closed_set.insert(current, current_profit);
    }
    finished_set
}

/// Initialize set of assignments to explore
fn init_open_set<'a>(config: &'a SolverConfig) -> HashMap<Assignment<'a>, i64> {
    let mut open_set = HashMap::new();
    let start = Assignment::new(config);
    let start_profit = start.profit();
    open_set.insert(start, start_profit);
    open_set
}

/// Determine all possible new tasks for an agent for the given assignment
fn expand_node<'a>(
    assignment: &Assignment<'a>,
    config: &'a SolverConfig,
    open_set: &mut HashMap<Assignment<'a>, i64>,
    closed_set: &HashMap<Assignment, i64>,
) -> Result<(), &'a str> {
    let mut finished = true;
    for agent in &config.agents {
        // Determine agent budget
        let agent_budget = assignment.agent_budget(agent);
        if agent_budget == 0 {
            continue;
        };
        // Determine all possible tasks for the agent
        let assigned = assignment.agent_tasks(agent);
        let possible_tasks = config
            .tasks
            .iter()
            // Agent cannot be assigned to the same task twice
            .filter(|t| match assigned {
                None => true,
                Some(tasks) => !tasks.contains(*t),
            })
            // Tasks within agent budget
            .filter(|t| config.agent_cost[&(*agent, **t)] <= agent_budget)
            // Tasks with enough budget for agent
            .filter(|t| config.task_cost[&(*agent, **t)] <= assignment.task_budget(t))
            .copied();

        // Create assignments for each task
        for t in possible_tasks {
            finished = false;
            let mut next = assignment.clone();
            next.assign(agent, t).unwrap();
            if !closed_set.contains_key(&next) {
                let profit = next.profit();
                open_set.insert(next, profit);
            }
        }
    }
    if finished {
        Err("Assignment is finished and cannot be expanded.")
    } else {
        Ok(())
    }
}
