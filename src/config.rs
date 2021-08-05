use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

/// Define the assignment problem configuration
#[derive(Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SolverConfig<A, T>
where
    A: Ord + Copy + Debug,
    T: Ord + Copy + Debug,
{
    agents: BTreeSet<A>,
    tasks: BTreeSet<T>,
    agent_budgets: BTreeMap<A, u32>,
    task_budgets: BTreeMap<T, u32>,
    agent_cost: BTreeMap<(A, T), u32>,
    task_cost: BTreeMap<(A, T), u32>,
    profit: BTreeMap<(A, T), i64>,
    assigned: BTreeMap<A, BTreeSet<T>>,
}

impl<A, T> SolverConfig<A, T>
where
    A: Ord + Copy + Debug,
    T: Ord + Copy + Debug,
{
    /// Initialize a new assignment problem specification.
    pub fn new<C, D>(agents: C, tasks: D) -> Self
    where
        C: IntoIterator<Item = A>,
        D: IntoIterator<Item = T>,
        A: Ord + Copy + Debug,
        T: Ord + Copy + Debug,
    {
        let agents: BTreeSet<A> = agents.into_iter().collect();
        let tasks: BTreeSet<T> = tasks.into_iter().collect();

        // By default, each agent does one task, each task is done by one agent
        let mut agent_budgets = BTreeMap::new();
        for a in &agents {
            agent_budgets.insert(*a, 1);
        }
        let mut task_budgets = BTreeMap::new();
        for t in &tasks {
            task_budgets.insert(*t, 1);
        }
        let mut agent_cost = BTreeMap::new();
        let mut task_cost = BTreeMap::new();
        let mut profit = BTreeMap::new();
        for a in &agents {
            for t in &tasks {
                agent_cost.insert((*a, *t), 1);
                task_cost.insert((*a, *t), 1);
                profit.insert((*a, *t), 1);
            }
        }

        Self {
            agents,
            tasks,
            agent_budgets,
            agent_cost,
            task_budgets,
            task_cost,
            profit,
            assigned: BTreeMap::new(),
        }
    }

    /// Set the budget for a single agent
    pub fn set_agent_budget(&mut self, agent: A, budget: u32) -> Result<u32, String> {
        if let Entry::Occupied(mut e) = self.agent_budgets.entry(agent) {
            Ok(e.insert(budget))
        } else {
            Err(format!("Agent {:?} not present in configuration.", agent))
        }
    }

    /// Set all agent budgets at once.
    pub fn set_agent_budgets<C>(&mut self, budget: C)
    where
        C: IntoIterator<Item = (A, u32)>,
    {
        self.agent_budgets = budget.into_iter().collect();
    }
    /// Set all task budgets at once.
    pub fn set_task_budgets<C>(&mut self, budget: C)
    where
        C: IntoIterator<Item = (T, u32)>,
    {
        self.task_budgets = budget.into_iter().collect();
    }
    /// Set all agent costs at once.
    pub fn set_agent_cost(&mut self, cost: BTreeMap<(A, T), u32>) {
        self.agent_cost = cost;
    }
    /// Set all task costs at once.
    pub fn set_task_cost(&mut self, cost: BTreeMap<(A, T), u32>) {
        self.task_cost = cost;
    }
    /// Set all profits at once.
    pub fn set_profit<C>(&mut self, profit: C)
    where
        C: IntoIterator<Item = ((A, T), i64)>,
    {
        self.profit = profit.into_iter().collect();
    }
    /// Set all pre-assigned agents at once.
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

    /// Get the list of all agents.
    pub fn agents(&self) -> &BTreeSet<A> {
        &self.agents
    }
    /// Get the list of all tasks.
    pub fn tasks(&self) -> &BTreeSet<T> {
        &self.tasks
    }

    /// Get the agent cost associated with the given agent-task combination.
    /// If the agent is assigned to the task, this cost will be deducted
    /// from its budget.
    pub fn agent_cost(&self, agent: &A, task: &T) -> u32 {
        self.agent_cost[&(*agent, *task)]
    }
    /// Get the task cost associated with the given agent-task combination.
    /// If the agent is assigned to the task, this cost will be deducted
    /// from the task budget.
    pub fn task_cost(&self, agent: &A, task: &T) -> u32 {
        self.task_cost[&(*agent, *task)]
    }
    /// Get the profit associated with the given agent-task combination.
    /// If the agent is assigned to the task, this profit will be added
    /// to the total assignment profit.
    pub fn profit(&self, agent: &A, task: &T) -> i64 {
        self.profit[&(*agent, *task)]
    }

    /// Get the map of agent budgets.
    pub fn agent_budgets(&self) -> &BTreeMap<A, u32> {
        &self.agent_budgets
    }
    /// Get the map of task budgets.
    pub fn task_budgets(&self) -> &BTreeMap<T, u32> {
        &self.task_budgets
    }
    /// Get the map of assigned agent-task combinations.
    pub fn assigned(&self) -> &BTreeMap<A, BTreeSet<T>> {
        &self.assigned
    }
}
