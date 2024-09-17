use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
// This function is called when the compiler sees #[derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // construct a syntax tree from the Rust code
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, my name is {}", stringify!(#name))
            }
        }
    };
    gen.into()
}