use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Component)]
pub fn component_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_component_trait(ast)
}

fn impl_component_trait(ast: DeriveInput) -> TokenStream {
    // get the struct identifier
    let ident = ast.ident;

    // generate impl
    (quote::quote! {
        impl Component for #ident {}
    }).into()
}
