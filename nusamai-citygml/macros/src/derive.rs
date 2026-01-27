//! CityGmlElement derive macro

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Error,
    LitByteStr, LitStr,
};

use crate::Stereotype;

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

const HASH_CHAR_SKIP: usize = 4;
const HASH_CHAR_TAKE: usize = 3;
const HASH_MASK: u32 = 0x7f;

fn hash(s: &[u8]) -> u8 {
    let h = s
        .iter()
        .skip(HASH_CHAR_SKIP)
        .take(HASH_CHAR_TAKE)
        .fold(5381u32, |a, c| a.wrapping_mul(33) ^ *c as u32);
    (h & HASH_MASK) as u8
}

fn generate_citygml_impl_for_struct(
    derive_input: &DeriveInput,
    struct_data: &DataStruct,
) -> Result<TokenStream, Error> {
    let mut attribute_arms = Vec::new();
    let mut child_arms = Vec::new();
    let mut into_object_stmts = Vec::new();
    let mut prop_stmts = Vec::new();

    let mut geom_into_object_stmt = quote! { Vec::new() };
    let mut id_value = quote!(String::new());
    let struct_ident = &derive_input.ident;
    let mut typename = String::from(stringify!(derive_input.ident));
    let mut ty = Stereotype::Feature;
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
                    "feature" => Stereotype::Feature,
                    "data" => Stereotype::Data,
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
                        let path = LitByteStr::new(&c, prefix.span());
                        let geomtype = format_ident!("{}", geomtype);
                        let hash = hash(&c);

                        child_arms.push(quote! {
                            (#hash, #path) => st.parse_geometric_attr(&mut self.#field_ident, #lod, ::nusamai_citygml::geometry::GeometryParseType::#geomtype),
                        });
                    };

                    add_arm(0, b"lod0Point", "Point");
                    add_arm(0, b"lod0MultiCurve", "MultiCurve");
                    add_arm(1, b"lod1MultiCurve", "MultiCurve");
                    add_arm(2, b"lod2MultiCurve", "MultiCurve");
                    add_arm(3, b"lod3MultiCurve", "MultiCurve");
                    add_arm(4, b"lod4MultiCurve", "MultiCurve"); // only in CityGML 2.0
                    add_arm(1, b"lod1Solid", "Solid");
                    add_arm(2, b"lod2Solid", "Solid");
                    add_arm(3, b"lod3Solid", "Solid");
                    add_arm(4, b"lod4Solid", "Solid"); // only in CityGML 2.0
                    add_arm(1, b"lod1MultiSolid", "MultiSolid");
                    add_arm(2, b"lod2MultiSolid", "MultiSolid");
                    add_arm(3, b"lod3MultiSolid", "MultiSolid");
                    add_arm(4, b"lod4MultiSolid", "MultiSolid");
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
                            add_arm( 0, b"lod0Network", "CompositeCurve");
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
                        self.#field_ident
                    };

                    Ok(())
                } else if meta.path.is_ident("generics") {
                    let mut add_arm = |path: &[u8]| {
                        let pat = LitByteStr::new(path, attr.span());
                        let hash = hash(path);
                        child_arms.push(quote! {
                            (#hash, #pat) => <#field_ty as CityGmlElement>::parse(&mut self.#field_ident, st),
                        });
                    };

                    add_arm(b"gen:dateAttribute");
                    add_arm(b"gen:doubleAttribute");
                    add_arm(b"gen:genericAttributeSet");
                    add_arm(b"gen:intAttribute");
                    add_arm(b"gen:measureAttribute");
                    add_arm(b"gen:stringAttribute");
                    add_arm(b"gen:uriAttribute");

                    into_object_stmts.push(
                        quote! {
                            if let Some(v) = self.#field_ident.into_object() {
                                attributes.insert("gen:genericAttribute".into(), v);
                            }
                        }
                    );
                    prop_stmts.push(
                        quote! {
                            attributes.insert("gen:genericAttribute".into(), <#field_ty as CityGmlElement>::collect_schema(schema));
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
                            self.#field_ident = <#field_ty as nusamai_citygml::CityGmlAttribute>::parse_attribute_value(
                                std::str::from_utf8(value).unwrap(),
                                ctx
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
                                attributes.insert(#name.into(), <#field_ty as CityGmlElement>::collect_schema(schema));
                            }
                        );
                    }
                } else {
                    // XML child elements (e.g. bldg:measuredHeight)

                    // if the path contains '/', add the first path as a 'noop' arm.
                    if let Some(pos) = path_value.iter().position(|&x| x == b'/') {
                        let prefix = LitByteStr::new(&path_value[..pos], path.span());
                        let hash = hash(&prefix.value());
                        child_arms.push(quote! {
                            (#hash, #prefix) => Ok(()),
                        });
                    };

                    let hash = hash(&path.value());
                    child_arms.push(quote! {
                        (#hash, #path) => <#field_ty as CityGmlElement>::parse(&mut self.#field_ident, st),
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
                                    let mut ty_ref = <#field_ty as CityGmlElement>::collect_schema(schema);
                                    if ty_ref.min_occurs == 0 { ty_ref.min_occurs = 1; }
                                    attributes.insert(#name.into(), ty_ref);
                                },
                                false => quote! {
                                    attributes.insert(#name.into(), <#field_ty as CityGmlElement>::collect_schema(schema));
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
            st.parse_attributes(|name, value, ctx| match name {
                #(#attribute_arms)*
                _ => Ok(()),
            })?;
        }
    });

    let into_object_impl = match ty {
        Stereotype::Feature => {
            quote! {
                Some(::nusamai_citygml::object::Value::Object(
                    ::nusamai_citygml::object::Object {
                        typename: #typename.into(),
                        attributes: {
                            let mut attributes = ::nusamai_citygml::object::Map::default();
                            #(#into_object_stmts)*
                            attributes
                        },
                        stereotype: ::nusamai_citygml::object::ObjectStereotype::Feature {
                            id: #id_value,
                            geometries: #geom_into_object_stmt,
                        }
                    }
                ))
            }
        }
        Stereotype::Data => {
            quote! {
                Some(::nusamai_citygml::object::Value::Object(
                    ::nusamai_citygml::object::Object {
                        typename: #typename.into(),
                        attributes: {
                            let mut attributes = ::nusamai_citygml::object::Map::default();
                            #(#into_object_stmts)*
                            attributes
                        },
                        stereotype: ::nusamai_citygml::object::ObjectStereotype::Data,
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
        Stereotype::Feature => quote! { Feature },
        Stereotype::Data => quote! { Data },
        _ => unreachable!(),
    };
    let stereotypedef = match ty {
        Stereotype::Feature => quote! { FeatureTypeDef },
        Stereotype::Data => quote! { DataTypeDef },
        _ => unreachable!(),
    };

    Ok(quote! {
        impl #impl_generics ::nusamai_citygml::CityGmlElement for #struct_ident #ty_generics #where_clause {
            #[inline(never)]
            fn parse<R: std::io::BufRead>(&mut self, st: &mut ::nusamai_citygml::SubTreeReader<R>) -> Result<(), ::nusamai_citygml::ParseError> {
                #attr_parsing

                st.parse_children(move |st| {
                    let path: &[u8] = &st.current_path();
                    let hash = (path.iter().skip(#HASH_CHAR_SKIP).take(#HASH_CHAR_TAKE).fold(5381u32, |a, c| a.wrapping_mul(33) ^ *c as u32) & #HASH_MASK) as u8;
                    match (hash, path) {
                        #(#child_arms)*
                        _ => #extra_arm,
                    }
                })
            }

            #[inline(never)]
            fn into_object(self) -> Option<::nusamai_citygml::object::Value> {
                #into_object_impl
            }

            fn collect_schema(schema: &mut ::nusamai_citygml::schema::Schema) -> ::nusamai_citygml::schema::Attribute {
                let key = #typename;
                if schema.types.get(key).is_none() {
                    // TODO: use entry API
                    schema.types.insert(
                        key.into(),
                        ::nusamai_citygml::schema::TypeDef::#stereotype(::nusamai_citygml::schema::#stereotypedef {
                            ..Default::default()
                        })
                    );
                    let mut attributes = ::nusamai_citygml::schema::Map::default();
                    #(#prop_stmts)*
                    match schema.types.get_mut(key).unwrap() {
                        ::nusamai_citygml::schema::TypeDef::#stereotype(t) => t.attributes = attributes,
                        _ => unreachable!(),
                    }
                }
                ::nusamai_citygml::schema::Attribute::new(::nusamai_citygml::schema::TypeRef::Named(key.into()))
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

    let mut typename = String::from(stringify!(derive_input.ident));

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
                let _: Ident = meta.value()?.parse()?;
                Ok(())
            } else {
                Ok(())
            }
        })?;
    }

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
            <#field_ty as CityGmlElement>::collect_schema(schema),
        });

        for attr in &variant.attrs {
            if !attr.path().is_ident(CITYGML_ATTR_IDENT) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("path") {
                    let value = meta.value()?;
                    let path: LitByteStr = value.parse()?;
                    let hash = hash(&path.value());

                    child_arms.push(if enum_data.variants.len() < 6 {
                        quote! {
                            (_, #path) => {
                                let mut v: #field_ty = Default::default();
                                <#field_ty as CityGmlElement>::parse(&mut v, st)?;
                                *self = Self::#variant_ident(v);
                                Ok(())
                            }
                        }
                    } else {
                        quote! {
                            (#hash, #path) => {
                                let mut v: #field_ty = Default::default();
                                <#field_ty as CityGmlElement>::parse(&mut v, st)?;
                                *self = Self::#variant_ident(v);
                                Ok(())
                            }
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
        impl #impl_generics ::nusamai_citygml::CityGmlElement for #struct_name #ty_generics #where_clause {
            #[inline(never)]
            fn parse<R: ::std::io::BufRead>(&mut self, st: &mut ::nusamai_citygml::SubTreeReader<R>) -> Result<(), ::nusamai_citygml::ParseError> {
                st.parse_children(|st| {
                    let path: &[u8] = &st.current_path();
                    let hash = (path.iter().skip(#HASH_CHAR_SKIP).take(#HASH_CHAR_TAKE).fold(5381u32, |a, c| a.wrapping_mul(33) ^ *c as u32) & #HASH_MASK) as u8;
                    match (hash, path) {
                        #(#child_arms)*
                        _ => Ok(()),
                    }
                })
            }

            #[inline(never)]
    fn into_object(self) -> Option<::nusamai_citygml::object::Value> {
                match self {
                    #(#into_object_arms,)*
                    _ => None,
                }
            }

            fn collect_schema(schema: &mut ::nusamai_citygml::schema::Schema) -> ::nusamai_citygml::schema::Attribute {
                let key = #typename;
                if schema.types.get(key).is_none() {
                    // TODO: use entry API
                    let members = vec![
                        #(#choice_types)*
                    ];
                    schema.types.insert(
                        key.into(),
                        ::nusamai_citygml::schema::TypeDef::Property(::nusamai_citygml::schema::PropertyTypeDef {
                            members
                        })
                    );
                }
                ::nusamai_citygml::schema::Attribute::new(::nusamai_citygml::schema::TypeRef::Named(key.into()))
            }
        }
    })
}
