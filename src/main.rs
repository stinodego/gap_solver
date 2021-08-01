use gap_solver::config;
use gap_solver::solver;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut config = config::SolverConfig::new();

    let mut agents = HashSet::new();
    agents.insert("a");
    agents.insert("b");
    agents.insert("c");
    config.set_agents(agents);

    let mut tasks = HashSet::new();
    tasks.insert("t1");
    tasks.insert("t2");
    config.set_tasks(tasks);

    let mut agent_budget = HashMap::new();
    agent_budget.insert("a", 1.0);
    agent_budget.insert("b", 2.0);
    agent_budget.insert("c", 1.0);
    config.set_agent_budget(agent_budget);

    let mut task_budget = HashMap::new();
    task_budget.insert("t1", 2.0);
    task_budget.insert("t2", 2.0);
    config.set_task_budget(task_budget);

    let mut agent_cost = HashMap::new();
    let mut task_cost = HashMap::new();
    for a in &config.agents {
        for t in &config.tasks {
            agent_cost.insert((*a, *t), 1.0);
            task_cost.insert((*a, *t), 1.0);
        }
    }
    config.set_agent_cost(agent_cost);
    config.set_task_cost(task_cost);

    let mut profit = HashMap::new();
    profit.insert(("a", "t1"), 3.0);
    profit.insert(("b", "t1"), 1.0);
    profit.insert(("c", "t1"), 2.0);
    profit.insert(("a", "t2"), 1.0);
    profit.insert(("b", "t2"), 3.0);
    profit.insert(("c", "t2"), 2.0);
    config.set_profit(profit);

    // let mut assigned = BTreeMap::new();
    // let mut a_tasks = BTreeSet::new();
    // a_tasks.insert("t1");
    // assigned.insert("a", a_tasks);
    // config.set_assigned(assigned);

    println!("{:?}", config);

    solver::solve(config);
}
