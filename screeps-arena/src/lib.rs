use log::warn;
use screeps_arena::game;
use wasm_bindgen::prelude::*;

mod logging;

fn setup() {
    logging::setup_logging(logging::Info);
}

#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup()
    }

    #[cfg(feature = "arena-tutorial-loop-and-import")]
    {
        let log = format!("current tick: {tick}");
        warn!("{log}");
    }
}
