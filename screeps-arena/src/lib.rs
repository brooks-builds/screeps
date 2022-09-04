#![allow(dead_code, unused_imports)]
use screeps_arena::game;
use spawn_and_swamp::run;
use wasm_bindgen::prelude::*;

mod logger;
mod spawn_and_swamp;

#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    #[cfg(feature = "arena-spawn-and-swamp")]
    {
        run();
    }
}
