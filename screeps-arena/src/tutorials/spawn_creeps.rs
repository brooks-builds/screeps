use js_sys::Reflect;
use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, Creep, Flag, Part, StructureSpawn,
};
use wasm_bindgen::JsValue;

struct Data {
    pub spawn: StructureSpawn,
    pub flags: Vec<Flag>,
    pub creeps: Vec<Creep>,
}

impl Data {
    pub fn init() -> Self {
        let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
        let flags = get_objects_by_prototype(prototypes::FLAG);
        let creeps = get_objects_by_prototype(prototypes::CREEP);

        Self {
            spawn: spawns[0].clone(),
            flags,
            creeps,
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let data = Data::init();
    let screeps_count = data.creeps.len();

    if screeps_count < 2 {
        if let Ok(creep) = data.spawn.spawn_creep(&[Part::Move]) {
            let target_key = JsValue::from_str("target");
            let flag = &data.flags[screeps_count];
            Reflect::set(&creep, &target_key, flag).expect("Error setting target on creep");
        }
    }

    data.creeps.iter().for_each(|creep| {
        let target_key = JsValue::from_str("target");
        if let Ok(target) = &Reflect::get(creep, &target_key) {
            creep.move_to(target, None);
        }
    })
}
