extern crate proc_macro;

use proc_macro::TokenStream;

mod derive;
mod type_attrs;

use type_attrs::ElementType;

#[proc_macro_derive(CityGMLElement, attributes(citygml))]
pub fn derive_citygml_element(token: TokenStream) -> TokenStream {
    derive::derive_citygml_element(token)
}

#[proc_macro_attribute]
pub fn citygml_feature(args: TokenStream, input: TokenStream) -> TokenStream {
    type_attrs::citygml_type(ElementType::Feature, args, input)
}

#[proc_macro_attribute]
pub fn citygml_data(args: TokenStream, input: TokenStream) -> TokenStream {
    type_attrs::citygml_type(ElementType::Data, args, input)
}

#[proc_macro_attribute]
pub fn citygml_property(args: TokenStream, input: TokenStream) -> TokenStream {
    type_attrs::citygml_type(ElementType::Property, args, input)
}
