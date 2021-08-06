use crate::spec::GapSpec;
use num::Num;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, SubAssign};

/// An assignment of agents to tasks.
/// Tracks agent and task budgets and the total profit.
#[derive(Clone)]
pub struct Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    assigned: BTreeMap<A, BTreeSet<T>>,
    agent_budgets: HashMap<A, C>,
    task_budgets: HashMap<T, C>,
    profit: P,
    spec: &'a GapSpec<A, T, C, P>,
}

impl<'a, A, T, C, P> Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    /// Create an Assignment from the given problem specification.
    pub fn from_spec(spec: &'a GapSpec<A, T, C, P>) -> Self {
        // Initialize empty assignment
        let mut assignment = Self {
            assigned: BTreeMap::new(),
            agent_budgets: spec.agent_budgets().clone(),
            task_budgets: spec.task_budgets().clone(),
            profit: P::zero(),
            spec,
        };
        // Handle agents that are already assigned
        for (agent, tasks) in spec.assigned() {
            for task in tasks {
                assignment.assign(agent, task).unwrap();
            }
        }
        assignment
    }

    /// Create an Assignment from a list of assigned agents.
    pub fn from_assigned<M, N>(assigned: M, spec: &'a GapSpec<A, T, C, P>) -> Self
    where
        M: IntoIterator<Item = (A, N)>,
        N: IntoIterator<Item = T>,
    {
        // Initialize empty assignment
        let mut assignment = Self {
            assigned: BTreeMap::new(),
            agent_budgets: spec.agent_budgets().clone(),
            task_budgets: spec.task_budgets().clone(),
            profit: P::zero(),
            spec,
        };
        // Handle agents that are already assigned
        for (agent, tasks) in assigned {
            for task in tasks {
                assignment.assign(&agent, &task).unwrap();
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
        let agent_spent = self.spec.agent_cost(agent, task);
        let agent_budget = self.agent_budgets.get_mut(agent).unwrap();
        if agent_spent > *agent_budget {
            return Err("Agent does not have enough budget for task.");
        }
        // Check task budget
        let task_spent = self.spec.task_cost(agent, task);
        let task_budget = self.task_budgets.get_mut(task).unwrap();
        if task_spent > *task_budget {
            return Err("Task does not have enough budget for agent.");
        }
        // Update assigned and budgets
        tasks.insert(*task);
        *agent_budget -= agent_spent;
        *task_budget -= task_spent;

        // Update profit
        let profit = self.spec.profit(agent, task);
        self.profit += profit;

        Ok(())
    }

    pub fn assigned(&self) -> &BTreeMap<A, BTreeSet<T>> {
        &self.assigned
    }
    pub fn agent_tasks(&self, agent: &A) -> Option<&BTreeSet<T>> {
        self.assigned.get(agent)
    }
    pub fn agent_budget(&self, agent: &A) -> C {
        self.agent_budgets[agent]
    }
    pub fn task_budget(&self, task: &T) -> C {
        self.task_budgets[task]
    }
    pub fn profit(&self) -> P {
        self.profit
    }
}

/// Only the assignment of agents to tasks matters here;
/// the rest can be derived from the problem specification
impl<'a, A, T, C, P> Hash for Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.assigned.hash(state);
    }
}

/// Only the assignment of agents to tasks matters here;
/// the rest can be derived from the problem specification
impl<'a, A, T, C, P> PartialEq for Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.assigned == other.assigned
    }
}
impl<'a, A, T, C, P> Eq for Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
}

impl<'a, A, T, C, P> Display for Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -- {}", self.assigned, self.profit)
    }
}

impl<'a, A, T, C, P> Debug for Assignment<'a, A, T, C, P>
where
    A: Hash + Ord + Copy + Debug,
    T: Hash + Ord + Copy + Debug,
    C: Num + SubAssign + PartialOrd + Copy + Debug,
    P: Num + AddAssign + PartialOrd + Copy + Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Assignment")
            .field("assigned", &self.assigned)
            .field("agent_budgets", &self.agent_budgets)
            .field("task_budgets", &self.task_budgets)
            .field("profit", &self.profit)
            .finish()
    }
}
