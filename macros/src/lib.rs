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
        impl Resource for #ident {}

        impl AsAny for #ident {
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

#[proc_macro_derive(Schedule)]
pub fn schedule_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // generate
    (quote::quote! {
        impl Schedule for #ident {
            fn add_system(&mut self, system: BoxedSystem) {
                self.systems.push(system);
            }

            fn update(&mut self, world: &mut World) -> QPResult<()> {
                for system in self.systems.iter_mut() {
                    system(world)?;
                }

                Ok(())
            }
        }
    })
    .into()
}

#[proc_macro_derive(AsAny)]
pub fn asany_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // generate
    (quote::quote! {
        impl AsAny for #ident {
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

#[proc_macro_derive(Asset)]
pub fn asset_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // generate
    (quote::quote! {
        impl Asset for #ident {}
    })
    .into()
}
