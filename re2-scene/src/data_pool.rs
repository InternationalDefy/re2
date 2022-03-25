use super::*;

pub struct DataPool {
    pub _texture_pool : Pool<String, glium::texture::Texture2d>,
    pub _program_pool : Pool<String, glium::Program>,
    pub _vertex_buffer_pool : Pool<String, glium::VertexBuffer<Vertex>>,
    /* 
        以下数据供事件机制使用
        os_events 仅包含 glutin 系统事件
        dispatched_events 为所有的游戏内事件队列(包括os events)
        evennt_listener_pool只实现系统层面 分发事件 而非确切的回调
    */
    pub _os_events : Vec<glutin::Event>,
    pub _dispatched_events : Vec<Event>,
    pub _event_listener_pool : Pool<EventType, DirtyVec<EventListenerRcRef>>,
}

impl DataPool {
    pub fn new() -> DataPool {
        DataPool{
            _texture_pool : Pool::new(), _program_pool : Pool::new(), 
            _vertex_buffer_pool : Pool::new(), 
            _event_listener_pool : Pool::new(),
            _os_events : Vec::new(),
            _dispatched_events : Vec::new(),
        }
    }
    pub fn dispatch_evnet(&mut self, event : Event) {
        self._dispatched_events.push(event);
    }
    pub fn add_listener_rcref(&mut self, t : EventType, p : u16, s : bool, el_rc : Rc<RefCell<dyn EventListener>>) {        
        let listener_rcref = EventListenerRcRef::new(p, s, el_rc);
        match self._event_listener_pool.get_mut_data().get_mut(&t) {
            Some(dirty_vec) => {
                dirty_vec.push(listener_rcref);
            }
            None => {
                let mut d_vec = DirtyVec::new();
                d_vec.push(listener_rcref);
                self._event_listener_pool.insert(t, d_vec);
            }
        }
    }
    // pub fn add_listener_clousure(&mut self, t : EventType, p : u16, func : Rc<dyn Fn(Event)-> bool>) {
    //     let clousure = EventListenerClousure::new(p, func);
    //     match self._event_listener_pool.get_mut_data().get_mut(&t) {
    //         Some(dirty_vec) => {
    //             dirty_vec.push(clousure);
    //         }
    //         None  => {
    //             let mut d_vec = DirtyVec::new();
    //             d_vec.push(clousure);
    //             self._event_listener_pool.insert(t, d_vec);
    //         }
    //     }
    // }
}
