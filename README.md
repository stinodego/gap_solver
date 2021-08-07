# gap_solver

[![crates.io](https://img.shields.io/crates/v/gap_solver.svg)](https://crates.io/crates/gap_solver)
[![docs](https://docs.rs/gap_solver/badge.svg)](https://docs.rs/gap_solver)
[![build](https://img.shields.io/github/workflow/status/stinodego/gap_solver/build)](https://github.com/stinodego/gap_solver/actions/workflows/build.yml)
[![coverage](https://img.shields.io/codecov/c/github/stinodego/gap_solver)](https://codecov.io/gh/stinodego/gap_solver)

This crate provides an interface for specifying a [generalized assignment problem](https://en.wikipedia.org/wiki/Generalized_assignment_problem), and an algorithm for finding the maximum assignment.

This code actually allows for further generalization, allowing multiple agents to perform a single task (regulated by a task budget).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gap_solver = "0.2.0"
```

Now, you can use the `gap_solver` API:
```rust
use gap_solver::{solve, GapSpec};
```

Refer to the [Rust docs](https://docs.rs/gap_solver) for code examples.


## Features

This crate is in active development. Currently, it offers the following features:

* A simple depth-first search algorithm for searching through the space of possible assignments.
* _[In development]_ A `complete` flag for requiring solutions to fully spend agent budgets.
* _[In development]_ A `fair` flag for breaking ties based on the lowest agent profit.
* _[In development]_ Early stopping to deal with large search spaces.

## Things to keep in mind

The generalized assignment problem is a very complex problem. The search space of possible assignments grows exponentially with the total agent and task budget. When dealing with large search spaces, the algorithm will likely be unable to completely search through all possibilities. Options for early stopping are in development.

For now, it is recommended to use a logging utility, and track the progress of the program as it tries to find the maximum assignment. As new maximum assignments are found, these are logged at the `Info` level.
