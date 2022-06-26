use js_sys::Array;
use screeps_arena::{game::utils::get_objects_by_prototype, prototypes, Creep, Flag};

struct Data {
    creeps: Vec<Creep>,
    flags: Vec<Flag>,
}

impl Data {
    pub fn init() -> Self {
        let creeps = get_objects_by_prototype(prototypes::CREEP);
        let flags = get_objects_by_prototype(prototypes::FLAG);

        Self { creeps, flags }
    }

    pub fn flags_to_array(&self) -> Array {
        let array = Array::new();

        self.flags.iter().for_each(|flag| {
            array.push(flag);
        });

        array
    }
}

#[allow(dead_code)]
pub fn run() {
    let data = Data::init();

    data.creeps.iter().for_each(|creep| {
        if let Some(flag) = &creep.find_closest_by_path(&data.flags_to_array(), None) {
            creep.move_to(flag, None);
        }
    })
}
