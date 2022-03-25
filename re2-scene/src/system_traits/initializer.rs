use super::*;

// #[derive(Debug)]
pub trait Initializer {
    fn init(&self, 
        data_pool : &mut DataPool,
        entitys : &Vec<Entity>, 
        display : &glium::Display);
}
