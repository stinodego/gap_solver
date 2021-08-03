use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

/// Define the assignment problem configuration
#[derive(Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SolverConfig<'a> {
    pub agents: BTreeSet<&'a str>,
    pub tasks: BTreeSet<&'a str>,
    pub agent_budget: BTreeMap<&'a str, i64>,
    pub agent_cost: BTreeMap<(&'a str, &'a str), i64>,
    pub task_budget: BTreeMap<&'a str, i64>,
    pub task_cost: BTreeMap<(&'a str, &'a str), i64>,
    pub profit: BTreeMap<(&'a str, &'a str), i64>,
    pub assigned: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

impl<'a> SolverConfig<'a> {
    pub fn new<T>(agents: T, tasks: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        let agents: BTreeSet<&'a str> = agents.into_iter().collect();
        let tasks: BTreeSet<&'a str> = tasks.into_iter().collect();

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

    pub fn set_agent_budget(&mut self, budget: BTreeMap<&'a str, i64>) {
        self.agent_budget = budget;
    }
    pub fn set_agent_cost(&mut self, cost: BTreeMap<(&'a str, &'a str), i64>) {
        self.agent_cost = cost;
    }
    pub fn set_task_budget(&mut self, budget: BTreeMap<&'a str, i64>) {
        self.task_budget = budget;
    }
    pub fn set_task_cost(&mut self, cost: BTreeMap<(&'a str, &'a str), i64>) {
        self.task_cost = cost;
    }
    pub fn set_profit<T>(&mut self, profit: T)
    where
        T: IntoIterator<Item = ((&'a str, &'a str), i64)>,
    {
        self.profit = profit.into_iter().collect();
    }
    pub fn set_assigned<T, U>(&mut self, assigned: T)
    where
        T: IntoIterator<Item = (&'a str, U)>,
        U: IntoIterator<Item = &'a str>,
    {
        self.assigned = assigned
            .into_iter()
            .map(|(agent, tasks)| (agent, tasks.into_iter().collect()))
            .collect();
    }
}
