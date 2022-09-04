mod jobs;
mod state;

use crate::logger::Logger;
use screeps_arena::game::utils::get_ticks;
use state::State;
use std::{cell::RefCell, rc::Rc};

use self::jobs::spawn_creep::SpawnCreepJob;

thread_local! {
    pub static STATE: Rc<RefCell<State>> = Rc::new(RefCell::new(State::new()));
}

pub fn run() {
    let tick = get_ticks();

    STATE.with(|state| {
        if tick == 1 {
            Logger::new().message("initializing bot").log();
            state
                .borrow_mut()
                .job_queue
                .add_job(Box::new(SpawnCreepJob::new(
                    jobs::spawn_creep::CreepType::InitialCollector,
                )));
        }
        state.borrow_mut().job_queue.new_tick();
        loop {
            if let Some(job) = state.borrow_mut().job_queue.take_job() {
                if let Some(job) = job.run(state.clone()) {
                    state.borrow_mut().job_queue.add_job(job);
                }
            } else {
                break;
            }
        }
    })
}
