use re2_core::*;
use re2_metadata::*;
use re2_system::*;
use re2_scene::*;

use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
use lazy_static::*;

#[cfg(test)]
pub mod render_test_world {
    use super::*;
    pub fn get_game(display : &glium::Display) -> Game {
        let mut game = Game::new(display);
        let shape = vec![
                Vertex { position : [-0.25, -0.5], tex_coords : [0.0, 0.0] },
                Vertex { position : [ 0.25, -0.5], tex_coords : [1.0, 0.0] },
                Vertex { position : [ 0.25, 0.5], tex_coords : [1.0, 1.0] }, 
                Vertex { position : [-0.25, 0.5], tex_coords : [0.0, 1.0] },
        ];

        add_component_to_data_struct!(
            (
                VertexDataComponent, 
                vertex : shape, 
                vertex_key : String::from("ShapeVertex")
            )
            (
                SpriteDataComponent, 
                texture_key : String::from("doge1"),
                image_path : "doge1.jpg"
            )
            (
                SpriteDataComponent,
                texture_key : String::from("doge2"), 
                image_path : "doge2.jpg"
            )
            (
                ShaderDataComponent, 
                program_key : String::from("ColorBlend"),
                vshader_name : String::from("color_blend"), 
                fshader_name : String::from("color_blend")
            )
        );

        let dog_entity = Entity{id : 1};
        add_component_to_entity!(dog_entity, 
            (
                SpriteComponent, texture_key : String::from("doge1")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList, 
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                render_type : None,
                vertex_key : String::from("ShapeVertex")
            )
            (
                TransformComponent, 
                matrix :glam::Mat4::from_translation(glam::Vec3::new(-0.5,0.5,0.0))
            )
        );
        game._world.add_entity(dog_entity);
        let dog_entity_2 = Entity{id : 2};
        add_component_to_entity!(dog_entity_2,
            (
                SpriteComponent, texture_key : String::from("doge2")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList,
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                vertex_key : String::from("ShapeVertex"),
                render_type : None
            )
            (
                TransformComponent, 
                matrix : glam::Mat4::from_translation(glam::Vec3::new(0.5,0.5,0.0))
            )
        );
        game._world.add_entity(dog_entity_2);
        let dog_entity_3 = Entity {id : 3};
        add_component_to_entity!(dog_entity_3,
            (
                SpriteComponent, texture_key : String::from("doge1")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList,
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                vertex_key : String::from("ShapeVertex"),
                render_type : None
            )
            (
                TransformComponent, 
                matrix : glam::Mat4::from_translation(glam::Vec3::new(0.5,-0.5,0.0))
            )
        );
        game._world.add_entity(dog_entity_3);
        let dog_entity_4 = Entity{id : 4};
        add_component_to_entity!(dog_entity_4,
            (
                SpriteComponent, texture_key : String::from("doge2")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList,
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                vertex_key : String::from("ShapeVertex"),
                render_type : Some(RenderType::ColorBlend)
            )
            (
                ColorBlendComponent, 
                blend_color : glam::Vec4::new(1.0, 0.0, 0.0, 1.0)
            )
            (
                TransformComponent, 
                matrix : glam::Mat4::from_translation(glam::Vec3::new(-0.5,-0.5,0.0))
            )
        );
        game._world.add_entity(dog_entity_4);

        let sprite_helper = SpriteHelper{};        
        let shader_helper = ShaderHelper{};
        let render_system = RenderSystem{};
        let vertex_helper = VertexBufferHelper{};
        let event_system = EventSystem{};
        game.add_initializer(Rc::new(RefCell::new(sprite_helper)));
        game.add_initializer(Rc::new(RefCell::new(shader_helper)));
        game.add_initializer(Rc::new(RefCell::new(vertex_helper)));
        game.add_updater(Rc::new(RefCell::new(render_system)));
        game.add_updater(Rc::new(RefCell::new(event_system)));
        game.initialize();
        game
    }
}

#[cfg(test)]
pub mod test_world {
    use super::*;
    pub fn get_game(display : &glium::Display) -> Game {
        let mut game = Game::new(display);
        // TODO 需要实现一个形状系统 -2020.4.20
        let shape = vec![
                Vertex { position : [-0.5, -0.5], tex_coords : [0.0, 0.0] },
                Vertex { position : [ 0.5, -0.5], tex_coords : [1.0, 0.0] },
                Vertex { position : [ 0.5, 0.5], tex_coords : [1.0, 1.0] }, 
                Vertex { position : [-0.5, 0.5], tex_coords : [0.0, 1.0] },
        ];
        // TODO VertexDataComponent的内容实际上是实体无关的, 但考虑到坐标系系统尚未实现，不排除其可能会相关
        add_component_to_data_struct!(
            (
                SpriteDataComponent,
                texture_key : String::from("doge"),
                image_path : "doge.jpg"
            )
            (
                SpriteDataComponent,
                texture_key : String::from("cat"),
                image_path : "cat.jpg"
            )
            (
                VertexDataComponent, 
                vertex : shape, 
                vertex_key : String::from("ShapeVertex")
            )
            (
                ShaderDataComponent, 
                program_key : String::from("ColorBlend"),
                vshader_name : String::from("color_blend"), 
                fshader_name : String::from("color_blend")
            )
        );
        
        let dog_entity = Entity{id : 1};
        add_component_to_entity!(dog_entity, 
            (
                SpriteComponent, texture_key : String::from("doge")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList, 
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                render_type : None,
                vertex_key : String::from("ShapeVertex")
            )
            (
                TransformComponent, matrix :glam::Mat4::from_translation(glam::Vec3::new(-0.6,0.5,0.1))
            )
        );
        game._world.add_entity(dog_entity);

        let dog_entity_2 = Entity{id : 2};
        add_component_to_entity!(dog_entity_2,
            (
                SpriteComponent, texture_key : String::from("doge")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList,
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                vertex_key : String::from("ShapeVertex"),
                render_type : None
            )
            (
                TransformComponent, 
                matrix : glam::Mat4::from_translation(glam::Vec3::new(0.6,0.5,0.1))
            )
        );
        game._world.add_entity(dog_entity_2);

        let cat_entity = Entity{id : 3};
        add_component_to_entity!(cat_entity,
            (
                SpriteComponent, texture_key : String::from("cat")
            )
            (
                RenderComponent, primitive_type : PrimitiveType::TrianglesList,
                indices : Some(vec![0, 1, 2, 0, 2, 3]),
                vertex_key : String::from("ShapeVertex"),
                render_type : Some(RenderType::ColorBlend)
            )
            (
                ColorBlendComponent, 
                blend_color : glam::Vec4::new(0.0, 0.0, 0.0, 1.0)
            )
            (
                TestTransformImpact,
                impact_point : 1.1
            )
            (
                TransformComponent, 
                matrix : glam::Mat4::from_translation(glam::Vec3::new(-0.6, -0.5, 0.1))
                * glam::Mat4::from_scale(glam::Vec3::new(0.8, 0.9, 1.0))
            )
        );
        game._world.add_entity(cat_entity);

        // let doge_1d = 1;
        // game._data_pool._event_listener_pool.insert(String::from("Mouse Scroll Up"), 
        // Box::new(move |event| {
        //     let matrix = TransformComponent::get_pool_singleton().get(&doge_1d).matrix 
        //         * glam::Mat4::from_scale(glam::Vec3::new(1.1, 1.1, 1.1));
        //     TransformComponent::get_pool_singleton().get_mut(&doge_1d).matrix = matrix;
        //     None
        // }));
        // game._data_pool._event_listener_pool.insert(String::from("Mouse Scroll Down"), 
        // Box::new(move |event| {
        //     let matrix = TransformComponent::get_pool_singleton().get(&doge_1d).matrix 
        //         * glam::Mat4::from_scale(glam::Vec3::new(0.9, 0.9, 0.9));
        //     TransformComponent::get_pool_singleton().get_mut(&doge_1d).matrix = matrix;
        //     None
        // }));

        // let cate_id = 3;
        // game._data_pool._event_listener_pool.insert(String::from("Mouse Input"), 
        // Box::new(move |event| {
        //     let mut rng = rand::thread_rng();
        //     ColorBlendComponent::get_pool_singleton().get_mut(&cate_id).blend_color = 
        //         glam::Vec4::new(
        //             rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0),
        //             rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
        //     None
        // }));
        // // TODO 此用法破坏封装，以后会改
        // let doge_id = 2;
        // game._data_pool._event_listener_pool.insert(String::from("Keyboard Input Pressed"), 
        // Box::new(move |event| {
        //     TransformComponent::get_pool_singleton().get_mut(&doge_id).matrix = 
        //         glam::Mat4::from_translation(glam::Vec3::new(0.6,-0.5,0.1));
        //     None
        // }));

        let sprite_helper = SpriteHelper{};        
        let shader_helper = ShaderHelper{};
        let render_system = RenderSystem{};
        let vertex_helper = VertexBufferHelper{};
        let event_system = EventSystem{};
        let transform_listener = re2_system::event_test::TestTransformSystem::new();
        let rcref = Rc::new(RefCell::new(transform_listener));
        game.add_initializer(Rc::new(RefCell::new(sprite_helper)));
        game.add_initializer(Rc::new(RefCell::new(shader_helper)));
        game.add_initializer(Rc::new(RefCell::new(vertex_helper)));
        game.add_updater(Rc::new(RefCell::new(render_system)));
        game.add_updater(Rc::new(RefCell::new(event_system)));
        game.add_updater(rcref.clone());
        game.add_event_listener(rcref.clone());
        game.initialize();
        game
    }
}