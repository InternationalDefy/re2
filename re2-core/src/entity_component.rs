//ComponentTrait绝不止id
pub trait Component {
    fn add_to_entity(&mut self, parent_entity : &Entity);
    fn get_parent_id(&self) -> i32;
}

#[derive(Debug)]
pub struct Entity {
    pub id : i32,
}

//TODO -> Entity 的 impl 应该将其实现为不止i32的i32
impl Entity {
    pub fn get_entity(et_identity : i32, entitys : &Vec<Entity>) -> Option<&Entity> {
        for et in entitys {
            if et.id == et_identity {
                return Some(&et);
            }
        }
        None
    }
}