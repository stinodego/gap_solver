use crate::config::SolverConfig;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Assignment<'a> {
    assigned: BTreeMap<&'a str, BTreeSet<&'a str>>,
    agent_budget: BTreeMap<&'a str, i64>,
    task_budget: BTreeMap<&'a str, i64>,
    profit: i64,
    config: &'a SolverConfig<'a>,
}

impl<'a> Assignment<'a> {
    pub fn new(config: &'a SolverConfig) -> Self {
        // Initialize empty assignment
        let mut assignment = Self {
            assigned: BTreeMap::new(),
            agent_budget: config.agent_budget.clone(),
            task_budget: config.task_budget.clone(),
            profit: 0,
            config,
        };
        // Handle agents that are already assigned
        for (agent, tasks) in &config.assigned {
            for task in tasks {
                assignment.assign(agent, task).unwrap();
            }
        }
        assignment
    }

    /// Assign an agent to a task
    pub fn assign(&mut self, agent: &'a str, task: &'a str) -> Result<(), AssignmentError> {
        // Check assigned tasks
        let tasks = self.assigned.entry(agent).or_insert_with(BTreeSet::new);
        if tasks.contains(&task) {
            return Err(AssignmentError::new(
                "Cannot assign agent to the same task twice.",
            ));
        }
        // Check agent budget
        let agent_spent = self.config.agent_cost[&(agent, task)];
        let agent_budget = self.agent_budget.get_mut(&agent).unwrap();
        if agent_spent > *agent_budget {
            return Err(AssignmentError::new(
                "Agent does not have enough budget for task.",
            ));
        }
        // Check task budget
        let task_spent = self.config.task_cost[&(agent, task)];
        let task_budget = self.task_budget.get_mut(&task).unwrap();
        if task_spent > *task_budget {
            return Err(AssignmentError::new(
                "Task does not have enough budget for agent.",
            ));
        }
        // Update assigned and budgets
        tasks.insert(task);
        *agent_budget -= agent_spent;
        *task_budget -= task_spent;

        // Update profit
        let profit = self.config.profit[&(agent, task)];
        self.profit += profit;

        Ok(())
    }

    pub fn agent_tasks(&self, agent: &'a str) -> Option<&BTreeSet<&'a str>> {
        self.assigned.get(agent)
    }
    pub fn agent_budget(&self, agent: &'a str) -> i64 {
        self.agent_budget[agent]
    }
    pub fn task_budget(&self, task: &'a str) -> i64 {
        self.task_budget[task]
    }
    pub fn profit(&self) -> i64 {
        self.profit
    }
}

impl fmt::Debug for Assignment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Assignment")
            .field("assigned", &self.assigned)
            .field("agent_budget", &self.agent_budget)
            .field("task_budget", &self.task_budget)
            .field("profit", &self.profit)
            .finish()
    }
}

#[derive(Debug)]
pub struct AssignmentError {
    details: String,
}

impl AssignmentError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AssignmentError {
    fn description(&self) -> &str {
        &self.details
    }
}
