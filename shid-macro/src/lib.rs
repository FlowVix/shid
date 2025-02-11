extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

fn add_trait_bounds(mut generics: syn::Generics) -> syn::Generics {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(crate::parser::Parse));
        }
    }
    generics
}

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);

    fn convert_fields(fields: syn::Fields) -> proc_macro2::TokenStream {
        match fields {
            syn::Fields::Named(fields_named) => {
                let mut fields = quote! {};
                for i in fields_named.named {
                    let name = i.ident.unwrap();
                    let typ = i.ty;
                    fields.extend(quote! { #name: <#typ as crate::parser::Parse>::parse(parser), });
                }
                quote! { { #fields } }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let mut fields = quote! {};
                for i in fields_unnamed.unnamed {
                    let typ = i.ty;
                    fields.extend(quote! { <#typ as crate::parser::Parse>::parse(parser), });
                }
                quote! { ( #fields ) }
            }
            syn::Fields::Unit => quote! {},
        }
    }

    let out = match item {
        syn::Item::Struct(item) => {
            let name = item.ident;
            let generics = add_trait_bounds(item.generics);
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            let fields = convert_fields(item.fields);

            quote! {
                impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
                    fn parse(parser: &mut crate::parser::Parser) -> Self {
                        Self #fields
                    }
                }
            }
        }
        syn::Item::Enum(item) => {
            let name = item.ident;
            let generics = add_trait_bounds(item.generics);
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            quote! {
                impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
                    fn parse(parser: &mut crate::parser::Parser) -> Self {
                        Self #fields
                    }
                }
            }
        }
        _ => panic!("uh oh stinkyyy"),
    };

    out.into()
}
