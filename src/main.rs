use gap_solver::solve;
use gap_solver::GapSpec;

fn main() {
    let agents = ['a', 'b', 'c'];
    let tasks = ['1', '2'];
    let mut spec = GapSpec::new(agents, tasks);

    let agent_budgets = [('a', 1), ('b', 2), ('c', 1)];
    spec.set_agent_budgets(agent_budgets);

    let task_budgets = [('1', 2), ('2', 2)];
    spec.set_task_budgets(task_budgets);

    let profits = [
        (('a', '1'), 3.0),
        (('a', '2'), 1.0),
        (('b', '1'), 1.0),
        (('b', '2'), 3.0),
        (('c', '1'), 2.0),
        (('c', '2'), 2.0),
    ];
    spec.set_profit(profits);

    let assigned = [('a', vec!['1'])];
    spec.set_assigned(assigned);

    println!("{:?}", spec);

    let solutions = solve(&spec);
    for assignment in solutions {
        println!("{}", assignment)
    }
}
