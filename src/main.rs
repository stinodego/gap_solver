use gap_solver::config;
use std::collections::HashMap;

fn main() {
    let mut config = config::SolverConfig::new();
    config.set_agents(vec!['a', 'b', 'c']);
    config.set_tasks(vec![1, 2]);

    let mut task_budget = HashMap::new();
    task_budget.insert(1, 2.0);
    config.set_task_budget(task_budget);

    gap_solver::solve(config);
}
