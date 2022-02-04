extern crate proc_macro;

use std::sync::Mutex;
use lazy_static::lazy_static;

use syn;
use proc_macro::TokenStream;
use quote::quote;


// This is gross, but I'm pretty sure there isn't a standardised way to store global state for rust macros
// someone send me an email with a link to some docs/examples at twhite17t@gmail.com if I am wrong about this. I wasn't able to find anything after hours of googling.
lazy_static!{
    static ref ID_COUNT: Mutex<u32> = Mutex::new(0);

}


#[proc_macro_derive(Id)]
pub fn get_id(input: TokenStream) -> TokenStream{

    let ast = syn::parse(input).unwrap();

    impl_get_id_macro(&ast)

}

fn impl_get_id_macro(ast: &syn::DeriveInput) -> TokenStream{

    let mut id_count = ID_COUNT.lock().unwrap();
    *id_count += 1;
    let id_count_copy = *id_count;
    let name = &ast.ident;
    let gen = quote!{
        impl Id for #name {
            fn get_id() -> u32 {
                #id_count_copy
            }

            fn get_instance_id(&self) -> u32 {
                #id_count_copy
            }
        }
    };

    gen.into()
}






