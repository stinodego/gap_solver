use crate::types::Assignment;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

/// Define the assignment problem configuration
#[derive(Debug, Default)]
pub struct SolverConfig {
    pub agents: HashSet<&'static str>,
    pub tasks: HashSet<&'static str>,
    pub agent_budget: BTreeMap<&'static str, f64>,
    pub agent_cost: HashMap<(&'static str, &'static str), f64>,
    pub task_budget: BTreeMap<&'static str, f64>,
    pub task_cost: HashMap<(&'static str, &'static str), f64>,
    pub profit: HashMap<(&'static str, &'static str), f64>,
    pub assigned: Assignment,
    pub complete: bool,
    pub fair: bool,
}

impl SolverConfig {
    pub fn new() -> Self {
        Self {
            agents: HashSet::new(),
            tasks: HashSet::new(),
            agent_budget: BTreeMap::new(),
            agent_cost: HashMap::new(),
            task_budget: BTreeMap::new(),
            task_cost: HashMap::new(),
            profit: HashMap::new(),
            assigned: BTreeMap::new(),
            complete: false,
            fair: false,
        }
    }

    pub fn set_agents(&mut self, agents: HashSet<&'static str>) {
        self.agents = agents;
    }
    pub fn set_tasks(&mut self, tasks: HashSet<&'static str>) {
        self.tasks = tasks;
    }
    pub fn set_agent_budget(&mut self, budget: BTreeMap<&'static str, f64>) {
        self.agent_budget = budget;
    }
    pub fn set_agent_cost(&mut self, cost: HashMap<(&'static str, &'static str), f64>) {
        self.agent_cost = cost;
    }
    pub fn set_task_budget(&mut self, budget: BTreeMap<&'static str, f64>) {
        self.task_budget = budget;
    }
    pub fn set_task_cost(&mut self, cost: HashMap<(&'static str, &'static str), f64>) {
        self.task_cost = cost;
    }
    pub fn set_profit(&mut self, profit: HashMap<(&'static str, &'static str), f64>) {
        self.profit = profit;
    }
    pub fn set_assigned(&mut self, assigned: BTreeMap<&'static str, BTreeSet<&'static str>>) {
        self.assigned = assigned;
    }
    pub fn set_complete(&mut self, complete: bool) {
        self.complete = complete;
    }
    pub fn set_fair(&mut self, fair: bool) {
        self.fair = fair;
    }
}
