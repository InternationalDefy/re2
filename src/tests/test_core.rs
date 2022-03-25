use super::*;

#[cfg(test)]
pub mod test_core {
    use super::*;
    #[test]
    pub fn core_component_creation() {
        use std::sync::{MutexGuard, Mutex};
        #[macro_use]
        use lazy_static::*;
        component_struct!(
            CustomComponent { 
                age : i32,
                name : String,
            }
        );
        let mut cst = create_component!{CustomComponent, age : 20, name : String::from("X")};
        println!("Component Created! {:?}", cst);
    }
    #[test]
    pub fn core_data_struct_creation() {
        component_struct!(
            CustomComponent { 
                age : i32,
                name : String,
            }
        );
        let mut cst = create_component!{CustomComponent, age : 20, name : String::from("X")};
        let mut vec = vec!(cst);
        let new_stack : Stack<CustomComponent> = Stack::new();
        println!("new_stack: {:?}", new_stack);
    }
    #[test]
    pub fn core_data_struct_add() {
        component_struct!(
            CustomComponent { 
                age : i32,
                name : String,
            }
        );
        let mut cst = create_component!{CustomComponent, age : 20, name : String::from("X")};
        let mut cst2 = create_component!{CustomComponent, age : 30, name : String::from("Y")};
        let mut add_stack : Stack<CustomComponent> = Stack::new();
        add_stack.push(cst);
        add_stack.push(cst2);
        println!("add_stack: {:?}", add_stack);
    }    
    #[test]
    pub fn core_scene_component_add(){
        component_struct!(
            CompoZero {
                age : i32,
                name : String,
            }
        );
        component_struct!(
            CompoOne {
                age : i32,
                number : i32,
            }
        );
        add_component_to_data_struct!(
            (CompoZero, age : 10, name : String::from("X"))
            (CompoZero, age : 55, name : String::from("Y"))
            (CompoOne, age : 20, number : 12)
            (CompoOne, age : 50, number : 99)
        );
        println!("{:?}", CompoZero::get_stack_singleton());
        println!("{:?}", CompoOne::get_stack_singleton());
    }
    #[test]
    pub fn core_entity_add_component(){
        component_struct!(
            CompoZero { 
                age : i32,
                name : String,
            }
        );
        component_struct!(
            CompoOne {
                age : i32,
                number : i32,
            }
        );
        let et = Entity{id : 1};
        add_component_to_entity!(et, 
            (CompoZero, age : 10, name : String::from("X"))
            (CompoZero, age : 55, name : String::from("Y"))
            (CompoOne, age : 20, number : 12)
            (CompoOne, age : 50, number : 99)
        );
        println!("{:?}", CompoZero::get_pool_singleton());
        println!("{:?}", CompoOne::get_pool_singleton());
    }    
    #[test]
    pub fn core_vec_singleton_generation() {
        use re2_derive::*; 
        #[derive(Debug, GenerateLazyStaticVec)]
        struct SingletonTypeOne {
            num : i32,
        }
        println!("{:?}", &SingletonTypeOne::get_vec_singleton());
        SingletonTypeOne::get_vec_singleton().push(SingletonTypeOne{num : 1});
        SingletonTypeOne::get_vec_singleton().push(SingletonTypeOne{num : 2});
        println!("{:?}", &SingletonTypeOne::get_vec_singleton());
    }
    #[test]
    pub fn core_singleton_generation() {
        use re2_derive::*;
        #[derive(Debug, LazyStaticSingleton)]
        struct SingletonTypeOne {
            num : i32,
        }
        impl SingletonTypeOne {
            pub fn new() -> SingletonTypeOne {
                SingletonTypeOne{num : 1}
            }
        }
        #[derive(Debug, LazyStaticSingleton)]
        struct SingletonTypeTwo {
            id : String,
        }
        impl SingletonTypeTwo {
            pub fn new() -> SingletonTypeTwo {
                SingletonTypeTwo{id : String::from("New")}
            }
        }
        println!("{}", &SingletonTypeOne::get_singleton().num);
        println!("{}", SingletonTypeTwo::get_singleton().id);        
        SingletonTypeOne::get_singleton().num = 2;
        SingletonTypeTwo::get_singleton().id = String::from("Neon");
        println!("{}", &SingletonTypeOne::get_singleton().num);
        println!("{}", &SingletonTypeTwo::get_singleton().id);
        #[derive(Debug, LazyStaticSingleton)]
        struct RandomPool {
            pool : Pool<String, i32>,
        }
        impl RandomPool {
            pub fn new() -> RandomPool {
                RandomPool {pool : Pool::new()}
            }
        }
        println!("{:?}", &RandomPool::get_singleton());
        &RandomPool::get_singleton().pool.insert(String::from("One"), 1);
        println!("{:?}", &RandomPool::get_singleton());
        &RandomPool::get_singleton().pool.insert(String::from("Two"), 2);
        println!("{:?}", &RandomPool::get_singleton());
        &RandomPool::get_singleton().pool.clear();
        println!("{:?}", &RandomPool::get_singleton());
    }
}
