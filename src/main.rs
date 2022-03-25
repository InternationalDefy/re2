#[macro_use]
extern crate glium;
extern crate image;
extern crate glam;
extern crate rand;

use re2_core::entity_component::Component;
// use re2_derive::ComponentMacro;

mod tests;
mod worlds;
fn main() {
    // #[allow(unused_imports)]
    // use glam::{Mat4, Vec3};
    // #[allow(unused_imports)]
    // use glium::{glutin, Surface};
    // let mut event_loop = glutin::event_loop::EventLoop::new();
    // let wb = glutin::window::WindowBuilder::new();
    // let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    // let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    // //TODO 把对Buffter的需求属性整理为一个单独的Component
    // let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    // let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    // let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();
    // //END TODO
    // let _shader = Shader::from_source_name(&display, "test.vs", "test.fs").unwrap();
    // let mut _camera = Camera::new(
    //     Some(glam::Vec3::new(2.0, -1.0, 1.0)),
    //     // None,
    //     None, None, None);
    // _camera.set_front(glam::Vec3::new(-2.0, 1.0, 1.0));
    // let light = [-1.0, 0.4, 0.9f32];
    //     let model = glam::Mat4::from_scale(glam::Vec3::new(0.01, 0.01, 0.01));
    //     let model = model.mul_mat4(&glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 200.0)));
    //     let model = model.to_cols_array_2d();
    //     //END TODO
    //     let perspective = glam::Mat4::perspective_lh(_camera.get_zoom().to_radians(), 1280.0 / 720.0, 0.1, 100.0).to_cols_array_2d();
    //     let view = _camera.get_view_matrix();
    //     println!("camera :\n{:?}", _camera);
    //     println!("model : \n{:?}", model);
    //     println!("view :\n{:?}", view);
    //     println!("projection :\n{:?}", perspective);
    // let params = glium::DrawParameters{
    //     depth: glium::Depth {
    //         test    :   glium::draw_parameters::DepthTest::IfLess,
    //         write   :   true,
    //         .. Default::default()
    //     },
    //     backface_culling : glium::draw_parameters::BackfaceCullingMode::CullClockwise,
    //     .. Default::default()
    // };
    // let mut closed = false;
    // while !closed {
    //     let mut target = display.draw();
    //     target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    //     target.draw((&positions, &normals), &indices, &_shader.program, 
    //                 &uniform!{model : model, u_light : light, view : view, perspective : perspective},
    //                 &params).unwrap();
    //     target.finish().unwrap();
    // }    
    
    // use tests::test_world::get_game;
    use re2_metadata::*;
    use glium::{glutin, Surface};
    use glutin::{EventsLoop, EventsLoopProxy, ControlFlow};
    use glutin::Event;
    use std::time::{Instant, Duration};
    use std::thread;
        
    //let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut event_loop = EventsLoop::new();

    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    //let mut game = test_world::get_game(&display);
    let mut closed = false;
    let mut frame_time = std::time::Instant::now();
    let mut index = 0;

    while !closed {
        index = index + 1;
        let dev = index % 4;
        
        let now = std::time::Instant::now();
        let delta = now - frame_time;
        frame_time = now;
        let delta = delta.as_secs_f32();
        //game.update(delta);
    }
}


