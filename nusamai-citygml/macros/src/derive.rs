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
    let mut prop_stmts = Vec::new();

    let mut geom_into_object_stmt = quote! { None };
    let mut id_value = quote!(String::new());
    let struct_ident = &derive_input.ident;
    let mut typename = String::from(stringify!(derive_input.ident));
    let mut ty = StereoType::Feature;
    let mut allow_extra = false;

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
        let mut required = false;

        let Some(field_ident) = &field.ident else {
            continue;
        };

        let field_ty = &field.ty;
        for attr in &field.attrs {
            if !attr.path().is_ident(CITYGML_ATTR_IDENT) {
                continue;
            }

            let mut path: Option<LitByteStr> = None;

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("required") {
                    required = true;
                    Ok(())
                }
                else if meta.path.is_ident("codelist") {
                    // TODO: codelist
                    let _codelist: LitStr = meta.value()?.parse()?;
                    Ok(())
                }
                else if meta.path.is_ident("path") {
                    let p: LitByteStr = meta.value()?.parse()?;
                    if p.value().iter().filter(|c| c == &&b'/').count() > 1 {
                        return Err(meta.error("path must not contain more than one '/'"));
                    }
                    path = Some(p);
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

                    add_arm(0, b"lod0Point", "Point");
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

                    // only in CityGML 2.0
                    match &prefix.value()[..] {
                        b"bldg" => {
                            add_arm(0, b"lod0RoofEdge", "MultiSurface");
                            add_arm(0, b"lod0FootPrint", "MultiSurface");
                        }
                        b"tran" => {
                            add_arm( 0, b"lod0Network", "MultiCurve");
                        }
                        b"uro" => {
                            if typename.as_str() == "uro:RailwayTrackAttribute" {
                                add_arm(2, b"lod2Network", "MultiCurve");
                                add_arm(3, b"lod3Network", "MultiCurve");
                            }
                        }
                        b"wtr" => {
                            add_arm( 2, b"lod2Surface", "Surface");
                            add_arm( 3, b"lod3Surface", "Surface");
                        }
                        b"dem" => {
                            add_arm( 0, b"tin", "Triangulated");
                        }
                        // lod*TerrainIntersection
                        // lod*ImplicitRepresentation
                        _ => {}
                    }

                    geom_into_object_stmt = quote! {
                        Some(self.#field_ident)
                    };

                    Ok(())
                } else if meta.path.is_ident("generics") {
                    chlid_arms.push(quote! {
                        b"gen:dateAttribute" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        b"gen:doubleAttribute" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        b"gen:genericAttributeSet" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        b"gen:intAttribute" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        b"gen:measureAttribute" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        b"gen:stringAttribute" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                        b"gen:uriAttribute" => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                    });
                    into_object_stmts.push(
                        quote! {
                            if let Some(v) = self.#field_ident.into_object() {
                                attributes.insert("gen:genericAttribute".into(), v);
                            }
                        }
                    );
                    prop_stmts.push(
                        quote! {
                            attributes.insert("gen:genericAttribute".into(), <#field_ty as CityGMLElement>::collect_schema(schema));
                        }
                    );
                    Ok(())
                } else {
                    Err(meta.error("unrecognized argument"))
                }
            })?;

            if let Some(path) = path {
                let path_value = path.value();

                if path_value.starts_with(b"@") {
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
                        let name = std::str::from_utf8(&path_value).unwrap();
                        into_object_stmts.push(quote! {
                            if let Some(v) = self.#field_ident.into_object() {
                                attributes.insert(stringify!(#field_ident).into(), v);
                            }
                        });
                        prop_stmts.push(
                            quote! {
                                attributes.insert(#name.into(), <#field_ty as CityGMLElement>::collect_schema(schema));
                            }
                        );
                    }
                } else {
                    // XML child elements (e.g. bldg:measuredHeight)

                    // if the path contains '/', add the first path as a 'noop' arm.
                    if let Some(pos) = path_value.iter().position(|&x| x == b'/') {
                        let prefix = LitByteStr::new(&path_value[..pos], path.span());
                        chlid_arms.push(quote! {
                            #prefix => Ok(()),
                        });
                    };

                    chlid_arms.push(quote! {
                        #path => <#field_ty as CityGMLElement>::parse(&mut self.#field_ident, st),
                    });

                    if !into_obj_generated {
                        // Use the first path component as the attribute name
                        // e.g. "bldg:interiorRoom/bldg:Room" -> "bldg:interiorRoom"
                        let pos_slash = path_value
                            .iter()
                            .position(|&x| x == b'/')
                            .unwrap_or(path_value.len());
                        let name = std::str::from_utf8(&path_value[..pos_slash]).unwrap();

                        into_object_stmts.push(quote! {
                            if let Some(v) = self.#field_ident.into_object() {
                                attributes.insert(#name.into(), v);
                            }
                        });
                        prop_stmts.push(
                            match required {
                                true => quote! {
                                    let mut ty_ref = <#field_ty as CityGMLElement>::collect_schema(schema);
                                    if ty_ref.min_occurs == 0 { ty_ref.min_occurs = 1; }
                                    attributes.insert(#name.into(), ty_ref);
                                },
                                false => quote! {
                                    attributes.insert(#name.into(), <#field_ty as CityGMLElement>::collect_schema(schema));
                                }
                            }
                        );
                        into_obj_generated = true;
                    }
                }
            }
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
                            let mut attributes = ::nusamai_citygml::object::Map::default();
                            #(#into_object_stmts)*
                            attributes
                        },
                        geometries: #geom_into_object_stmt,
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
                            let mut attributes = ::nusamai_citygml::object::Map::default();
                            #(#into_object_stmts)*
                            attributes
                        },
                    }
                ))
            }
        }
        _ => unreachable!(),
    };

    let extra_arm = match allow_extra {
        true => quote! { Ok(()) },
        false => quote! {
            Err(::nusamai_citygml::ParseError::SchemaViolation(
                format!("unexpected element: {}", String::from_utf8_lossy(st.current_absolute_path())),
            ))
        },
    };

    let stereotype = match ty {
        StereoType::Feature => quote! { Feature },
        StereoType::Data => quote! { Data },
        _ => unreachable!(),
    };

    Ok(quote! {
        impl #impl_generics ::nusamai_citygml::CityGMLElement for #struct_ident #ty_generics #where_clause {
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

            fn collect_schema(schema: &mut ::nusamai_citygml::schema::Schema) -> ::nusamai_citygml::schema::TypeRef {
                let key = #typename;
                if schema.types.get(key).is_none() {
                    // TODO: use entry API
                    schema.types.insert(
                        key.into(),
                        ::nusamai_citygml::schema::TypeDef {
                            stereotype: ::nusamai_citygml::schema::StereoType::#stereotype,
                            attributes: Default::default(),
                            any: false,
                        });
                    let mut attributes = ::nusamai_citygml::schema::Map::default();
                    #(#prop_stmts)*
                    schema.types.get_mut(key).unwrap().attributes = attributes;
                }
                ::nusamai_citygml::schema::TypeRef::new(::nusamai_citygml::schema::Type::Ref(key.into()))
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
    let mut choice_types = Vec::new();

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
        choice_types.push(quote! {
            <#field_ty as CityGMLElement>::collect_schema(schema),
        });

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

            fn collect_schema(schema: &mut ::nusamai_citygml::schema::Schema) -> ::nusamai_citygml::schema::TypeRef {
                ::nusamai_citygml::schema::TypeRef::new(::nusamai_citygml::schema::Type::Property(
                    vec![
                        #(#choice_types)*
                    ]
                ))
            }
        }
    })
}
