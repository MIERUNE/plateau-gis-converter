use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    meta::ParseNestedMeta, parse::Parser, parse_macro_input, parse_quote, Data, DeriveInput, Error,
    LitByteStr, LitStr,
};

use crate::Stereotype;

/// Arguments for `#[citygml_feature(...)]` and `#[citygml_data(...)]`
#[derive(Default)]
struct FeatureArgs {
    name: Option<LitStr>,       // "bldg:Building"
    prefix: Option<LitByteStr>, // b"bldg"
    is_cityobj: bool,
}

impl FeatureArgs {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        self.is_cityobj = true;
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
        } else if meta.path.is_ident("noncityobj") {
            self.is_cityobj = false;
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    }
}

pub(crate) fn citygml_type(
    ty: Stereotype,
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
        #[derive(Default, ::nusamai_citygml::CityGmlElement)]
        #input
    }
    .into()
}

fn add_named_field(pos: usize, fields: &mut syn::FieldsNamed, body: TokenStream) {
    fields
        .named
        .insert(pos, syn::Field::parse_named.parse2(body).unwrap())
}

fn modify(ty: &Stereotype, args: &FeatureArgs, input: &mut DeriveInput) -> Result<(), Error> {
    match &args.name {
        Some(name) => {
            input.attrs.push(syn::parse_quote! {
                #[citygml(name = #name)]
            });
        }
        None => return Err(Error::new_spanned(input, "name is required")),
    };

    input.attrs.push(match &ty {
        Stereotype::Feature => {
            syn::parse_quote! { #[citygml(type = feature)] }
        }
        Stereotype::Data => {
            syn::parse_quote! { #[citygml(type = data)] }
        }
        Stereotype::Property => {
            syn::parse_quote! { #[citygml(type = property)] }
        }
    });

    match &mut input.data {
        Data::Struct(data) => {
            // for #[citygml_feature] and #[citygml_data]

            match ty {
                Stereotype::Feature | Stereotype::Data => {}
                _ => return Err(Error::new_spanned(input, "target must be struct")),
            }

            if let syn::Fields::Named(ref mut fields) = data.fields {
                if let Stereotype::Feature = ty {
                    // for #[citygml_feature]

                    let prefix = args.prefix.as_ref().unwrap();

                    // temporary hack
                    let geom_prefix = match args.name.as_ref().unwrap().value().as_str() {
                        "uro:UndergroundBuilding" => LitByteStr::new(b"bldg", prefix.span()),
                        "uro:Waterway" => LitByteStr::new(b"tran", prefix.span()),
                        "uro:Appurtenance" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:Manhole" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:Handhole" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:Pipe" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:WaterPipe" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:SewerPipe" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:OilGasChemicalsPipe" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:ThermalPipe" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:Cable" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:ElectricityCable" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:TelecommunicationsCable" => LitByteStr::new(b"frn", prefix.span()),
                        "uro:Duct" => LitByteStr::new(b"frn", prefix.span()),
                        _ => prefix.clone(),
                    };

                    let mut pos = 0;

                    if args.is_cityobj {
                        add_named_field(
                            pos,
                            fields,
                            quote! {
                                #[citygml(geom = #geom_prefix)]
                                pub geometries: ::nusamai_citygml::GeometryRefs
                            },
                        );
                        pos += 1;
                    }

                    add_named_field(
                        pos,
                        fields,
                        quote! {
                            #[citygml(path = b"@gml:id", required)]
                            pub id: String
                        },
                    );
                    pos += 1;

                    //// CityGML 3.0
                    // add_named_field(
                    //     fields,
                    //     quote! {
                    //         #[citygml(path = b"gml:identifier")]
                    //         pub id: Option<String>
                    //     },
                    // );
                    // pos += 1;

                    add_named_field(
                        pos,
                        fields,
                        quote! {
                            #[citygml(path = b"gml:description")]
                            pub description: Option<String>
                        },
                    );
                    pos += 1;

                    add_named_field(
                        pos,
                        fields,
                        quote! {
                            #[citygml(path = b"gml:name")]
                            pub name: Vec<::nusamai_citygml::Code>
                        },
                    );
                    pos += 1;

                    if args.is_cityobj {
                        add_named_field(
                            pos,
                            fields,
                            quote! {
                                #[citygml(path = b"core:creationDate")]
                                pub creation_date: Option<::nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                            },
                        );
                        pos += 1;

                        add_named_field(
                            pos,
                            fields,
                            quote! {
                                #[citygml(path = b"core:terminationDate")]
                                pub termination_date: Option<::nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                            },
                        );
                        pos += 1;

                        // // CityGML 3.0
                        // add_named_field(
                        //     pos,
                        //     fields,
                        //     quote! {
                        //         #[citygml(path = b"core:validFrom")]
                        //         pub valid_from: Option<::nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                        //     },
                        // );
                        // pos += 1;
                        //
                        // // CityGML 3.0
                        // add_named_field(
                        //     pos,
                        //     fields,
                        //     quote! {
                        //         #[citygml(path = b"core:validTo")]
                        //         pub valid_to: Option<::nusamai_citygml::Date> // TODO: DateTime (CityGML 3.0)
                        //     },
                        // );
                        // pos += 1;

                        add_named_field(
                            pos,
                            fields,
                            quote! {
                                #[citygml(generics)]
                                pub generic_attribute: ::nusamai_citygml::GenericAttribute
                            },
                        );
                    }
                }
            }
        }
        Data::Enum(_data) => match ty {
            Stereotype::Property => {
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
