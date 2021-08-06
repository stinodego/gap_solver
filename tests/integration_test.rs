use gap_solver::{solve, Assignment, GapSpec};

#[test]
fn default_spec() {
    let agents = ['a', 'b'];
    let tasks = ['1', '2'];
    let spec: GapSpec<char, char, u32, f64> = GapSpec::new(agents, tasks);

    let result = solve(&spec);

    let assigned1 = [('a', vec!['1']), ('b', vec!['2'])];
    let truth1 = Assignment::from_assigned(assigned1, &spec);

    let assigned2 = [('a', vec!['2']), ('b', vec!['1'])];
    let truth2 = Assignment::from_assigned(assigned2, &spec);

    assert_eq!(result.len(), 2);
    assert!(result.contains(&truth1));
    assert!(result.contains(&truth2));
}

#[test]
fn default_spec_more_agents_than_tasks() {
    let agents = ['a', 'b'];
    let tasks = ['1'];
    let spec: GapSpec<char, char, u32, f64> = GapSpec::new(agents, tasks);

    let result = solve(&spec);

    let assigned1 = [('a', vec!['1']), ('b', vec![])];
    let truth1 = Assignment::from_assigned(assigned1, &spec);

    let assigned2 = [('a', vec![]), ('b', vec!['1'])];
    let truth2 = Assignment::from_assigned(assigned2, &spec);

    assert_eq!(result.len(), 2);
    assert!(result.contains(&truth1));
    assert!(result.contains(&truth2));
}

#[test]
fn default_spec_more_tasks_than_agents() {
    let agents = ['a'];
    let tasks = ['1', '2'];
    let spec: GapSpec<char, char, u32, f64> = GapSpec::new(agents, tasks);

    let result = solve(&spec);

    let assigned1 = [('a', vec!['1'])];
    let truth1 = Assignment::from_assigned(assigned1, &spec);

    let assigned2 = [('a', vec!['2'])];
    let truth2 = Assignment::from_assigned(assigned2, &spec);

    assert_eq!(result.len(), 2);
    assert!(result.contains(&truth1));
    assert!(result.contains(&truth2));
}

#[test]
fn small_problem_spec() {
    // Setup
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
    spec.set_profits(profits);

    let assigned = [('a', vec!['1'])];
    spec.set_assigned(assigned);

    // Run
    let result = solve(&spec);

    // Assert
    let assigned1 = [('a', vec!['1']), ('b', vec!['1', '2']), ('c', vec!['2'])];
    let truth1 = Assignment::from_assigned(assigned1, &spec);

    let assigned2 = [('a', vec!['1']), ('b', vec!['2']), ('c', vec!['1'])];
    let truth2 = Assignment::from_assigned(assigned2, &spec);

    assert_eq!(result, vec![truth1, truth2]);
}
