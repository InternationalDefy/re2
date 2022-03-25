use crate::*;

pub struct EventSystem {
}

impl Updater for EventSystem {
    fn update(&mut self, 
        data_pool : &mut DataPool, _entitys : &Vec<Entity>, 
        _display : &glium::Display, _dt : f32) {
        // use re2_metadata::EventType;
        // 此处两边reverse的设计不优雅
        while !data_pool._os_events.is_empty() {            
            let os_ev : glutin::Event = data_pool._os_events.pop().unwrap();
            let ev = self.transform_os_event(&os_ev);    
            data_pool.dispatch_evnet(ev);
        }
        let mut reverse_event_queue = Vec::new();
        while !data_pool._dispatched_events.is_empty(){
            reverse_event_queue.push(data_pool._dispatched_events.pop().unwrap())
        }
        while !reverse_event_queue.is_empty(){
            let ev = reverse_event_queue.pop().unwrap();
            let d_vec = data_pool._event_listener_pool.get_mut(&ev._type);
            match d_vec {
                Some(d) => {
                    let vec = d.get_data();
                    for listener_rc in vec {
                        listener_rc._listener_rc_ref.borrow_mut().send_event(ev.clone());
                        if listener_rc._swallow {
                            println!("Swallow Event");
                            break;
                        }
                    }
                },
                None => {}
            }
        }
        // 以下用于Debug输出Event
        // if !reverse_event_queue.is_empty() {println!("Reversed");}
        // for (i) in &reverse_event_queue {
        //     if i._type != EventType::UNIMPLEMENTED_TYPE {
        //         print!("{:?}", &i);
        //     }
        // }
    }
}

impl EventSystem {    
    // 把OS Event转换为内建数据结构
    fn transform_os_event(&mut self, event : &glutin::Event) -> Event {
        // use glutin::{WindowEvent, DeviceEvent};
        // use re2_metadata::{Event};
        use re2_metadata::EventType::*;
        use re2_metadata::EventMeta::*;
        // TODO 这一个enum需要完善 也有可能换用其它的enum取代String 作为key
        match event {
            glutin::Event::WindowEvent{window_id : _, event} => {
                match event {
                    glutin::WindowEvent::KeyboardInput{ device_id : _, input } => {
                        match input.state {
                            glutin::ElementState::Pressed => {
                                Event{_type : KeyBoardDown, _meta : UNIMPLEMENTED}
                            },
                            glutin::ElementState::Released => {
                                Event{_type : KeyBoardUp, _meta : UNIMPLEMENTED}
                            }
                        }
                    },
                    glutin::WindowEvent::MouseInput{device_id : _,
                        state : _, button : _, modifiers : _  } => {
                            Event{_type : MouseLeftDown, _meta : UNIMPLEMENTED}
                    },
                    glutin::WindowEvent::MouseWheel{device_id : _, 
                        delta, phase : _, modifiers : _} => {
                            match delta {
                                // glutin::MouseScrollDelta::LineDelta{delta1, delta2} => {
                                //     String::from("Mouse Scroll")
                                // }
                                glutin::MouseScrollDelta::LineDelta(_x, y) => {
                                    // println!("LineScroll, x {} y{}", x, y);
                                    if y > &0.0 {
                                        Event{_type : MouseScroll, _meta : Offset(true)}
                                    }
                                    else {
                                        Event{_type : MouseScroll, _meta : Offset(false)}
                                    }
                                }
                                _ => {
                                    Event{_type : MouseScroll, _meta : UNIMPLEMENTED}
                                }
                            }
                    },
                    _ => {
                        Event{_type : UnimplementedType, _meta : UNIMPLEMENTED}
                    },
                }
            },
            glutin::Event::DeviceEvent{device_id : _, event : _} => {
                Event{_type : UnimplementedType, _meta : UNIMPLEMENTED}
            },
            _ => {
                Event{_type : UnimplementedType, _meta : UNIMPLEMENTED}
            },
        }
    }
}