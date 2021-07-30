use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct SolverConfig<Agent, Task, Currency> {
    agents: Vec<Agent>,
    tasks: Vec<Task>,
    agent_budget: HashMap<Agent, Currency>,
    agent_cost: HashMap<(Agent, Task), Currency>,
    task_budget: HashMap<Task, Currency>,
    task_cost: HashMap<(Agent, Task), Currency>,
    profit: HashMap<(Agent, Task), Currency>,
    assigned: HashMap<Agent, Task>,
    complete: bool,
    fair: bool,
}

impl<Agent, Task, Currency> SolverConfig<Agent, Task, Currency> {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            tasks: Vec::new(),
            agent_budget: HashMap::new(),
            agent_cost: HashMap::new(),
            task_budget: HashMap::new(),
            task_cost: HashMap::new(),
            profit: HashMap::new(),
            assigned: HashMap::new(),
            complete: false,
            fair: false,
        }
    }

    pub fn set_agents(&mut self, agents: Vec<Agent>) {
        self.agents = agents;
    }
    pub fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }
    pub fn set_agent_budget(&mut self, budget: HashMap<Agent, Currency>) {
        self.agent_budget = budget;
    }
    pub fn set_agent_cost(&mut self, cost: HashMap<(Agent, Task), Currency>) {
        self.agent_cost = cost;
    }
    pub fn set_task_budget(&mut self, budget: HashMap<Task, Currency>) {
        self.task_budget = budget;
    }
    pub fn set_task_cost(&mut self, cost: HashMap<(Agent, Task), Currency>) {
        self.task_cost = cost;
    }
    pub fn set_profit(&mut self, profit: HashMap<(Agent, Task), Currency>) {
        self.task_cost = profit;
    }
    pub fn set_assigned(&mut self, assigned: HashMap<Agent, Task>) {
        self.assigned = assigned;
    }
    pub fn set_complete(&mut self, complete: bool) {
        self.complete = complete;
    }
    pub fn set_fair(&mut self, fair: bool) {
        self.fair = fair;
    }
}
