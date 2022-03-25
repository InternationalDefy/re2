use super::*;

/*
    事件监听 ListenerBox
    实现了一组Trait以实现其比较的能力
    _priority : 低者优先级更高
    _swallow : 事件是否吞没
    _listener_box : 监听系统的Box,
*/
pub struct EventListenerRcRef {
    pub _priority : u16,
    pub _swallow : bool,
    pub _listener_rc_ref : Rc<RefCell<dyn EventListener>>,
}

impl EventListenerRcRef {
    pub fn new(priority : u16, swallow : bool, listener : Rc<RefCell<dyn EventListener>>) -> EventListenerRcRef {
        EventListenerRcRef{_priority : priority, _swallow : swallow, _listener_rc_ref : listener}
    }
}

impl PartialEq for EventListenerRcRef {
    fn eq(&self, other: &Self) -> bool {
        self._priority == other._priority
    }
}
impl Eq for EventListenerRcRef {}
impl PartialOrd for EventListenerRcRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EventListenerRcRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self._priority.cmp(&other._priority)
    }
}

/*
    以下 Struct 是为创建EventListenerBox服务的
    其中 _p : u16 -> 优先级
    _s : bool -> 事件吞没
    _evt : EventType -> 事件类型
*/
pub struct ListenerRcStats {
    pub _p : u16, 
    pub _s : bool,
    pub _evt : EventType,
}
impl ListenerRcStats {
    pub fn new(p : u16, s : bool, evt : EventType) -> ListenerRcStats{
        ListenerRcStats{_p : p, _s : s, _evt : evt}
    }
}
pub trait EventListener {
    fn get_listener_rc_stats(&self) -> Vec<ListenerRcStats>;
    fn send_event(&mut self, ev : Event);
}