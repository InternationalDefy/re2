use crate::*;

// impl Component for #name
pub fn impl_component_trait_tokenstream(ast : &syn::DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    quote!{
        impl Component for #name {
            fn add_to_entity(&mut self, parent_entity : &Entity) {
                self.parent_id = parent_entity.id;                
            }
            fn get_parent_id(&self) -> i32 {
                self.parent_id
            }
        }
    }
}

pub fn impl_component_stack_and_pool_singleton_tokenstream(ast : &syn::DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    
    let pool_string = String::from("get_pool_singleton");
    let pool_singleton_string = String::from("POOL_SINGLETON");
    let pool_name = syn::Ident::new(&pool_string, name.span());
    let pool_singleton_name = syn::Ident::new(&pool_singleton_string, name.span());

    let stack_string = String::from("get_stack_singleton");
    let stack_singleton_string = String::from("STACK_SINGLETON");
    let stack_name = syn::Ident::new(&stack_string, name.span());
    let stack_singleton_name = syn::Ident::new(&stack_singleton_string, name.span());

    quote!{
        pub fn #pool_name() -> MutexGuard<'static,  Pool<i32, #name>>{
            lazy_static!{
                static ref #pool_singleton_name : Mutex<Pool<i32, #name>> = Mutex::new(Pool::new());
            };
            #pool_singleton_name.lock().unwrap()
        }
        pub fn #stack_name() -> MutexGuard<'static, Stack<#name>> {
            lazy_static!{
                static ref #stack_singleton_name : Mutex<Stack<#name>> = Mutex::new(Stack::new());
            };
            #stack_singleton_name.lock().unwrap()
        }
    }
}

// pub fn impl_getters_setters_tokenstream(ast : &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let function_tokenstream = match ast.data {
//         Data::Struct(ref data) => {
//             match data.fields {
//                 Fields::Named(ref fields) => {
//                     let mut curve = quote!();
//                     for f in fields.named.iter() {
//                         let _name = &f.ident.as_ref().unwrap();
//                         let _type = &f.ty;
//                         let get_func_string = format!("get_{}", _name);
//                         let get_func_name = syn::Ident::new(&get_func_string, _name.span());
//                         let set_func_string = format!("set_{}", _name);
//                         let set_func_name = syn::Ident::new(&set_func_string, _name.span());
//                         curve = quote!{
//                             #curve
//                             pub fn #get_func_name (&self) ->&#_type {
//                                 &self.#_name
//                             }
//                             pub fn #set_func_name (&mut self, val :#_type) {
//                                 self.#_name = val;
//                             }
//                         };
//                     };
//                     curve
//                 }
//                 _ => quote!(),
//             }
//         }
//         _ => quote!(),
//     };
//     function_tokenstream.into()
// }