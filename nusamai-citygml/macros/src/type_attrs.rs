use crate::StereoType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::meta::ParseNestedMeta;
use syn::{parse::Parser, parse_macro_input, Data, DeriveInput, Error, LitStr};
use syn::{parse_quote, LitByteStr};

/// Arguments for `#[citygml_feature(...)]` and `#[citygml_data(...)]`
#[derive(Default)]
struct FeatureArgs {
    name: Option<LitStr>,       // "bldg:Building"
    prefix: Option<LitByteStr>, // b"bldg"
}

impl FeatureArgs {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("name") {
            let s: LitStr = meta.value()?.parse()?;
            self.prefix = Some(LitByteStr::new(
                s.value()
                    .split_once(':')
                    .ok_or_else(|| meta.error("ns prefix is missing"))?
                    .0
                    .as_bytes(),
                s.span(),
            ));
            self.name = Some(s);
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    }
}

pub(crate) fn citygml_type(
    ty: StereoType,
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut attrs = FeatureArgs::default();
    let tea_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with tea_parser);

    let mut input = parse_macro_input!(input as DeriveInput);

    if let Err(e) = modify(&ty, &attrs, &mut input) {
        return e.to_compile_error().into();
    };

    quote! {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type"))]
        #[derive(Default, Debug, nusamai_citygml::CityGMLElement)]
        #input
    }
    .into()
}

fn add_named_field(fields: &mut syn::FieldsNamed, body: TokenStream) {
    fields
        .named
        .push(syn::Field::parse_named.parse2(body).unwrap())
}

fn modify(ty: &StereoType, args: &FeatureArgs, input: &mut DeriveInput) -> Result<(), Error> {
    match &args.name {
        Some(name) => {
            input.attrs.push(syn::parse_quote! {
                #[citygml(name = #name)]
            });
        }
        None => return Err(Error::new_spanned(input, "name is required")),
    };

    input.attrs.push(match &ty {
        StereoType::Feature => {
            syn::parse_quote! { #[citygml(type = feature)] }
        }
        StereoType::Data => {
            syn::parse_quote! { #[citygml(type = data)] }
        }
        StereoType::Property => {
            syn::parse_quote! { #[citygml(type = property)] }
        }
    });

    match &mut input.data {
        Data::Struct(data) => {
            // for #[citygml_feature] and #[citygml_data]

            match ty {
                StereoType::Feature | StereoType::Data => {}
                _ => return Err(Error::new_spanned(input, "target must be struct")),
            }

            if let syn::Fields::Named(ref mut fields) = data.fields {
                if let StereoType::Feature = ty {
                    // for #[citygml_feature]

                    let prefix = args.prefix.as_ref().unwrap();

                    // hack
                    let geom_prefix = match args.name.as_ref().unwrap().value().as_str() {
                        "uro:UndergroundBuilding" => LitByteStr::new(b"bldg", prefix.span()),
                        "uro:Waterway" => LitByteStr::new(b"tran", prefix.span()),
                        _ => prefix.clone(),
                    };

                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(geom = #geom_prefix)]
                            pub geometries: nusamai_citygml::GeometryRef
                        },
                    );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"@gml:id")]
                            pub id: Option<String>
                        },
                    );
                    //// CityGML 3.0
                    // add_named_field(
                    //     fields,
                    //     quote! {
                    //         #[citygml(path = b"gml:identifier")]
                    //         pub id: Option<String>
                    //     },
                    // );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"gml:description")]
                            pub description: Option<String>
                        },
                    );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"gml:name")]
                            pub name: Vec<String>
                        },
                    );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"core:creationDate")]
                            pub creation_date: Option<nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                        },
                    );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"core:terminationDate")]
                            pub termination_date: Option<nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                        },
                    );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"core:validFrom")]
                            pub valid_from: Option<nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                        },
                    );
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(path = b"core:validTo")]
                            pub valid_to: Option<nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                        },
                    );
                    // TODO: not implemented yet
                    add_named_field(
                        fields,
                        quote! {
                            #[citygml(generics)]
                            pub generic_attribute: nusamai_citygml::GenericAttribute
                        },
                    );
                }
            }
        }
        Data::Enum(_data) => match ty {
            StereoType::Property => {
                // for #[citygml_property]
                _data.variants.push(parse_quote! {
                    #[default]
                    Unknown
                });
            }
            _ => return Err(Error::new_spanned(input, "target must be enum")),
        },
        _ => return Err(Error::new_spanned(input, "target must be struct or enum")),
    }
    Ok(())
}
