#![feature(test)]

extern crate test;

use gap_solver::{solve, GapSpec};
use test::Bencher;

#[bench]
fn bench_small(b: &mut Bencher) {
    let spec = setup_small();
    b.iter(|| solve(&spec));
}

#[bench]
fn bench_medium(b: &mut Bencher) {
    let spec = setup_medium();
    b.iter(|| solve(&spec));
}

fn setup_small<'a>() -> GapSpec<&'a str, &'a str, u32, f64> {
    let agents = ["a", "b", "c"];
    let tasks = ["1", "2"];
    let mut spec = GapSpec::new(agents, tasks);

    let agent_budgets = [("a", 1), ("b", 2), ("c", 1)];
    spec.set_agent_budgets(agent_budgets);

    let task_budgets = [("1", 2), ("2", 2)];
    spec.set_task_budgets(task_budgets);

    let profits = [
        (("a", "1"), 3.0),
        (("a", "2"), 1.0),
        (("b", "1"), 1.0),
        (("b", "2"), 3.0),
        (("c", "1"), 2.0),
        (("c", "2"), 2.0),
    ];
    spec.set_profits(profits);

    let assigned = [("a", vec!["1"])];
    spec.set_assigned(assigned);
    spec
}

fn setup_medium<'a>() -> GapSpec<&'a str, &'a str, u32, u32> {
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
    spec
}
