use crate::assignment::Assignment;
use crate::spec::GapSpec;
use num::Num;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::AddAssign;

/// Solve the assignment problem specified in the given spec
pub fn solve<A, T, P>(spec: &GapSpec<A, T, P>) -> Vec<Assignment<A, T, P>>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut open_set: HashSet<Assignment<A, T, P>> = init_open_set(spec);
    let mut closed_set: HashSet<Assignment<A, T, P>> = HashSet::new();
    let mut finished_set: HashSet<Assignment<A, T, P>> = HashSet::new();

    while !open_set.is_empty() {
        // Explore the most promising node
        let current = open_set
            .iter()
            .max_by(|x, y| x.profit().partial_cmp(&y.profit()).unwrap())
            .unwrap()
            .clone();
        // Remove current node from open set
        open_set.remove(&current);
        // Determine all possible next assignments
        match expand_node(&current, spec, &mut open_set, &closed_set) {
            Ok(_) => {}
            Err(_) => {
                finished_set.insert(current.clone());
            }
        }
        // Move node to the closed set
        closed_set.insert(current);
    }
    // Sort resulting assignments by profit (descending)
    format_result(finished_set)
}

/// Initialize set of assignments to explore
fn init_open_set<A, T, P>(spec: &GapSpec<A, T, P>) -> HashSet<Assignment<A, T, P>>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut open_set = HashSet::new();
    let start = Assignment::from_spec(spec);
    open_set.insert(start);
    open_set
}

/// Determine all possible new tasks for an agent for the given assignment
fn expand_node<'a, A, T, P>(
    assignment: &Assignment<'a, A, T, P>,
    spec: &GapSpec<A, T, P>,
    open_set: &mut HashSet<Assignment<'a, A, T, P>>,
    closed_set: &HashSet<Assignment<A, T, P>>,
) -> Result<(), &'a str>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut finished = true;
    for agent in spec.agents() {
        // Determine agent budget
        let agent_budget = assignment.agent_budget(agent);
        if agent_budget == 0 {
            continue;
        };
        // Determine all possible tasks for the agent
        let assigned = assignment.agent_tasks(agent);
        let possible_tasks = spec
            .tasks()
            .iter()
            .copied()
            // Agent cannot be assigned to the same task twice
            .filter(|t| match assigned {
                None => true,
                Some(tasks) => !tasks.contains(t),
            })
            // Tasks within agent budget
            .filter(|t| spec.agent_cost(agent, t) <= agent_budget)
            // Tasks with enough budget for agent
            .filter(|t| spec.task_cost(agent, t) <= assignment.task_budget(t));

        // Create assignments for each task
        for t in possible_tasks {
            finished = false;
            let mut next = assignment.clone();
            next.assign(agent, &t).unwrap();
            if !closed_set.contains(&next) {
                open_set.insert(next);
            }
        }
    }
    if finished {
        Err("Assignment is finished and cannot be expanded.")
    } else {
        Ok(())
    }
}

/// Format the set of finished assignments.
fn format_result<A, T, P>(assignments: HashSet<Assignment<A, T, P>>) -> Vec<Assignment<A, T, P>>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut result: Vec<Assignment<A, T, P>> = assignments.into_iter().collect();
    result.sort_by(|y, x| x.profit().partial_cmp(&y.profit()).unwrap());
    result
}
