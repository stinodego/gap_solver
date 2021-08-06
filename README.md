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

The implementation is a simple depth-first search algorithm. Therefore, it does not work well for very large problems.

The depth-first search expands most promising nodes first. True maximum assignment is guaranteed when algorithm is allowed to complete. Otherwise, the assignment printed last may be used as a best guess.

## Running the code

Solving your assignment problem is easy. Just specify your assignment problem (refer to `main.rs` for an example), then run it. An example problem specification is given, to make clear what syntax is expected.
