extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Error, LitByteStr};

const CITYGML_ATTR_IDENT: &str = "citygml";

fn generate_citygml_struct_model(
    derive_input: &DeriveInput,
    struct_data: &DataStruct,
) -> Result<TokenStream, Error> {
    let mut attribute_arms = Vec::new();
    let mut chlid_arms = Vec::new();

    for field in &struct_data.fields {
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
                    let path: LitByteStr = meta.value()?.parse()?;

                    if path.value().starts_with(b"@") {
                        // XML attributes (e.g. @gml:id)
                        attribute_arms.push(quote! {
                            #path => {
                                self.id = <#field_ty as citygml::CityGMLAttribute>::parse_attr_value(
                                    std::str::from_utf8(value).unwrap(),
                                )?;
                                Ok(())
                            }
                        });
                    } else {
                        // XML child elements (e.g. bldg:measuredHeight)
                        chlid_arms.push(quote! {
                            #path => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        });
                    }
                    Ok(())
                }
                else if meta.path.is_ident("auto_geom") {
                    let prefix: LitByteStr = meta.value()?.parse()?;

                    let mut add_arm = |lod: u8, name: &[u8], geomtype: &str  | {
                        let mut c = prefix.value().clone();
                        c.push(b':');
                        c.extend(name);
                        let pat = LitByteStr::new(c.as_ref(), prefix.span());
                        let geomtype = format_ident!("{}", geomtype);

                        chlid_arms.push(quote! {
                            #pat => st.parse_geometric_attr(&mut self.#field_ident, #lod, ::citygml::geometric::GeometryParseType::#geomtype),
                        });
                    };

                    add_arm(1, b"lod1Solid", "Solid");
                    add_arm(1, b"lod1MultiSurface", "MultiSurface");
                    add_arm(2, b"lod2MultiSurface", "MultiSurface");
                    add_arm(3, b"lod3MultiSurface", "MultiSurface");
                    add_arm(4, b"lod4MultiSurface", "MultiSurface");
                    add_arm(1, b"lod1Geometry", "Geometry");
                    add_arm(2, b"lod2Geometry", "Geometry");
                    add_arm(3, b"lod3Geometry", "Geometry");
                    add_arm(4, b"lod4Geometry", "Geometry");
                    add_arm(1, b"tin", "Triangulated");

                    Ok(())
                } else {
                    Err(meta.error("unrecognized attribute"))
                }
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
    enum_data: &DataEnum,
) -> Result<TokenStream, Error> {
    let mut child_arms = Vec::new();
    for variant in &enum_data.variants {
        if variant.fields.len() > 1 {
            return Err(Error::new_spanned(
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

fn generate_citygml_model(derive_input: &DeriveInput) -> Result<TokenStream, Error> {
    match &derive_input.data {
        Data::Struct(data) => generate_citygml_struct_model(derive_input, data),
        Data::Enum(data) => generate_citygml_enum_model(derive_input, data),
        _ => Err(Error::new_spanned(
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
