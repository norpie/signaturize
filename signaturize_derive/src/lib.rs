use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(Signature)]
pub fn derive_signature(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields_signature = match input.data {
        syn::Data::Struct(ref data) => {
            let fields = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! {
                    signaturize::Signature::Field {
                        name: Box::new(signaturize::Signature::Type(stringify!(#name))),
                        value: Box::new(<#ty>::signature())
                    }
                }
            });
            quote! {
                vec![#(#fields),*]
            }
        }
        _ => panic!("Signature derive macro only supports structs"),
    };

    let expanded = quote! {
        impl signaturize::Signaturize for #name {
            fn signature() -> signaturize::Signature {
                signaturize::Signature::Struct {
                    name: Box::new(signaturize::Signature::Type(stringify!(#name))),
                    fields: #fields_signature
                }
            }
        }
    };

    TokenStream::from(expanded)
}
