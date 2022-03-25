use super::*;

#[macro_export]
macro_rules! run_with_world {
    ($world : ident) => {
        let mut event_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let mut game = $world::get_game(&display);
        let mut frame_time = std::time::Instant::now();
        
        let mut closed = false;

        while !closed {
            event_loop.poll_events(|ev| {
                match &ev {                
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => {
                            game._data_pool._os_events.push(ev);
                        },
                    },
                    _ => {
                        game._data_pool._os_events.push(ev);
                    },
                }
            });
            // println!("Update!");
            let now = std::time::Instant::now();
            let delta = now - frame_time;
            frame_time = now;
            let delta = delta.as_secs_f32();
            game.update(delta);        
        }            
    };
}