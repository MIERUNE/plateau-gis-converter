//! Parsing utilities for CityGML 2.0 (and possibly 3.0)

extern crate proc_macro;

mod derive;
mod type_attrs;

use proc_macro::TokenStream;

#[proc_macro_derive(CityGmlElement, attributes(citygml))]
pub fn derive_citygml_element(token: TokenStream) -> TokenStream {
    derive::derive_citygml_element(token)
}

#[proc_macro_attribute]
pub fn citygml_feature(args: TokenStream, input: TokenStream) -> TokenStream {
    type_attrs::citygml_type(Stereotype::Feature, args, input)
}

#[proc_macro_attribute]
pub fn citygml_data(args: TokenStream, input: TokenStream) -> TokenStream {
    type_attrs::citygml_type(Stereotype::Data, args, input)
}

#[proc_macro_attribute]
pub fn citygml_property(args: TokenStream, input: TokenStream) -> TokenStream {
    type_attrs::citygml_type(Stereotype::Property, args, input)
}

#[derive(Clone, Copy)]
pub(crate) enum Stereotype {
    Feature,
    Data,
    Property,
}
