/*
    事件类 Event
    1.注意此类没有实现 Trait Component
    2.Event由两个enum组合而成
*/
#[derive(Debug, Clone)]
pub struct Event {
    pub _type : EventType,
    pub _meta : EventMeta,
}
/*
    枚举分为两部分 
        - 各种系统事件
        - 用户自定义事件
    编辑此类的同时注意修改 EventSystem 中的 match 语句块
*/
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum EventType {
    // OS Events
    // KeyboardClick(key : u8)
    KeyBoardDown,
    KeyBoardUp,
    // MouseClick(x : f32, y : f32)
    MouseLeftDown,
    MouseLeftUp,
    MouseRightDown,
    MouseRightUp,
    // MouseScroll(bias : f32)
    MouseScroll,
    // CustomEvents
    // TODO : IMPLEMENT EM
    UnimplementedType,
}
#[derive(Debug, Clone)]
pub enum EventMeta {
    Key(u32),
    Offset(bool),
    Position(f32, f32),
    None,
    // TODO : IMPLEMENT EM
    UNIMPLEMENTED,
}