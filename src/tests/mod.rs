use re2_core::entity_component::*;
use re2_core::datastructs::*;
use re2_core::*;
use re2_derive::*;
use std::sync::{MutexGuard, Mutex};
#[macro_use]
use lazy_static::*;
/*
*测试的组织结构意图在于降低单元测试间的依存性
*虽然在每个包的独立lib中设计单元测试自然在文件依存性上最低，但是简单如core的单元都包含derive，所以作为集成测试的实现，即使不满足最低文件依存性，但能够简化编程过程中对cargo.toml和use的过多修改。
*2020.6.13
*/


#[macro_use]
mod macros;

mod test_core;
mod test_render;
mod test_world;
mod test_event;
mod test_main;
