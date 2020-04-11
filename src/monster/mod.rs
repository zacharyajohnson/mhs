// pub enum MonsterType {

// }

// pub enum MonsterWeakness {

// }

// pub enum MonsterAilment {

// }

//TODO, Are these enums needed?
// Do we really want to constrict them if future games can introduce new types

pub struct Monster {
    name: String,
    type: String,
    weaknesses: Vec<String>,
    ailments: Vec<String>
}