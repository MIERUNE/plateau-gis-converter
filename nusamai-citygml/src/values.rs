use crate::object::{self, Value};
use crate::parser::{ParseError, SubTreeReader};
use crate::schema;
use crate::CityGMLElement;
pub use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::io::BufRead;

// type aliases
// type aliases
pub type Date = chrono::NaiveDate;
pub type Length = Measure; // Length is almost same as Measure
pub type GYear = String; // TODO?
pub type GYearMonth = String; // TODO?
pub type MeasureOrNullList = String; // TODO?
pub type BuildingLODType = String; // TODO?
pub type DoubleList = String; // TODO?
pub type LODType = u64; // TODO?

impl CityGMLElement for String {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.parse_text()?);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::String(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::String)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq)]
pub struct URI(String);

impl URI {
    pub fn new(s: &str) -> Self {
        Self(s.into())
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl CityGMLElement for URI {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.0.push_str(st.parse_text()?);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::String(self.0))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::URI)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Code {
    value: String,
    code: String,
    // pub code_space: Option<String>,
}

impl Code {
    pub fn new(value: String, code: String) -> Self {
        Self { value, code }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn code(&self) -> &str {
        &self.code
    }
}

impl CityGMLElement for Code {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let code_space = st.find_codespace_attr();
        let code = st.parse_text()?.to_string();
        self.code = code.clone();

        if let Some(code_space) = code_space {
            if let Some(base_url) = st.context().source_url() {
                match st
                    .context()
                    .code_resolver()
                    .resolve(base_url, &code_space, &code)
                {
                    Ok(Some(v)) => {
                        self.value = v;
                        return Ok(());
                    }
                    Ok(None) => {}
                    Err(_) => {
                        // FIXME
                        log::warn!("Failed to lookup code {} form {}", code, code_space);
                        self.value = code;
                        return Ok(());
                    }
                }
            }
        }
        self.value = code;
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Code(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Code)
    }
}

impl CityGMLElement for i64 {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected an integer, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Integer(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Integer)
    }
}

impl CityGMLElement for u64 {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected an integer, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Integer(self as i64))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::NonNegativeInteger)
    }
}

impl CityGMLElement for f64 {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a floating point number, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Double(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Double)
    }
}

impl CityGMLElement for bool {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?.trim();
        match text {
            "1" | "true" | "True" | "TRUE" => {
                *self = true;
                Ok(())
            }
            "0" | "false" | "False" | "FALSE" => {
                *self = false;
                Ok(())
            }
            _ => Err(ParseError::InvalidValue(format!(
                "Expected a boolean value, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Boolean(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Boolean)
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Measure {
    value: f64,
    // pub uom: Option<String>,
}

impl Measure {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl CityGMLElement for Measure {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                self.value = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a floating point number, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Measure(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Measure)
    }
}

impl CityGMLElement for Date {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match Date::parse_from_str(text, "%Y-%m-%d") {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a date in the format YYYY-MM-DD, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Date(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Date)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct Point {
    // TODO
}

pub type Vector = Point;

impl CityGMLElement for Point {
    #[inline]
    fn parse<R: BufRead>(&mut self, _st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        // TODO
        todo!();
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Point(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Point)
    }
}

impl<T: CityGMLElement + Default + std::fmt::Debug> CityGMLElement for Option<T> {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        if self.is_some() {
            return Err(ParseError::SchemaViolation(format!(
                "{} must not occur two or more times.",
                String::from_utf8_lossy(st.current_path()),
            )));
        }
        let mut v: T = Default::default();
        T::parse(&mut v, st)?;
        *self = Some(v);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        match self {
            Some(v) => v.into_object(),
            None => None,
        }
    }

    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute {
        let mut ty_ref = T::collect_schema(schema);
        ty_ref.min_occurs = 0;
        ty_ref
    }
}

impl<T: CityGMLElement + Default> CityGMLElement for Vec<T> {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let mut v: T = Default::default();
        <T as CityGMLElement>::parse(&mut v, st)?;
        self.push(v);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        if self.is_empty() {
            None
        } else {
            Some(Value::Array(
                self.into_iter().filter_map(|v| v.into_object()).collect(),
            ))
        }
    }

    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute {
        let mut ty_ref = T::collect_schema(schema);
        ty_ref.min_occurs = 0;
        ty_ref.max_occurs = None;
        ty_ref
    }
}

impl<T: CityGMLElement + Default> CityGMLElement for Box<T> {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        <T as CityGMLElement>::parse(self, st)?;
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        (*self).into_object()
    }

    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute {
        T::collect_schema(schema)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GenericAttribute {
    pub string_attrs: Vec<(String, String)>,
    pub int_attrs: Vec<(String, i64)>,
    pub double_attrs: Vec<(String, f64)>,
    pub measure_attrs: Vec<(String, Measure)>,
    pub code_attrs: Vec<(String, Code)>,
    pub date_attrs: Vec<(String, Date)>,
    pub uri_attrs: Vec<(String, URI)>,
    pub generic_attr_set: Vec<(String, GenericAttribute)>,
}

impl CityGMLElement for GenericAttribute {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        match st.current_path() {
            b"gen:stringAttribute" | b"gen:StringAttribute" => {
                self.string_attrs.push(parse_value(st)?)
            }
            b"gen:intAttribute" | b"gen:IntAttribute" => self.int_attrs.push(parse_value(st)?),
            b"gen:doubleAttribute" | b"gen:DoubleAttribute" => {
                self.double_attrs.push(parse_value(st)?)
            }
            b"gen:measureAttribute" | b"gen:MeasureAttribute" => {
                self.measure_attrs.push(parse_value(st)?)
            }
            b"gen:codeAttribute" | b"gen:CodeAttribute" => self.code_attrs.push(parse_value(st)?),
            b"gen:dateAttribute" | b"gen:DateAttribute" => self.date_attrs.push(parse_value(st)?),
            b"gen:uriAttribute" | b"gen:UriAttribute" => self.uri_attrs.push(parse_value(st)?),
            b"gen:genericAttributeSet" | b"gen:GenericAttributeSet" => {
                self.generic_attr_set.push(parse_generic_set(st)?)
            }
            _ => {
                return Err(ParseError::SchemaViolation(format!(
                    "generic attributes are expected but found {}",
                    String::from_utf8_lossy(st.current_path()),
                )))
            }
        }
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        let mut map = object::Map::default();
        map.extend(
            self.string_attrs
                .into_iter()
                .map(|(k, v)| (k, Value::String(v))),
        );
        map.extend(
            self.int_attrs
                .into_iter()
                .map(|(k, v)| (k, Value::Integer(v))),
        );
        map.extend(
            self.double_attrs
                .into_iter()
                .map(|(k, v)| (k, Value::Double(v))),
        );
        map.extend(
            self.measure_attrs
                .into_iter()
                .map(|(k, v)| (k, Value::Measure(v))),
        );
        map.extend(
            self.code_attrs
                .into_iter()
                .map(|(k, v)| (k, Value::Code(v))),
        );
        map.extend(
            self.date_attrs
                .into_iter()
                .map(|(k, v)| (k, Value::Date(v))),
        );
        map.extend(self.uri_attrs.into_iter().map(|(k, v)| (k, Value::URI(v))));
        map.extend(
            self.generic_attr_set
                .into_iter()
                .flat_map(|(k, v)| match v.into_object() {
                    Some(Value::Data(data)) => Some((k, Value::Data(data))),
                    _ => None,
                }),
        );
        Some(Value::Data(object::Data {
            typename: "gen:genericAttribute".into(),
            attributes: map,
        }))
    }

    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute {
        let key = "gen:genericAttribute";
        if schema.types.get(key).is_none() {
            schema.types.insert(
                key.into(),
                schema::TypeDef::Data(schema::DataTypeDef {
                    attributes: Default::default(),
                    any: true,
                }),
            );
        }
        schema::Attribute::new(schema::TypeRef::Named(key.into()))
    }
}

fn parse_value<T, R: BufRead>(st: &mut SubTreeReader<R>) -> Result<(String, T), ParseError>
where
    T: CityGMLElement + Default,
{
    let mut name = None;
    let mut value = None;
    st.parse_attributes(|k, v| {
        // CityGML 2.0
        if k == b"@name" {
            name = Some(String::from_utf8_lossy(v).into());
        }
        Ok(())
    })?;
    st.parse_children(|st| {
        match st.current_path() {
            // CityGML 3.0
            b"gen:name" => {
                name = Some(st.parse_text()?.to_string());
            }
            b"gen:value" => {
                let mut v: T = Default::default();
                v.parse(st)?;
                value = Some(v);
            }
            _ => {}
        }
        Ok(())
    })?;

    match (name, value) {
        (Some(name), Some(value)) => Ok((name, value)),
        _ => Err(ParseError::SchemaViolation(
            "generic attribute must have both name and value.".to_string(),
        )),
    }
}

fn parse_generic_set<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<(String, GenericAttribute), ParseError> {
    let mut name = None;
    let mut value: Option<GenericAttribute> = None;
    st.parse_attributes(|k, v| {
        if k == b"@name" {
            name = Some(String::from_utf8_lossy(v).into());
        }
        Ok(())
    })?;
    st.parse_children(|st| {
        match st.current_path() {
            b"gen:name" => {
                name = Some(st.parse_text()?.to_string());
            }
            b"gen:codeSpace" => {
                // TODO
            }
            _ => {
                if value.is_none() {
                    value = Some(Default::default());
                }
                value.as_mut().unwrap().parse(st)?;
            }
        };
        Ok(())
    })?;

    match (name, value) {
        (Some(name), Some(value)) => Ok((name, value)),
        _ => Err(ParseError::SchemaViolation(
            "GenericAttributeSet must have a name and at least one value.".to_string(),
        )),
    }
}
