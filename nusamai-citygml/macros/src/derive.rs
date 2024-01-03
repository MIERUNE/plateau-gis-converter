//! CityGMLElement derive macro

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Error, LitByteStr, LitStr};

use crate::StereoType;

const CITYGML_ATTR_IDENT: &str = "citygml";

pub(crate) fn derive_citygml_element(token: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match generate_citygml_impl(&parse_macro_input!(token)) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn generate_citygml_impl(derive_input: &DeriveInput) -> Result<TokenStream, Error> {
    match &derive_input.data {
        Data::Struct(data) => generate_citygml_impl_for_struct(derive_input, data),
        Data::Enum(data) => generate_citygml_impl_for_enum(derive_input, data),
        _ => Err(Error::new_spanned(
            derive_input,
            "target must be struct or enum",
        )),
    }
}

fn generate_citygml_impl_for_struct(
    derive_input: &DeriveInput,
    struct_data: &DataStruct,
) -> Result<TokenStream, Error> {
    let mut attribute_arms = Vec::new();
    let mut chlid_arms = Vec::new();
    let mut into_object_stmts = Vec::new();
    let mut geom_into_object_expr = quote! { None };
    let mut id_value = quote!(None);
    let struct_ident = &derive_input.ident;
    let mut typename = String::from(stringify!(derive_input.ident));
    let mut ty = StereoType::Feature;
    let mut allow_extra = true; // FIXME

    for attr in &derive_input.attrs {
        if !attr.path().is_ident(CITYGML_ATTR_IDENT) {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let name: LitStr = meta.value()?.parse()?;
                typename = name.value();
                Ok(())
            } else if meta.path.is_ident("type") {
                let ty_ident: Ident = meta.value()?.parse()?;
                ty = match ty_ident.to_string().as_str() {
                    "feature" => StereoType::Feature,
                    "data" => StereoType::Data,
                    _ => {
                        return Err(meta.error("feature or data expected"));
                    }
                };
                Ok(())
            } else if meta.path.is_ident("allow_extra") {
                allow_extra = true;
                Ok(())
            } else {
                Ok(())
            }
        })?;
    }

    // Scan struct fields
    for field in &struct_data.fields {
        let mut into_obj_generated = false;

        let Some(field_ident) = &field.ident else {
            continue;
        };

        let field_ty = &field.ty;
        for attr in &field.attrs {
            if !attr.path().is_ident(CITYGML_ATTR_IDENT) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("required") {
                    // TODO: required
                    Ok(())
                }
                else if meta.path.is_ident("codelist") {
                    // TODO: codelist
                    let _codelist: LitStr = meta.value()?.parse()?;
                    Ok(())
                }
                else if meta.path.is_ident("path") {
                    let path: LitByteStr = meta.value()?.parse()?;

                    if path.value().iter().filter(|c| c == &&b'/').count() > 1 {
                        return Err(meta.error("path must not contain more than one '/'"));
                    }

                    if path.value().starts_with(b"@") {
                        // XML attributes (e.g. @gml:id)
                        attribute_arms.push(quote! {
                            #path => {
                                self.#field_ident = <#field_ty as nusamai_citygml::CityGMLAttribute>::parse_attr_value(
                                    std::str::from_utf8(value).unwrap(),
                                )?;
                                Ok(())
                            }
                        });
                        if field_ident == "id" {
                            id_value = quote! {
                                self.id
                            };
                        } else {
                            into_object_stmts.push(
                                quote! {
                                    if let Some(v) = self.#field_ident.into_object() {
                                        attributes.insert(stringify!(#field_ident).into(), v);
                                    }
                                }
                            )
                        }
                    } else {
                        // XML child elements (e.g. bldg:measuredHeight)

                        // if the path contains '/', add the first path as a 'noop' arm.
                        if let Some(pos) = path.value().iter().position(|&x| x == b'/') {
                            let prefix = LitByteStr::new(&path.value()[..pos], path.span());
                            chlid_arms.push(
                                quote! {
                                    #prefix => Ok(()),
                                }
                            );
                        };

                        chlid_arms.push(
                            quote! {
                                #path => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                            }
                        );

                        if !into_obj_generated {
                            // Use the first path component as the attribute name
                            // e.g. "bldg:interiorRoom/bldg:Room" -> "bldg:interiorRoom"
                            let path_value = path.value();
                            let pos_slash = path_value.iter().position(|&x| x == b'/').unwrap_or(path_value.len());
                            let name = std::str::from_utf8(&path_value[..pos_slash]).unwrap();

                            into_object_stmts.push(
                                quote! {
                                    if let Some(v) = self.#field_ident.into_object() {
                                        attributes.insert(#name.into(), v);
                                    }
                                }
                            );
                            into_obj_generated = true;
                        }
                    }
                    Ok(())
                }
                else if meta.path.is_ident("geom") {
                    let prefix: LitByteStr = meta.value()?.parse()?;

                    let mut add_arm = |lod: u8, name: &[u8], geomtype: &str| {
                        let mut c = prefix.value().clone();
                        c.push(b':');
                        c.extend(name);
                        let pat = LitByteStr::new(c.as_ref(), prefix.span());
                        let geomtype = format_ident!("{}", geomtype);

                        chlid_arms.push(quote! {
                            #pat => st.parse_geometric_attr(&mut self.#field_ident, #lod, ::nusamai_citygml::geometry::GeometryParseType::#geomtype),
                        });
                    };

                    add_arm(0, b"lod0Point", "Point"); // only in CityGML 2.0
                    add_arm(0, b"lod0RoofEdge", "MultiSurface"); // only in CityGML 2.0
                    add_arm(0, b"lod0FootPrint", "MultiSurface"); // only in CityGML 2.0
                    add_arm(0, b"lod0MultiCurve", "MultiCurve");
                    add_arm(2, b"lod2MultiCurve", "MultiCurve");
                    add_arm(3, b"lod3MultiCurve", "MultiCurve");
                    add_arm(4, b"lod4MultiCurve", "MultiCurve"); // only in CityGML 2.0
                    add_arm(1, b"lod1Solid", "Solid");
                    add_arm(2, b"lod2Solid", "Solid");
                    add_arm(3, b"lod3Solid", "Solid");
                    add_arm(4, b"lod4Solid", "Solid"); // only in CityGML 2.0
                    add_arm(0, b"lod0MultiSurface", "MultiSurface");
                    add_arm(1, b"lod1MultiSurface", "MultiSurface");
                    add_arm(2, b"lod2MultiSurface", "MultiSurface");
                    add_arm(3, b"lod3MultiSurface", "MultiSurface");
                    add_arm(4, b"lod4MultiSurface", "MultiSurface"); // only in CityGML 2.0
                    add_arm(0, b"lod0Geometry", "Geometry"); // only in CityGML 2.0
                    add_arm(1, b"lod1Geometry", "Geometry"); // only in CityGML 2.0
                    add_arm(2, b"lod2Geometry", "Geometry"); // only in CityGML 2.0
                    add_arm(3, b"lod3Geometry", "Geometry"); // only in CityGML 2.0
                    add_arm(4, b"lod4Geometry", "Geometry"); // only in CityGML 2.0
                    add_arm(0, b"tin", "Triangulated");

                    geom_into_object_expr = quote! {
                        Some(self.#field_ident)
                    };

                    Ok(())
                } else {
                    Err(meta.error("unrecognized argument"))
                }
            })?;
        }
    }

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let attr_parsing = (!attribute_arms.is_empty()).then(|| {
        quote! {
            st.parse_attributes(|name, value| match name {
                #(#attribute_arms)*
                _ => Ok(()),
            })?;
        }
    });

    let into_object_impl = match ty {
        StereoType::Feature => {
            quote! {
                Some(::nusamai_citygml::object::Value::Feature(
                    ::nusamai_citygml::object::Feature {
                        typename: #typename.into(),
                        id: #id_value,
                        attributes: {
                            let mut attributes = ::std::collections::HashMap::new();
                            #(#into_object_stmts)*
                            attributes
                        },
                        geometries: #geom_into_object_expr,
                    }
                ))
            }
        }
        StereoType::Data => {
            quote! {
                Some(::nusamai_citygml::object::Value::Data(
                    ::nusamai_citygml::object::Data {
                        typename: #typename.into(),
                        attributes: {
                            let mut attributes = ::std::collections::HashMap::new();
                            #(#into_object_stmts)*
                            attributes
                        },
                    }
                ))
            }
        }
        _ => unreachable!(),
    };

    let element_type = match ty {
        StereoType::Feature => quote! { ::nusamai_citygml::ElementType::FeatureType },
        StereoType::Data => quote! { ::nusamai_citygml::ElementType::DataType },
        _ => unreachable!(),
    };

    let extra_arm = match allow_extra {
        true => quote! { Ok(()) },
        false => quote! {
            Err(::nusamai_citygml::ParseError::SchemaViolation(
                format!("unexpected element: {}", String::from_utf8_lossy(st.current_path())),
            ))
        },
    };

    Ok(quote! {
        impl #impl_generics ::nusamai_citygml::CityGMLElement for #struct_ident #ty_generics #where_clause {
            const ELEMENT_TYPE: ::nusamai_citygml::ElementType = #element_type;

            fn parse<R: std::io::BufRead>(&mut self, st: &mut ::nusamai_citygml::SubTreeReader<R>) -> Result<(), ::nusamai_citygml::ParseError> {
                #attr_parsing

                st.parse_children(|st| {
                    match st.current_path() {
                        #(#chlid_arms)*
                        _ => #extra_arm,
                    }
                })
            }

            fn into_object(self) -> Option<::nusamai_citygml::object::Value> {
                #into_object_impl
            }
        }
    })
}

fn generate_citygml_impl_for_enum(
    derive_input: &DeriveInput,
    enum_data: &DataEnum,
) -> Result<TokenStream, Error> {
    let mut child_arms = Vec::new();
    let mut into_object_arms = Vec::new();

    // Scan enum variants
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

                    child_arms.push(quote! {
                        #path => {
                            let mut v: #field_ty = Default::default();
                            <#field_ty as CityGMLElement>::parse(&mut v, st)?;
                            *self = Self::#variant_ident(v);
                            Ok(())
                        }
                    });
                    into_object_arms.push(quote! {
                        Self::#variant_ident(v) => v.into_object()
                    });
                }
                Ok(())
            })?;
        }
    }

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let struct_name = &derive_input.ident;

    Ok(quote! {
        impl #impl_generics ::nusamai_citygml::CityGMLElement for #struct_name #ty_generics #where_clause {
            const ELEMENT_TYPE: ::nusamai_citygml::ElementType = ::nusamai_citygml::ElementType::PropertyType;

            fn parse<R: ::std::io::BufRead>(&mut self, st: &mut ::nusamai_citygml::SubTreeReader<R>) -> Result<(), ::nusamai_citygml::ParseError> {
                st.parse_children(|st| {
                    match st.current_path() {
                        #(#child_arms)*
                        _ => Ok(()),
                    }
                })
            }

            fn into_object(self) -> Option<::nusamai_citygml::object::Value> {
                match self {
                    #(#into_object_arms,)*
                    _ => None,
                }
            }
        }
    })
}
