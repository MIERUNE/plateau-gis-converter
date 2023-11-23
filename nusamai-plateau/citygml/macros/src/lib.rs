extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput, LitByteStr};

const CITYGML_ATTR_IDENT: &str = "citygml";

fn generate_citygml_struct_model(
    derive_input: &DeriveInput,
    data_struct: &DataStruct,
) -> Result<TokenStream, syn::Error> {
    let mut attribute_arms = Vec::new();
    let mut chlid_arms = Vec::new();

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
                    let path: LitByteStr = value.parse()?;

                    if path.value().starts_with(b"@") {
                        // xml attributes
                        attribute_arms.push(quote! {
                            #path => {
                                self.id = <#field_ty as citygml::CityGMLAttribute>::parse_attr_value(
                                    std::str::from_utf8(value).unwrap(),
                                )?;
                                Ok(())
                            }
                        });
                    } else {
                        // xml child elements
                        chlid_arms.push(quote! {
                            #path => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        });
                    }
                }
                Ok(())
            })?;
        }
    }

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let struct_name = &derive_input.ident;

    let attr_parsing = (!attribute_arms.is_empty()).then(|| {
        quote! {
            st.parse_attributes(|name, value| match name {
                #(#attribute_arms)*
                _ => Ok(()),
            })?;
        }
    });

    Ok(quote! {
        impl #impl_generics ::citygml::CityGMLElement for #struct_name #ty_generics #where_clause {
            fn parse<R: std::io::BufRead>(&mut self, st: &mut ::citygml::SubTreeReader<R>) -> Result<(), ::citygml::ParseError> {
                #attr_parsing

                st.parse_children(|st| {
                    match st.current_path() {
                        #(#chlid_arms)*
                        _ => Ok(()),
                    }
                })
            }
        }
    })
}

fn generate_citygml_enum_model(
    derive_input: &DeriveInput,
    data_enum: &DataEnum,
) -> Result<TokenStream, syn::Error> {
    let mut child_arms = Vec::new();
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
                    let path: LitByteStr = value.parse()?;

                    let arm = quote! {
                        #path => {
                            let mut v: #field_ty = Default::default();
                            <#field_ty as CityGMLElement>::parse(&mut v, st)?;
                            *self = Self::#variant_ident(v);
                            Ok(())
                        }
                    };
                    child_arms.push(arm);
                }
                Ok(())
            })?;
        }
    }

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let struct_name = &derive_input.ident;

    let tokens = quote! {
        impl #impl_generics ::citygml::CityGMLElement for #struct_name #ty_generics #where_clause {
            fn parse<R: ::std::io::BufRead>(&mut self, st: &mut ::citygml::SubTreeReader<R>) -> Result<(), ::citygml::ParseError> {
                st.parse_children(|st| {
                    match st.current_path() {
                        #(#child_arms)*
                        _ => Ok(()),
                    }
                })
            }
        }
    };

    Ok(tokens)
}

fn generate_citygml_model(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    match &derive_input.data {
        syn::Data::Struct(data_sturct) => generate_citygml_struct_model(derive_input, data_sturct),
        syn::Data::Enum(data_sturct) => generate_citygml_enum_model(derive_input, data_sturct),
        _ => Err(syn::Error::new_spanned(
            derive_input,
            "target must be struct or enum",
        )),
    }
}

#[proc_macro_derive(CityGMLElement, attributes(citygml))]
pub fn derive_citygml_model(token: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match generate_citygml_model(&parse_macro_input!(token)) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
