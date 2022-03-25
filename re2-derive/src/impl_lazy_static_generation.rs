use crate::*;

// 此fn将某Pool类生成其lazy_static!
pub fn impl_singleton_tokenstream(ast : &syn::DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    
    let singleton_string = String::from("SINGLETON");
    let singleton_name = syn::Ident::new(&singleton_string, name.span());

    quote!{
        pub fn get_singleton() -> MutexGuard<'static,  #name>{
            lazy_static!{
                static ref #singleton_name : Mutex<#name> = Mutex::new(#name::new());
            };
            #singleton_name.lock().unwrap()
        }
    }
}

// 此fn将某单类生成其stack_singleton
pub fn impl_generate_lazy_vec_singleton_tokenstream(ast : &syn::DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    
    let vec_singleton_string = String::from("VEC_SINGLETON");
    let vec_singleton_name = syn::Ident::new(&vec_singleton_string, name.span());

    quote!{
        pub fn get_vec_singleton() -> MutexGuard<'static,  Vec<#name>>{
            lazy_static!{
                static ref #vec_singleton_name : Mutex<Vec<#name>> = Mutex::new(Vec::new());
            };
            #vec_singleton_name.lock().unwrap()
        }
    }
}
