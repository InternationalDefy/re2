use crate::*;

#[derive(Debug)]
pub struct ShaderHelper {
}

impl ShaderHelper {
    fn build_shader_add_to_pool(
            display : &glium::Display ,
            key : &String, v_name : &String, f_name : &String,
            pool : &mut DataPool) 
    {
        use std::path::PathBuf;
        let vshader = {
            let mut path = PathBuf::from("f:\\");
            path.push("re2");
            path.push("shaders");
            path.push(v_name);
            path.set_extension("vs");
            println!("vertex_shaderpath = {:?}", path);
            let shader_src =  fs::read_to_string(path)
                            .expect("VertexShader NotFound");                            
            shader_src
        };
        let fshader = {
            let mut path = PathBuf::from("f:\\");
            path.push("re2");
            path.push("shaders");
            path.push(f_name);
            path.set_extension("fs");
            println!("fragment_shader path = {:?}", path);
            let shader_src =  fs::read_to_string(path)
                            .expect("Fragement Shader NotFound");                            
            shader_src
        };
        // println!("shader scource! f :{:?}, v :{:?}", fshader, vshader);
        let _program = glium::Program::from_source(display,
            vshader.as_str(), fshader.as_str(), None)
            .unwrap();
        pool._program_pool.insert(key.clone(), _program);
    }
}

impl Initializer for ShaderHelper {
    fn init(&self,
        data_pool : &mut DataPool,
        _entitys : &Vec<Entity>,
        display : &glium::Display)
    {
        // println!("ShaderDataComponents! {:?}", ShaderDataComponent::get_pool_singleton());

        // TODO Shader素材的添加是可以整合的，有想法使用RenderSystem整合，但目前先不完成这件事
        ShaderHelper::build_shader_add_to_pool(display, &String::from("Default"), 
                &String::from("Default"), &String::from("Default"), data_pool
            );
        for shader_data in ShaderDataComponent::get_stack_singleton().get_data() {
            ShaderHelper::build_shader_add_to_pool(display, &shader_data.program_key, 
                &shader_data.vshader_name, &shader_data.fshader_name, data_pool
            );
        }
        println!("Program Pool! {:?}", &data_pool._program_pool.get_data());
    }
}

pub struct VertexBufferHelper {}

impl Initializer for VertexBufferHelper{
    fn init(&self, 
        data_pool : &mut DataPool,
        _entitys : &Vec<Entity>, 
        display : &glium::Display) 
    {        // println!("RenderSystem Updated!");
        for vertex_data in VertexDataComponent::get_stack_singleton().get_data() {
            let vertex_buffer = glium::VertexBuffer::new(display,
                &vertex_data.vertex).unwrap();
            // println!("VertexGeneration!");
            let key_string = vertex_data.vertex_key.clone();
            data_pool._vertex_buffer_pool.insert(key_string, vertex_buffer);
        }
    }
}

#[derive(Debug)]
pub struct SpriteHelper {

}
impl Initializer for SpriteHelper {    
    fn init(&self,
            data_pool : &mut DataPool,
            _entitys : &Vec<Entity>,
            display : &glium::Display)
    {
        use std::path::PathBuf;
        // println!("SpriteDataComponents! {:?}", SpriteDataComponent::get_stack_singleton());
        // let sprite_data_stack = SpriteDataComponent::get_stack_singleton().get_data();
        for sprite_data in SpriteDataComponent::get_stack_singleton().get_data() {
            //TODO 注意还需要分类处理 但目前从简
            //TODO 这里需要处理文件系统的string传入问题
            let mut path = PathBuf::from("f:\\");
            let split : Vec<&str> = sprite_data.image_path.split(".").collect();
            path.push("re2");
            path.push("rescourses");
            path.push(split[0]);
            path.set_extension(split[1]);
            println!("path = {:?}", path);
            let image = image::open(&path).unwrap().into_rgb();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), image_dimensions);
            let texture = glium::texture::Texture2d::new(display, image).unwrap();
            data_pool._texture_pool.insert(sprite_data.texture_key.clone(), texture);
            // wd._texture_pool.get_data().insert(t_str, texture);
        }
    }
}