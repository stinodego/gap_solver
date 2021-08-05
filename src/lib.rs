//! A solver for the generalized assignment problem.

mod assignment;
mod solver;
mod spec;

pub use crate::assignment::Assignment;
pub use crate::solver::solve;
pub use crate::spec::GapSpec;
