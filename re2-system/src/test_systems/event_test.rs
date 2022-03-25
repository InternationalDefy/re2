use re2_metadata::{Event, EventType, EventMeta, TestTransformImpact, TransformComponent};
use re2_scene::{EventListener, Updater, DataPool, ListenerRcStats};
use re2_core::entity_component::Entity;

pub struct TestTransformSystem {
    _transform_events : Vec<Event>,
}

impl Updater for TestTransformSystem {
    fn update(&mut self,
        _data_pool : &mut DataPool,
        _entitys : &Vec<Entity>,
        _display : &glium::Display,
        _dt : f32) {
        // TODO IMPL
        for ev in self._transform_events.iter() {
            let pool_singleton = TestTransformImpact::get_pool_singleton();
            for (et, cp) in pool_singleton.get_data() {
                let matrix = TransformComponent::get_pool_singleton().get(&et).unwrap().matrix;
                let scale_point : f32 = match ev._meta {
                    EventMeta::Offset(true) => cp.impact_point,
                    EventMeta::Offset(false) => 1.0 / cp.impact_point,
                    _ => {
                        println!("Wrong Meta!");
                        1.0
                    }
                };
                let scale_matrix = glam::Mat4::from_scale(glam::Vec3::new(scale_point, scale_point, 1.0));
                TransformComponent::get_pool_singleton().get_mut(&et).unwrap().matrix = matrix * scale_matrix;
            }
        }
        self._transform_events.clear();
    }
}

// impl EventListener for TestTransformSystem {
//     fn register_event_listener_box(&self, 
//         data_pool : &mut DataPool) {
//         data_pool.add_listener_box(
//             EventType::KeyBoardDown, 1, true, 
//         );
//     }
//     fn send(&)
// }

impl EventListener for TestTransformSystem {
    fn get_listener_rc_stats(&self) -> Vec<ListenerRcStats> {
        let mut vec_ret = Vec::new();
        vec_ret.push(ListenerRcStats::new(1, true,  EventType::MouseScroll));
        vec_ret
    }
    fn send_event(&mut self, ev : Event){
        self._transform_events.push(ev);
    }
}

impl TestTransformSystem {
    pub fn new() -> TestTransformSystem {
        TestTransformSystem{_transform_events : Vec::new()}
    }
}

pub struct TestColorBlendSystem {
    _color_blend_events : Vec<Event>,
}
impl Updater for TestColorBlendSystem {    
    fn update(&mut self,
        _data_pool : &mut DataPool,
        _entitys : &Vec<Entity>,
        _display : &glium::Display,
        _dt : f32) {
        // TODO IMPL
        println!("impl test ColorBlendSystem update");
    }
}
// impl EventListener for TestColorBlendSystem {
//     fn register_event_listener_clousures(&self, 
//         data_pool : &mut DataPool) {
//         data_pool.add_listener_clousure(
//             EventType::KeyBoardDown, 1, Box::new(move |event| {
//                 // self._color_blend_events.push(event);
//                 false
//             })
//         );
//     }
// }
impl EventListener for TestColorBlendSystem {
    fn get_listener_rc_stats(&self) -> Vec<ListenerRcStats> {
        Vec::new()
    }
    fn send_event(&mut self, ev : Event){
        self._color_blend_events.push(ev);
    }
}