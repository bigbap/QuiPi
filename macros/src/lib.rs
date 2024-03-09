use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Component)]
pub fn component_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // generate
    (quote::quote! {
        impl Component for #ident {
            fn id() -> ComponentId {
                ComponentId::new::<Self>()
            }
        }
    })
    .into()
}

#[proc_macro_derive(Resource)]
pub fn resource_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // generate
    (quote::quote! {
        impl Resource for #ident {
            fn name(&self) -> &str {
                std::any::type_name::<Self>()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    })
    .into()
}
