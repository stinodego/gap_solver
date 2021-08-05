use crate::config::SolverConfig;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
    assigned: BTreeMap<A, BTreeSet<T>>,
    agent_budget: HashMap<A, u32>,
    task_budget: HashMap<T, u32>,
    profit: f64,
    config: &'a SolverConfig<A, T>,
}

impl<'a, A, T> Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
    pub fn new(config: &'a SolverConfig<A, T>) -> Self {
        // Initialize empty assignment
        let mut assignment = Self {
            assigned: BTreeMap::new(),
            agent_budget: config.agent_budgets().clone(),
            task_budget: config.task_budgets().clone(),
            profit: 0.0,
            config,
        };
        // Handle agents that are already assigned
        for (agent, tasks) in config.assigned() {
            for task in tasks {
                assignment.assign(agent, task).unwrap();
            }
        }
        assignment
    }

    /// Assign an agent to a task
    pub fn assign(&mut self, agent: &A, task: &T) -> Result<(), &str> {
        // Check assigned tasks
        let tasks = self.assigned.entry(*agent).or_insert_with(BTreeSet::new);
        if tasks.contains(task) {
            return Err("Cannot assign agent to the same task twice.");
        }
        // Check agent budget
        let agent_spent = self.config.agent_cost(agent, task);
        let agent_budget = self.agent_budget.get_mut(agent).unwrap();
        if agent_spent > *agent_budget {
            return Err("Agent does not have enough budget for task.");
        }
        // Check task budget
        let task_spent = self.config.task_cost(agent, task);
        let task_budget = self.task_budget.get_mut(task).unwrap();
        if task_spent > *task_budget {
            return Err("Task does not have enough budget for agent.");
        }
        // Update assigned and budgets
        tasks.insert(*task);
        *agent_budget -= agent_spent;
        *task_budget -= task_spent;

        // Update profit
        let profit = self.config.profit(agent, task);
        self.profit += profit;

        Ok(())
    }

    pub fn assigned(&self) -> &BTreeMap<A, BTreeSet<T>> {
        &self.assigned
    }
    pub fn agent_tasks(&self, agent: &A) -> Option<&BTreeSet<T>> {
        self.assigned.get(agent)
    }
    pub fn agent_budget(&self, agent: &A) -> u32 {
        self.agent_budget[agent]
    }
    pub fn task_budget(&self, task: &T) -> u32 {
        self.task_budget[task]
    }
    pub fn profit(&self) -> f64 {
        self.profit
    }
}

/// Only the assignment of agents to tasks matters here;
/// the rest can be derived from the problem specification
impl<'a, A, T> Hash for Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.assigned.hash(state);
    }
}

/// Only the assignment of agents to tasks matters here;
/// the rest can be derived from the problem specification
impl<'a, A, T> PartialEq for Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.assigned == other.assigned
    }
}
impl<'a, A, T> Eq for Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
}

impl<'a, A, T> fmt::Display for Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assignment {{ {:?}: {} }}", self.assigned, self.profit)
    }
}

impl<'a, A, T> Debug for Assignment<'a, A, T>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Assignment")
            .field("assigned", &self.assigned)
            .field("agent_budget", &self.agent_budget)
            .field("task_budget", &self.task_budget)
            .field("profit", &self.profit)
            .finish()
    }
}
