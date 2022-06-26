use screeps_arena::constants::ResourceType;
use screeps_arena::{game::utils::get_objects_by_prototype, prototypes};
use screeps_arena::{Creep, ReturnCode, StructureContainer, StructureTower};

struct Data {
    pub my_creep: Creep,
    pub container: StructureContainer,
    pub tower: StructureTower,
    pub enemy: Creep,
}

impl Data {
    pub fn init() -> Self {
        let mut my_creep = None;
        let mut energy_container = None;
        let mut my_tower = None;
        let mut enemy = None;

        get_objects_by_prototype(prototypes::CREEP)
            .into_iter()
            .for_each(|creep| {
                if creep.my() {
                    my_creep = Some(creep);
                } else {
                    enemy = Some(creep);
                }
            });

        get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER)
            .into_iter()
            .for_each(|container| energy_container = Some(container));

        get_objects_by_prototype(prototypes::STRUCTURE_TOWER)
            .into_iter()
            .for_each(|tower| my_tower = Some(tower));

        Self {
            my_creep: my_creep.expect("Could not find my creep"),
            container: energy_container.expect("Could not find container"),
            tower: my_tower.expect("could not find tower"),
            enemy: enemy.expect("Could not find enemy creep"),
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let data = Data::init();

    let result = data
        .my_creep
        .withdraw(&data.container, ResourceType::Energy, None);

    if result == ReturnCode::NotInRange {
        data.my_creep.move_to(&data.container, None);
    }

    if data
        .my_creep
        .transfer(&data.tower, ResourceType::Energy, None)
        == ReturnCode::NotInRange
    {
        data.my_creep.move_to(&data.tower, None);
    }

    data.tower.attack(&data.enemy);
}
