use crate::assignment::Assignment;
use crate::config::SolverConfig;
use crate::utils;
use std::collections::HashMap;

pub fn solve(config: &SolverConfig) {
    let mut open_set: HashMap<Assignment, i64> = init_open_set(config);
    let mut closed_set: HashMap<Assignment, i64> = HashMap::new();

    while !open_set.is_empty() {
        // Explore the most promising node
        let current = utils::max_key_by_value(&open_set).unwrap().clone();

        // Move current node from open set to the closet set
        let current_profit = open_set.remove(&current).unwrap();
        closed_set.insert(current, current_profit);

        // Determine next steps
        println!("{:#?}", closed_set)
    }
}

/// Initialize set of assignments to explore
fn init_open_set<'a>(config: &'a SolverConfig) -> HashMap<Assignment<'a>, i64> {
    let mut open_set = HashMap::new();
    let start = Assignment::new(config);
    let start_profit = start.profit();
    open_set.insert(start, start_profit);
    open_set
}

// /// Determine all possible new tasks for an agent for the given assignment
// fn possible_tasks(
//     agent: &'static str,
//     assignment: &Assignment,
//     config: &'static SolverConfig,
// ) -> Vec<&'static str> {
//     // Calculate remaining budget
//     let budget = calc_agent_budget(agent, assignment, config);
//     if budget == 0.0 {
//         return Vec::new();
//     };

//     config
//         .tasks
//         .iter()
//         // Agent cannot be assigned to the same task twice
//         .filter(|t| !assignment[agent].contains(*t))
//         // Tasks within agent budget
//         .filter(|t| config.agent_cost[&(agent, **t)] <= budget)
//         // Tasks with enough budget for agent
//         .filter(|t| config.agent_cost[&(agent, **t)] <= budget)
//         .copied()
//         .collect()
// }
