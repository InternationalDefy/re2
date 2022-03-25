use crate::*;

#[derive(Debug)]
pub struct RenderSystem {
}

impl Updater for RenderSystem {
    fn update(&mut self,
        data_pool : &mut DataPool,
        entitys : &Vec<Entity>,
        display : &glium::Display,
        _dt : f32) {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        // println!("RenderSystem Updated!");
        for (render_id, render_data) in RenderComponent::get_pool_singleton().get_data() {
            let entity = Entity::get_entity(*render_id, entitys);
            match entity {
                Some(_) => {
                    match &render_data.render_type {
                        Some(render_type) => {
                            match render_type {
                                RenderType::ColorBlend => {
                                    RenderSystem::color_blend_render_process(display, &mut target, data_pool, render_id, render_data)
                                }
                                _ => RenderSystem::default_render_process(display, &mut target, data_pool, render_id, render_data)
                            }
                            // TODO multiply match
                        },
                        None => RenderSystem::default_render_process(display, &mut target, data_pool, render_id, render_data)
                    };
                },
                None => {}
            };
        }
        target.finish().unwrap();
    }    
}

impl RenderSystem {
    fn color_blend_render_process(display : &glium::Display, target : &mut glium::Frame,
        data_pool : &DataPool, 
        render_id : &i32, render_data : &re2_metadata::RenderComponent) {
        let vertex_buffer = data_pool._vertex_buffer_pool.
            get(&render_data.vertex_key).expect("VertexBuffer Unfound");
        let primitive_type = match &render_data.primitive_type {
            PrimitiveType::LineStrip => glium::index::PrimitiveType::LineStrip,
            PrimitiveType::LinesList => glium::index::PrimitiveType::LinesList,             
            PrimitiveType::TrianglesList => glium::index::PrimitiveType::TrianglesList,
            PrimitiveType::TriangleStrpi => glium::index::PrimitiveType::TriangleStrip,
            PrimitiveType::Points => glium::index::PrimitiveType::Points,
        };
        let key = String::from("ColorBlend");
        let program = data_pool._program_pool.get(&key).expect("Porgram Unfound");
        let texture = data_pool._texture_pool.get(
            &SpriteComponent::get_pool_singleton().get(render_id).expect("Texture Key Unfound").texture_key).expect("Texture Unfound");
        let color_vec : [f32; 4] = ColorBlendComponent::get_pool_singleton().get(render_id).expect("ColorBlendComponent Unfound").blend_color.into();
        let uniforms = uniform!{
            matrix : TransformComponent::get_pool_singleton().get(render_id).expect("Transform Unfound").matrix.to_cols_array_2d(), 
            tex : texture, blend_color : color_vec,
        };
        // println!("Render Here! Program key {:?} Color Vec {:?}", &ShaderComponent::get_pool_singleton().get(render_id).program_key, color_vec);
        match &render_data.indices {
            Some(indices) => {
                let indice_buffer = glium::index::IndexBuffer::new(display, 
                                    primitive_type, indices).unwrap();
                target.draw(vertex_buffer, &indice_buffer, program, &uniforms,
                    &Default::default()).unwrap();
            },
            None => {
                let indice_buffer = glium::index::NoIndices(primitive_type);
                target.draw(vertex_buffer, indice_buffer, program, &uniforms,
                    &Default::default()).unwrap();
            }
        };

    }
    fn default_render_process(display : &glium::Display, target : &mut glium::Frame,
            data_pool : &DataPool, 
            render_id : &i32, render_data : &re2_metadata::RenderComponent) {
        let vertex_buffer = data_pool._vertex_buffer_pool.
            get(&render_data.vertex_key).expect("VertexBuffer Unfound");
        let primitive_type = match &render_data.primitive_type {
            PrimitiveType::LineStrip => glium::index::PrimitiveType::LineStrip,
            PrimitiveType::LinesList => glium::index::PrimitiveType::LinesList,             
            PrimitiveType::TrianglesList => glium::index::PrimitiveType::TrianglesList,
            PrimitiveType::TriangleStrpi => glium::index::PrimitiveType::TriangleStrip,
            PrimitiveType::Points => glium::index::PrimitiveType::Points,
        };
        // let program = data_pool._program_pool.get(&ShaderComponent::get_pool_singleton().get(render_id).program_key);
        let key = String::from("Default");
        let program = data_pool._program_pool.get(&key).expect("Program Unfound");
        let texture = data_pool._texture_pool.get(&SpriteComponent::get_pool_singleton().get(render_id).expect("TextureKey Unfound").texture_key).expect("Texture Unfound");                    
        let uniforms = uniform!{
            matrix : TransformComponent::get_pool_singleton().get(render_id).expect("Matrix Unfound").matrix.to_cols_array_2d(), tex : texture,
        };
        // TODO indices 分配
        // let indice_buffer = match &render_data.indice {
        //     Some(indice) => glium::IndexBuffer::new(display, primitive_type, &indice).unwrap(),
        //     None => glium::index::NoIndices(primitive_type),
        // };
        match &render_data.indices {
            Some(indices) => {
                let indice_buffer = glium::index::IndexBuffer::new(display, 
                                    primitive_type, indices).unwrap();
                target.draw(vertex_buffer, &indice_buffer, program, &uniforms,
                    &Default::default()).unwrap();
            },
            None => {
                let indice_buffer = glium::index::NoIndices(primitive_type);
                target.draw(vertex_buffer, indice_buffer, program, &uniforms,
                    &Default::default()).unwrap();
            }
        };
    }
}
