//! A solver for the generalized assignment problem.

mod assignment;
mod config;
mod solver;

pub use crate::config::GapSpec;
pub use crate::solver::solve;
