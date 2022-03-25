use super::*;

#[cfg(test)]
pub mod test_event {
    use re2_metadata::{Event, EventType, EventMeta};
    use std::collections::HashMap;
    #[test]
    fn test_event_enum_hashmap(){
        let mut map : HashMap<EventType, String> = HashMap::new();
        map.insert(EventType::KeyBoardDown,  String::from("KeyBoardDown"));
        map.insert(EventType::KeyBoardUp, String::from("KeyBoardUp"));
        let event1 = Event{_type : EventType::KeyBoardDown, _meta : EventMeta::Key(1)};
        let event2 = Event{_type : EventType::KeyBoardDown, _meta : EventMeta::Key(2)};
        let event3 = Event{_type : EventType::KeyBoardUp, _meta : EventMeta::Position(1.1, 2.2)};
        println!("e1 : type :{:?} ev : {:?}, e2 : type :{:?} ev : {:?}, e3 : type :{:?} ev : {:?}",
            map.get(&event1._type),&event1, 
            map.get(&event2._type),&event2, 
            map.get(&event3._type),&event3
        )
    }
}
