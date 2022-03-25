use crate::*;

pub struct Game<'a> {
    _updaters : Vec<Rc<RefCell<dyn Updater + 'static>>>,
    _initializers : Vec<Rc<RefCell<dyn Initializer + 'static>>>,
    _events_listeners : Vec<Rc<RefCell<dyn EventListener + 'static>>>,
    pub _world : World,
    pub _display : &'a glium::Display,
    pub _data_pool : DataPool,
}

impl Game<'_> {
    pub fn run_with_world(&mut self, world : World) {
        self._world = world
    }
    pub fn new(display : &glium::Display) -> Game {
        Game{ _updaters : vec!(), _initializers : vec!(), _events_listeners : Vec::new(),
                _world : World::new(), _display : display,
                _data_pool : DataPool::new()}
    }
    pub fn add_initializer(&mut self, init : Rc<RefCell<dyn Initializer + 'static>>) {
        self._initializers.push(init);
    }
    pub fn add_updater(&mut self, update : Rc<RefCell<dyn Updater + 'static>>) {
        self._updaters.push(update);
    }
    pub fn add_event_listener(&mut self, listener : Rc<RefCell<dyn EventListener + 'static>>) {
        self._events_listeners.push(listener);
    }
    pub fn initialize(&mut self) {
        for listener in &self._events_listeners {
            let elrcs_vec = listener.borrow().get_listener_rc_stats();
            for elrcs in elrcs_vec {
                self._data_pool.add_listener_rcref(
                    elrcs._evt, elrcs._p, elrcs._s, listener.clone());
            }
        }
        for init in &self._initializers {
            init.borrow().init(&mut self._data_pool, self._world.get_entitys(), &self._display);
        }
    }
    pub fn update(&mut self, dt : f32) {
        for update in &mut self._updaters {
            update.borrow_mut().update(&mut self._data_pool, self._world.get_entitys(),
                        &self._display, dt);
        }
    }    // TODO -> Display, PollEvents
}