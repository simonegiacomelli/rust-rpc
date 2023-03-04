use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    dbg!(input.clone());
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
    // input
}

// https://doc.rust-lang.org/book/ch19-06-macros.html
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}


#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    dbg!("========================== sql! start ===================");
    dbg!(input.clone());
    dbg!("========================== sql! end ===================");
    let stream = quote! {
         {
             println!("Hello, sql! macro");
            5
         }
    };
    stream.into()
}