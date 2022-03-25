use super::*;

// #[derive(Debug)]
pub struct World {
    _entitys : Vec<Entity>,
}

impl World {
    pub fn get_entitys(&self) -> &Vec<Entity> {
        &self._entitys
    }
    pub fn new() -> World {
        World {_entitys : vec!()}
    }
    pub fn add_entity(&mut self, et : Entity) {
        self._entitys.push(et);
    }
}
