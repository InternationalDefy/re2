#[macro_use]
use crate::*;

#[cfg(test)]
pub mod test_main {
    // use super::*;
    use glium::{glutin, Surface};
    use glutin::EventsLoop;
    use crate::worlds::*;

    #[test]
    fn test_main() {
        run_with_world!(test_world);        
    }
}