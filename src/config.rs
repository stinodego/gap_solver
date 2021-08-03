use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

/// Define the assignment problem configuration
#[derive(Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SolverConfig<A, T>
where
    A: Ord + Copy,
    T: Ord + Copy,
{
    pub agents: BTreeSet<A>,
    pub tasks: BTreeSet<T>,
    pub agent_budget: BTreeMap<A, i64>,
    pub task_budget: BTreeMap<T, i64>,
    pub agent_cost: BTreeMap<(A, T), i64>,
    pub task_cost: BTreeMap<(A, T), i64>,
    pub profit: BTreeMap<(A, T), i64>,
    pub assigned: BTreeMap<A, BTreeSet<T>>,
}

impl<A, T> SolverConfig<A, T>
where
    A: Ord + Copy,
    T: Ord + Copy,
{
    pub fn new<C, D>(agents: C, tasks: D) -> Self
    where
        C: IntoIterator<Item = A>,
        D: IntoIterator<Item = T>,
        A: Ord + Copy,
        T: Ord + Copy,
    {
        let agents: BTreeSet<A> = agents.into_iter().collect();
        let tasks: BTreeSet<T> = tasks.into_iter().collect();

        // By default, each agent does one task, each task is done by one agent
        let mut agent_budget = BTreeMap::new();
        for a in &agents {
            agent_budget.insert(*a, 1);
        }
        let mut task_budget = BTreeMap::new();
        for t in &tasks {
            task_budget.insert(*t, 1);
        }
        let mut agent_cost = BTreeMap::new();
        let mut task_cost = BTreeMap::new();
        for a in &agents {
            for t in &tasks {
                agent_cost.insert((*a, *t), 1);
                task_cost.insert((*a, *t), 1);
            }
        }

        Self {
            agents,
            tasks,
            agent_budget,
            agent_cost,
            task_budget,
            task_cost,
            profit: BTreeMap::new(),
            assigned: BTreeMap::new(),
        }
    }

    pub fn set_agent_budget<C>(&mut self, budget: C)
    where
        C: IntoIterator<Item = (A, i64)>,
    {
        self.agent_budget = budget.into_iter().collect();
    }
    pub fn set_task_budget<C>(&mut self, budget: C)
    where
        C: IntoIterator<Item = (T, i64)>,
    {
        self.task_budget = budget.into_iter().collect();
    }
    pub fn set_agent_cost(&mut self, cost: BTreeMap<(A, T), i64>) {
        self.agent_cost = cost;
    }
    pub fn set_task_cost(&mut self, cost: BTreeMap<(A, T), i64>) {
        self.task_cost = cost;
    }
    pub fn set_profit<C>(&mut self, profit: C)
    where
        C: IntoIterator<Item = ((A, T), i64)>,
    {
        self.profit = profit.into_iter().collect();
    }
    pub fn set_assigned<C, D>(&mut self, assigned: C)
    where
        C: IntoIterator<Item = (A, D)>,
        D: IntoIterator<Item = T>,
    {
        self.assigned = assigned
            .into_iter()
            .map(|(agent, tasks)| (agent, tasks.into_iter().collect()))
            .collect();
    }
}
