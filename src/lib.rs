//! A solver for the generalized assignment problem.
//!
//! This crate provides an interface for specifying a [generalized assignment
//! problem](https://en.wikipedia.org/wiki/Generalized_assignment_problem),
//! and an algorithm for finding the maximum assignment.
//!
//! This code actually allows for further generalization, allowing multiple
//! agents to perform a single task (regulated by a task budget).
//!
//! # Usage
//!
//! First, specify the problem definition using the `GapSpec` struct.
//! Then, call the `solve` function to produce the set of maximum assignments.
//! Each resulting assignment is represented by an `Assignment` struct.
//!
//! ```
//! use gap_solver::{solve, Assignment, GapSpec};
//!
//! // Define agents and tasks, and initialize the problem specification
//! let agents = ["a", "b", "c"];
//! let tasks = ["1", "2"];
//! let mut spec = GapSpec::new(agents, tasks);
//!
//! // Specify agent and task budgets
//! let agent_budgets = [("a", 1), ("b", 2), ("c", 1)];
//! spec.set_agent_budgets(agent_budgets);
//!
//! let task_budgets = [("1", 2), ("2", 2)];
//! spec.set_task_budgets(task_budgets);
//!
//! // Specify profit associated with agent-task assignments
//! let profits = [
//!     (("a", "1"), 3.0),
//!     (("a", "2"), 1.0),
//!     (("b", "1"), 1.0),
//!     (("b", "2"), 3.0),
//!     (("c", "1"), 2.0),
//!     (("c", "2"), 2.0),
//! ];
//! spec.set_profits(profits);
//!
//! // Pre-assign any agents
//! let assigned = [("a", vec!["1"])];
//! spec.set_assigned(assigned);
//!
//! // Let the solving algorithm work its magic!
//! let assignments = solve(&spec);
//!
//! // This problem specification will result in a single maximum assignment
//! let assigned = [("a", vec!["1"]), ("b", vec!["1", "2"]), ("c", vec!["2"])];
//! assert_eq!(assignments.len(), 1);
//! assert!(assignments.contains(&Assignment::from_assigned(assigned, &spec)));
//! ```

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod assignment;
mod solver;
mod spec;

pub use crate::assignment::Assignment;
pub use crate::solver::solve;
pub use crate::spec::GapSpec;
