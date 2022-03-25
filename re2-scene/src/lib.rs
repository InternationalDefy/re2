extern crate glium;
#[macro_use()]
extern crate lazy_static;

use std::rc::Rc;
use std::cell::RefCell;

use glium::glutin;

use re2_core::*;
use re2_metadata::{EventType, Event, Vertex};

pub use system_traits::*;
pub use data_pool::*;
pub use world::*;
pub use game::*;

pub mod system_traits;
pub mod data_pool;
pub mod world;
pub mod game;
