pub mod spawn_creep;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::state::State;

pub struct JobQueue {
    jobs: VecDeque<Box<dyn Job>>,
    next_tick: VecDeque<Box<dyn Job>>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
            next_tick: VecDeque::new(),
        }
    }

    pub fn take_job(&mut self) -> Option<Box<dyn Job>> {
        self.jobs.pop_front()
    }

    pub fn add_job(&mut self, job: Box<dyn Job>) {
        self.next_tick.push_back(job);
    }

    pub fn new_tick(&mut self) {
        self.jobs = VecDeque::new();
        std::mem::swap(&mut self.jobs, &mut self.next_tick);
    }

    pub fn no_jobs(&self) -> bool {
        self.jobs.is_empty()
    }
}

pub trait Job {
    fn run(&self, state: Rc<RefCell<State>>) -> Option<Box<dyn Job>>;
}
