//! A solver for the generalized assignment problem.

mod assignment;
mod config;
mod solver;
mod utils;

pub use crate::config::SolverConfig;
pub use crate::solver::solve;
