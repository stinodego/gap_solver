# gap_solver

[![crates.io](https://img.shields.io/crates/v/gap_solver.svg)](https://crates.io/crates/gap_solver)
[![docs](https://docs.rs/gap_solver/badge.svg)](https://docs.rs/gap_solver/latest/gap_solver/)
[![build](https://github.com/stinodego/gap_solver/actions/workflows/build.yml/badge.svg)](https://github.com/stinodego/gap_solver/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/stinodego/gap_solver/branch/master/graph/badge.svg)](https://codecov.io/gh/stinodego/gap_solver)

_Generalized Assignment Problem Solver_

This code aims to implement an efficient solver for the generalized assignment problem.

## The generalized assignment problem

The generalized assignment problem is described quite well [on Wikipedia](https://en.wikipedia.org/wiki/Generalized_assignment_problem).

This code actually allows for further generalization, multiple agents to perform a single task (regulated by a task budget).

## The implementation

The implementation is a simple depth-first search algorithm. Therefore, it does not work well for very large problems.

The depth-first search expands most promising nodes first. True maximum assignment is guaranteed when algorithm is allowed to complete. Otherwise, the assignment printed last may be used as a best guess.

## Running the code

Solving your assignment problem is easy. Just specify your assignment problem (refer to `main.rs` for an example), then run it. An example problem specification is given, to make clear what syntax is expected.
