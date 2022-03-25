use super::*;

#[cfg(test)]
pub mod test_render {
    use crate::worlds::*;
    use glium::{glutin, Surface};
    use glutin::EventsLoop;
    
    #[test]
    fn test_render_world_image() {        
        run_with_world!(render_test_world);
    }    
}