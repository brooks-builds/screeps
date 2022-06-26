use log::warn;
use screeps_arena::game;

#[allow(dead_code)]
pub fn run() {
    let tick = game::utils::get_ticks();
    warn!("current tick: {tick}");
}
