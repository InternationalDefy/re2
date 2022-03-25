use super::*;

// TODO Updater理论上不应具有mut 属性 将system自存数据尽量地分配在 Game中
// 原因 &mut self 的 mutability 一致性
pub trait Updater {
    fn update(&mut self,
            data_pool : &mut DataPool,
            entitys : &Vec<Entity>,
            display : &glium::Display,
            dt : f32);
}
