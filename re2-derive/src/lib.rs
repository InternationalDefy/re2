extern crate proc_macro;
extern crate syn;

// #[macro_use]
// extern crate quote;
// #[macro_use]
// extern crate lazy_static;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

// use syn::spanned::Spanned;
// use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

mod impl_component_macro;
mod impl_lazy_static_generation;

use impl_component_macro::*;
use impl_lazy_static_generation::*;

#[proc_macro_derive(LazyStaticSingleton)]
pub fn lazy_static_singleton_macro_derive(input : TokenStream) -> TokenStream {
    let ast : syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    
    let impl_tokenstream : proc_macro2::TokenStream = impl_singleton_tokenstream(&ast);
    let gen = quote!{
        impl #name {
            #impl_tokenstream
        }
    };
    let output : TokenStream2 = gen.into();
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(GenerateLazyStaticVec)]
pub fn generate_lazy_static_vec_macro_derive(input : TokenStream) -> TokenStream{
    let ast : syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    
    let impl_tokenstream : proc_macro2::TokenStream = impl_generate_lazy_vec_singleton_tokenstream(&ast);
    let gen = quote!{
        impl #name {
            #impl_tokenstream
        }
    };
    let output : TokenStream2 = gen.into();
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(ComponentMacro)]
pub fn component_macro_derive (input : TokenStream) -> TokenStream {
    let ast : syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    // let function_tokenstream = impl_getters_setters_macro(&ast);
    let trait_tokenstream : proc_macro2::TokenStream = impl_component_trait_tokenstream(&ast);
    let stack_tokenstream : proc_macro2::TokenStream = impl_component_stack_and_pool_singleton_tokenstream(&ast);
    // let creator_tokenstream = impl_component_creator_tokenstream(&ast);
    let gen = quote!{
        #trait_tokenstream
        impl #name {
            #stack_tokenstream
        }
    };
    //using while testing
    // println!("{:?}", gen.to_string());
    let output : TokenStream2 = gen.into();
    proc_macro::TokenStream::from(output)
}

