use std::collections::HashMap;
use crate::entity_component::Component;

#[derive(Debug)]
pub struct Pool<K : std::cmp::Eq + std::hash::Hash, V> {
    _data : HashMap<K, V>,
}

impl<K : std::cmp::Eq + std::hash::Hash, V> Pool<K, V> {
    pub fn new() -> Pool<K, V> {
        Pool{_data : HashMap::new()}
    }
    pub fn get_data(&self) -> &HashMap<K, V> {
        &self._data
    }
    pub fn get_mut_data(&mut self) -> &mut HashMap<K, V> {
        &mut self._data
    }
    pub fn insert(&mut self, key : K, value : V) {
        self._data.insert(key, value);
    }
    pub fn get(&self, key : &K) -> Option<&V> {
        self._data.get(key)
    }
    pub fn get_mut(&mut self, key : &K) -> Option<&mut V> {
        self._data.get_mut(key)
    }
    pub fn clear(&mut self) {
        self._data.clear();
    }
}

#[derive(Debug)]
pub struct Stack<T : Component> {
    _data : Vec<T>,
}

impl<T : Component> Stack<T> {    
    pub fn new() -> Stack<T> {
        Stack{_data : vec!()}
    }
    pub fn push(&mut self, value : T) {
        self._data.push(value);        
    }
    pub fn pop(&mut self) {
        self._data.pop();
    }
    pub fn clear(&mut self) {
        self._data.clear();
    }
    pub fn get_data(&self) -> &Vec<T> {
        &self._data
    }
    pub fn get_mut_data(&mut self) -> &mut Vec<T> {
        &mut self._data
    }
}

#[derive(Debug)]
pub struct DirtyVec<T : Ord> {
    _data : Vec<T>,
    _dirty_flag : bool,
}

impl<T : Ord> DirtyVec<T>{
    pub fn new() -> DirtyVec<T> {
        DirtyVec{ _data : Vec::new(), _dirty_flag : false }
    }
    pub fn push(&mut self, value : T) {
        self._data.push(value);
        self._dirty_flag = true;
    }
    pub fn get_data(&mut self) -> &Vec<T> {
        self.sort_if_dirty();
        &self._data
    }    
    pub fn clear(&mut self) {
        self._data.clear();
        self._dirty_flag = false;
    }
    fn sort_if_dirty(&mut self) {
        match self._dirty_flag {
            true => {
                self._data.sort();
                self._dirty_flag = false;
            },
            _ => {}
        }
    }
}