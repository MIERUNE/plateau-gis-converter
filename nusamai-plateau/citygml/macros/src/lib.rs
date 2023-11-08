extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput, LitByteStr};

const CITYGML_ATTR_IDENT: &str = "citygml";

fn generate_citygml_struct_model(
    derive_input: &DeriveInput,
    data_struct: &DataStruct,
) -> Result<TokenStream, syn::Error> {
    let mut arms = Vec::new();
    for field in &data_struct.fields {
        let Some(field_ident) = &field.ident else {
            continue;
        };
        let field_ty = &field.ty;
        for attr in &field.attrs {
            if !attr.path().is_ident(CITYGML_ATTR_IDENT) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("path") {
                    let value = meta.value()?;
                    let s: LitByteStr = value.parse()?;
                    let arm = quote! {
                        Some(#s) => {
                            <#field_ty as CityGMLModel>::parse(&mut self.#field_ident, &mut st)?;
                        }
                    };
                    arms.push(arm);
                }
                Ok(())
            })?;
        }
    }

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let struct_name = &derive_input.ident;

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics ::citygml::CityGMLModel for #struct_name #ty_generics #where_clause {
            fn parse<R: std::io::BufRead>(&mut self, st: &mut ::citygml::SubTreeReader<R>) -> Result<(), ::citygml::ParseError> {
                let mut st = st.start_new_subtree();
                // st.parse_children(|c| {
                //     match c {
                //         #(#arms)*
                //         Some(_) => (),
                //         None => return Ok(()),
                //     };
                // })?;
                loop {
                    match st.get_next()? {
                        #(#arms)*
                        Some(_) => (),
                        None => return Ok(()),
                    }
                }
            }
        }
    };
    Ok(tokens)
}

fn generate_citygml_enum_model(
    derive_input: &DeriveInput,
    data_enum: &DataEnum,
) -> Result<TokenStream, syn::Error> {
    let mut arms = Vec::new();
    for variant in &data_enum.variants {
        if variant.fields.len() > 1 {
            return Err(syn::Error::new_spanned(
                variant,
                "variant must not have two or more fields",
            ));
        }
        if variant.fields.is_empty() {
            continue;
        }
        let field = variant.fields.iter().next().unwrap();
        let field_ty = &field.ty;
        let variant_ident = &variant.ident;

        for attr in &variant.attrs {
            if !attr.path().is_ident(CITYGML_ATTR_IDENT) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("path") {
                    let value = meta.value()?;
                    let s: LitByteStr = value.parse()?;
                    let arm = quote! {
                        Some(#s) => {
                            let mut v: #field_ty = Default::default();
                            <#field_ty as CityGMLModel>::parse(&mut v, &mut st)?;
                            *self = Self::#variant_ident(v);
                        }
                    };
                    arms.push(arm);
                }
                Ok(())
            })?;
        }
    }

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let struct_name = &derive_input.ident;

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics ::citygml::CityGMLModel for #struct_name #ty_generics #where_clause {
            fn parse<R: ::std::io::BufRead>(&mut self, st: &mut ::citygml::SubTreeReader<R>) -> Result<(), ::citygml::ParseError> {
                let mut st = st.start_new_subtree();
                loop {
                    match st.get_next()? {
                        #(#arms)*
                        Some(_) => (),
                        None => return Ok(()),
                    }
                }
            }
        }
    };

    Ok(tokens)
}

fn generate_citygml_model(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    match &derive_input.data {
        syn::Data::Struct(data_sturct) => generate_citygml_struct_model(derive_input, data_sturct),
        syn::Data::Enum(data_sturct) => generate_citygml_enum_model(derive_input, data_sturct),
        _ => Err(syn::Error::new_spanned(derive_input, "must be struct")),
    }
}

#[proc_macro_derive(CityGMLModel, attributes(citygml))]
pub fn derive_citygml_model(token: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match generate_citygml_model(&parse_macro_input!(token)) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
