use num::Num;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{AddAssign, SubAssign};

/// Define the assignment problem configuration
#[derive(Debug)]
pub struct GapSpec<A, T, C, P> {
    agents: HashSet<A>,
    tasks: HashSet<T>,
    agent_budgets: HashMap<A, C>,
    task_budgets: HashMap<T, C>,
    agent_cost: HashMap<(A, T), C>,
    task_cost: HashMap<(A, T), C>,
    profit: HashMap<(A, T), P>,
    assigned: HashMap<A, HashSet<T>>,
}

impl<A, T, C, P> GapSpec<A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    /// Initialize a new assignment problem specification.
    pub fn new<M, N>(agents: M, tasks: N) -> Self
    where
        M: IntoIterator<Item = A>,
        N: IntoIterator<Item = T>,
    {
        let agents: HashSet<A> = agents.into_iter().collect();
        let tasks: HashSet<T> = tasks.into_iter().collect();

        // By default, each agent does one task, each task is done by one agent
        let mut agent_budgets = HashMap::new();
        for a in &agents {
            agent_budgets.insert(*a, C::one());
        }
        let mut task_budgets = HashMap::new();
        for t in &tasks {
            task_budgets.insert(*t, C::one());
        }
        let mut agent_cost = HashMap::new();
        let mut task_cost = HashMap::new();
        let mut profit = HashMap::new();
        for a in &agents {
            for t in &tasks {
                agent_cost.insert((*a, *t), C::one());
                task_cost.insert((*a, *t), C::one());
                profit.insert((*a, *t), P::one());
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
            assigned: HashMap::new(),
        }
    }

    /// Set the budget for a single agent
    pub fn set_agent_budget(&mut self, agent: A, budget: C) -> Result<C, String> {
        if let Entry::Occupied(mut e) = self.agent_budgets.entry(agent) {
            Ok(e.insert(budget))
        } else {
            Err(format!("Agent {:?} not present in configuration.", agent))
        }
    }

    /// Set all agent budgets at once.
    pub fn set_agent_budgets<M>(&mut self, budgets: M)
    where
        M: IntoIterator<Item = (A, C)>,
    {
        self.agent_budgets = budgets.into_iter().collect();
    }
    /// Set all task budgets at once.
    pub fn set_task_budgets<M>(&mut self, budgets: M)
    where
        M: IntoIterator<Item = (T, C)>,
    {
        self.task_budgets = budgets.into_iter().collect();
    }

    /// Set all agent costs at once.
    pub fn set_agent_cost(&mut self, cost: HashMap<(A, T), C>) {
        self.agent_cost = cost;
    }
    /// Set all task costs at once.
    pub fn set_task_cost(&mut self, cost: HashMap<(A, T), C>) {
        self.task_cost = cost;
    }

    /// Set all profits at once.
    pub fn set_profits<M>(&mut self, profit: M)
    where
        M: IntoIterator<Item = ((A, T), P)>,
    {
        self.profit = profit.into_iter().collect();
    }
    /// Set all pre-assigned agents at once.
    pub fn set_assigned<M, N>(&mut self, assigned: M)
    where
        M: IntoIterator<Item = (A, N)>,
        N: IntoIterator<Item = T>,
    {
        self.assigned = assigned
            .into_iter()
            .map(|(agent, tasks)| (agent, tasks.into_iter().collect()))
            .collect();
    }

    /// Get the list of all agents.
    pub fn agents(&self) -> &HashSet<A> {
        &self.agents
    }
    /// Get the list of all tasks.
    pub fn tasks(&self) -> &HashSet<T> {
        &self.tasks
    }

    /// Get the agent cost associated with the given agent-task combination.
    /// If the agent is assigned to the task, this cost will be deducted
    /// from its budget.
    pub fn agent_cost(&self, agent: &A, task: &T) -> C {
        self.agent_cost[&(*agent, *task)]
    }
    /// Get the task cost associated with the given agent-task combination.
    /// If the agent is assigned to the task, this cost will be deducted
    /// from the task budget.
    pub fn task_cost(&self, agent: &A, task: &T) -> C {
        self.task_cost[&(*agent, *task)]
    }
    /// Get the profit associated with the given agent-task combination.
    /// If the agent is assigned to the task, this profit will be added
    /// to the total assignment profit.
    pub fn profit(&self, agent: &A, task: &T) -> P {
        self.profit[&(*agent, *task)]
    }

    /// Get the map of agent budgets.
    pub fn agent_budgets(&self) -> &HashMap<A, C> {
        &self.agent_budgets
    }
    /// Get the map of task budgets.
    pub fn task_budgets(&self) -> &HashMap<T, C> {
        &self.task_budgets
    }
    /// Get the map of assigned agent-task combinations.
    pub fn assigned(&self) -> &HashMap<A, HashSet<T>> {
        &self.assigned
    }
}
