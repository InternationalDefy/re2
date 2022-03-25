use super::*;

#[cfg(test)]
pub mod test_world {
    use crate::worlds::*;
    use glium::{glutin, Surface};
    use glutin::EventsLoop;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_world_show_events() {    
        let mut event_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
        let mut game = test_world::get_game(&display);
        let mut frame_time = std::time::Instant::now();
        
        let mut closed = false;
    
        while !closed {
            event_loop.poll_events(|ev| {
                match &ev {                
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        glutin::WindowEvent::CursorMoved{..} => {},
                        _ => {
                            println!("{:?}", ev);
                        },
                    },
                    _ => {
                        println!("{:?}", ev);
                    },
                }            
            });
            // println!("Update!");
            let now = std::time::Instant::now();
            let delta = now - frame_time;
            frame_time = now;
            let delta = delta.as_secs_f32();
            game.update(delta);        
            thread::sleep(Duration::from_millis(4000));
        }
    }
}
