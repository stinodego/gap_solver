use gap_solver::solve;
use gap_solver::GapSpec;
// use simple_logger::SimpleLogger;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Setup
    let agents = ["a", "b", "c"];
    let tasks = ["1", "2", "3", "4", "5", "6"];
    let mut spec = GapSpec::new(agents, tasks);

    let agent_budgets: Vec<(&str, u32)> = agents.iter().map(|a| (*a, 2)).collect();
    spec.set_agent_budgets(agent_budgets);

    let task_budgets: Vec<(&str, u32)> = tasks.iter().map(|t| (*t, 1)).collect();
    spec.set_task_budgets(task_budgets);

    let profits = [
        (("a", "1"), 1),
        (("a", "2"), 2),
        (("a", "3"), 3),
        (("a", "4"), 4),
        (("a", "5"), 5),
        (("a", "6"), 6),
        (("b", "1"), 1),
        (("b", "2"), 5),
        (("b", "3"), 4),
        (("b", "4"), 6),
        (("b", "5"), 5),
        (("b", "6"), 6),
        (("c", "1"), 1),
        (("c", "2"), 2),
        (("c", "3"), 1),
        (("c", "4"), 1),
        (("c", "5"), 1),
        (("c", "6"), 1),
    ];
    spec.set_profits(profits);

    let solutions = solve(&spec);
    for assignment in solutions {
        println!("{}", assignment)
    }
}
