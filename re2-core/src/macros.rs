// !Attention 此处把Component的位置做了Entity和DataStruct的区分
#[macro_export()]
macro_rules! add_component_to_entity {
    ($entity:ident,$(($name:ident$(, $field:ident : $value:expr)*))*) => {
        $(add_component_to_entity!($entity,$name$(, $field : $value)*);)*
    };
    ($entity:ident,$name:ident$(, $field:ident : $value:expr)*) => {
        let mut _t = create_component!($name$(, $field : $value)*);
        _t.add_to_entity(&$entity);
        $name::get_pool_singleton().insert($entity.id, _t);
    }
}

#[macro_export()]
macro_rules! add_component_to_data_struct {
    ($(($name:ident$(, $field:ident : $value:expr)*))*) => {
        $(add_component_to_data_struct!($name$(,$field : $value)*);)*
    };    
    ($name:ident$(, $field:ident : $value:expr)*) => {
        let mut _t = create_component!($name$(, $field : $value)*);
        $name::get_stack_singleton().push(_t);
    }
}

#[macro_export()]
macro_rules! create_component {
    ($name:ident$(, $field:ident : $value:expr)*) => {
        $name{
            $($field : $value,)*
            parent_id : 0
        }
    }
}

#[macro_export()]
macro_rules! component_struct { 
    ($name:ident{$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, ComponentMacro, Clone)]
        pub struct $name {
            $(pub $field : $t,)*
            pub parent_id : i32,
        }
    }
}