#[macro_use()]
extern crate glium;
extern crate image;

use re2_core::entity_component::Entity;
use re2_metadata::*;
use re2_scene::*;

use std::fs;
use glium::*;
use glam::*;

pub use test_systems::*;
pub use initialization_helpers::*;
pub use render_system::*;
pub use event_system::*;

pub mod test_systems;
pub mod initialization_helpers;
pub mod render_system;
pub mod event_system;
