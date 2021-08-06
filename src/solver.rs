use crate::assignment::Assignment;
use crate::spec::GapSpec;
use log::{debug, info, trace};
use num::Num;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{AddAssign, SubAssign};

/// Solve the assignment problem specified in the given spec
pub fn solve<A, T, C, P>(spec: &GapSpec<A, T, C, P>) -> HashSet<Assignment<A, T, C, P>>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut open_set: HashSet<Assignment<A, T, C, P>> = init_open_set(spec);
    let mut closed_set: HashSet<Assignment<A, T, C, P>> = HashSet::new();
    let mut finished_set: HashSet<Assignment<A, T, C, P>> = HashSet::new();
    let mut max_profit = open_set.iter().next().unwrap().profit();

    while !open_set.is_empty() {
        // Explore the most promising node
        let current = open_set
            .iter()
            .max_by(|x, y| x.profit().partial_cmp(&y.profit()).unwrap())
            .unwrap()
            .clone();
        trace!(
            "Set sizes -- open: {} - closed: {} - maximum: {}",
            open_set.len(),
            closed_set.len(),
            finished_set.len(),
        );
        trace!("Expanding -- {}", current);

        // Remove current node from open set
        open_set.remove(&current);
        // Determine all possible next assignments
        match expand_node(&current, spec, &mut open_set, &closed_set) {
            Ok(_) => {}
            Err(_) => {
                debug!("Found finished assignment -- {}", current);
                handle_finished_assignment(&current, &mut max_profit, &mut finished_set)
            }
        }
        // Move node to the closed set
        closed_set.insert(current);
    }
    finished_set
}

/// Initialize set of assignments to explore
fn init_open_set<A, T, C, P>(spec: &GapSpec<A, T, C, P>) -> HashSet<Assignment<A, T, C, P>>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut open_set = HashSet::new();
    let start = Assignment::from_spec(spec);
    open_set.insert(start);
    open_set
}

/// Determine all possible new tasks for an agent for the given assignment
fn expand_node<'a, A, T, C, P>(
    assignment: &Assignment<'a, A, T, C, P>,
    spec: &GapSpec<A, T, C, P>,
    open_set: &mut HashSet<Assignment<'a, A, T, C, P>>,
    closed_set: &HashSet<Assignment<A, T, C, P>>,
) -> Result<(), &'a str>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    let mut finished = true;
    for agent in spec.agents() {
        // Determine agent budget
        let agent_budget = assignment.agent_budget(agent);
        if agent_budget == C::zero() {
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

/// Manage set of finished assignments.
fn handle_finished_assignment<'a, A, T, C, P>(
    assignment: &Assignment<'a, A, T, C, P>,
    max_profit: &mut P,
    finished_set: &mut HashSet<Assignment<'a, A, T, C, P>>,
) where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    match assignment.profit().partial_cmp(max_profit) {
        Some(Ordering::Equal) => {
            info!("Found maximum assignment -- {}", assignment);
            finished_set.insert(assignment.clone());
        }
        Some(Ordering::Greater) => {
            info!("Found new maximum assignment -- {}", assignment);
            *max_profit = assignment.profit();
            *finished_set = HashSet::new();
            finished_set.insert(assignment.clone());
        }
        _ => {}
    }
}
