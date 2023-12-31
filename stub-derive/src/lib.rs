extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;

#[proc_macro_derive(Stub, attributes(stub))]
pub fn derive_stub(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse_macro_input!(item as syn::DeriveInput);

    let name = ident;

    let params = generics.params;
    let where_clause = generics.where_clause;

    match data {
        syn::Data::Struct(data) => {
            let fields = parse_feilds(data.fields);

            quote::quote!(
                impl #params Stub for #name #params #where_clause {
                    fn stub() -> Self {
                        Self #fields
                    }
                }
            )
        }
        syn::Data::Enum(data) => {
            let syn::Variant { ident, fields, .. } = data
                .variants
                .into_iter()
                .find(|v| v.attrs.iter().any(|a| a.path().is_ident("stub")))
                .expect("Tag one of the variants with #[stub]");

            let fields = parse_feilds(fields);

            quote::quote!(
                impl #params Stub for #name #params #where_clause {
                    fn stub() -> Self {
                        Self::#ident #fields
                    }
                }
            )
        }
        syn::Data::Union(data) => {
            // compile_error!("oops");
            syn::Error::new_spanned(data.union_token, "Stub trait cannot be derived for unions")
                .into_compile_error()
        }
    }
    .into()
}

fn parse_feilds<I: IntoIterator<Item = syn::Field>>(fields: I) -> TokenStream2 {
    let mut has_feilds = true;

    let fs: Vec<TokenStream2> = fields
        .into_iter()
        .map(|f| {
            if let Some(ident) = f.ident {
                quote::quote!(#ident: Stub::stub())
            } else {
                has_feilds = false;
                quote::quote!(Stub::stub())
            }
        })
        .collect();

    if has_feilds {
        quote::quote! { { #( #fs, )* } }
    } else {
        quote::quote! { ( #( #fs, )* ) }
    }
}
