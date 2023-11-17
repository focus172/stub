extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;

#[proc_macro_derive(Stub)]
pub fn derive_stub(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse_macro_input!(item as syn::DeriveInput);

    match data {
        syn::Data::Struct(data) => {
            let fs: Vec<TokenStream2> = data
                .fields
                .into_iter()
                .map(|f| {
                    let i = f.ident.unwrap();
                    quote::quote!(#i: Stub::stub(), )
                })
                .collect();

            let params = generics.params;
            let where_clause = generics.where_clause;

            quote::quote!(
                impl #params Stub for #ident #params #where_clause {
                    fn stub() -> Self {
                        Self {
                            #( #fs )*
                        }
                    }
                }
            )
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }
    .into()
}
