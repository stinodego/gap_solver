use gap_solver::solve;
use gap_solver::GapSpec;
use std::collections::HashMap;

fn main() {
    let agents = vec!['a', 'b', 'c'];
    let tasks = vec!['1', '2'];
    let mut config = GapSpec::new(agents, tasks);

    let mut agent_budget = HashMap::new();
    agent_budget.insert('a', 1);
    agent_budget.insert('b', 2);
    agent_budget.insert('c', 1);
    config.set_agent_budgets(agent_budget);

    let mut task_budget = HashMap::new();
    task_budget.insert('1', 2);
    task_budget.insert('2', 2);
    config.set_task_budgets(task_budget);

    let mut profit = HashMap::new();
    profit.insert(('a', '1'), 3.0);
    profit.insert(('b', '1'), 1.0);
    profit.insert(('c', '1'), 2.0);
    profit.insert(('a', '2'), 1.0);
    profit.insert(('b', '2'), 3.0);
    profit.insert(('c', '2'), 2.0);
    config.set_profit(profit);

    let mut assigned = HashMap::new();
    assigned.insert('a', vec!['1']);
    config.set_assigned(assigned);

    println!("{:?}", config);

    let solutions = solve(&config);
    for assignment in solutions {
        println!("{}", assignment)
    }
}
