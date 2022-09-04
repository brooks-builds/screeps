use modes::spawn_and_swamp::run;
use wasm_bindgen::prelude::wasm_bindgen;

mod js;
mod log;
mod modes;

#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    run();
}
