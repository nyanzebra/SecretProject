use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

extern crate proc_macro;

#[proc_macro_derive(Component)]
pub fn my_fn_like_proc_macro(input: TokenStream) -> TokenStream {
    // 1. Use syn to parse the input tokens into a syntax tree.
    // 2. Use quote to generate new tokens based on what we parsed.
    // 3. Return the generated tokens.

    let DeriveInput { ident, .. } = syn::parse(input).unwrap();
    let expanded = quote! {
        impl ecs::component::Component for #ident {
            fn id() -> ecs::component::ComponentId {
                ecs::component::COMPONENT_GENERATOR
                    .lock()
                    .expect("lock")
                    .generate(std::stringify!(#ident))
            }
        }
    };
    expanded.into()
}
