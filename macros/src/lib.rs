use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Component)]
pub fn component_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // generate
    (quote::quote! {
        impl Component for #ident {}
    })
    .into()
}
