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
    pub fn new() -> Self {
        Self {
            agents: BTreeSet::new(),
            tasks: BTreeSet::new(),
            agent_budget: BTreeMap::new(),
            agent_cost: BTreeMap::new(),
            task_budget: BTreeMap::new(),
            task_cost: BTreeMap::new(),
            profit: BTreeMap::new(),
            assigned: BTreeMap::new(),
        }
    }

    pub fn set_agents(&mut self, agents: BTreeSet<&'a str>) {
        self.agents = agents;
    }
    pub fn set_tasks(&mut self, tasks: BTreeSet<&'a str>) {
        self.tasks = tasks;
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
    pub fn set_profit(&mut self, profit: BTreeMap<(&'a str, &'a str), i64>) {
        self.profit = profit;
    }
    pub fn set_assigned(&mut self, assigned: BTreeMap<&'a str, BTreeSet<&'a str>>) {
        self.assigned = assigned;
    }
}
