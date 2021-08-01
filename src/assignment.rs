use crate::config::SolverConfig;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Assignment {
    assigned: BTreeMap<&'static str, BTreeSet<&'static str>>,
    agent_budget: BTreeMap<&'static str, f64>,
    task_budget: BTreeMap<&'static str, f64>,
    profit: f64,
    config: &'static SolverConfig,
}

impl Assignment {
    pub fn new(config: &'static SolverConfig) -> Self {
        // Initialize empty assignment
        let assignment = Self {
            assigned: BTreeMap::new(),
            agent_budget: config.agent_budget.clone(),
            task_budget: config.task_budget.clone(),
            profit: 0.0,
            config,
        };
        // Handle agents that are already assigned
        for (agent, tasks) in config.assigned {
            for task in tasks {
                assignment.assign(agent, task);
            }
        }
        assignment
    }

    /// Assign an agent to a task
    pub fn assign(
        &mut self,
        agent: &'static str,
        task: &'static str,
    ) -> Result<(), AssignmentError> {
        // Check assigned tasks
        let tasks = self.assigned.entry(agent).or_insert_with(BTreeSet::new);
        if tasks.contains(task) {
            return Err(AssignmentError::new(
                "Cannot assign agent to the same task twice.",
            ));
        }
        // Check agent budget
        let agent_spent = self.config.agent_cost[&(agent, task)];
        let agent_budget = self.agent_budget.get_mut(agent).unwrap();
        if agent_spent > *agent_budget {
            return Err(AssignmentError::new(
                "Agent does not have enough budget for task.",
            ));
        }
        // Check task budget
        let task_spent = self.config.task_cost[&(agent, task)];
        let task_budget = self.task_budget.get_mut(task).unwrap();
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
