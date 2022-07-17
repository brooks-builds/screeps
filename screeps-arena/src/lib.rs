use screeps_arena::game;
use wasm_bindgen::prelude::*;

mod ctf;
mod global;
mod logging;
mod tutorials;

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
        tutorials::loop_and_import::run();
    }

    #[cfg(feature = "arena-tutorial-simple_move")]
    {
        tutorials::simple_move::run();
    }

    #[cfg(feature = "arena-tutorial-first_attack")]
    {
        tutorials::first_attack::run();
    }

    #[cfg(feature = "arena-tutorial-creeps_bodies")]
    {
        tutorials::creeps_bodies::run();
    }

    #[cfg(feature = "arena-tutorial-store-and-transfer")]
    {
        tutorials::store_and_transfer::run();
    }

    #[cfg(feature = "arena-tutorial-terrain")]
    {
        tutorials::terrain::run();
    }

    #[cfg(feature = "arena-tutorial-spawn-creeps")]
    {
        tutorials::spawn_creeps::run();
    }

    #[cfg(feature = "arena-tutorial-harvest-energy")]
    {
        tutorials::harvest_energy::run();
    }

    #[cfg(feature = "arena-tutorial-construction")]
    {
        tutorials::construction::run(tick);
    }

    #[cfg(feature = "arena-tutorial-final-test")]
    {
        tutorials::final_test::run(tick);
    }

    #[cfg(feature = "arena-capture-the-flag")]
    {
        ctf::run(tick);
    }
}
