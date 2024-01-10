use crate::object::{self, Value};
use crate::parser::{ParseError, SubTreeReader};
use crate::{CityGMLElement, ElementType};
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
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.parse_text()?);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::String(self))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Clone)]
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
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.0.push_str(st.parse_text()?);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::String(self.0))
    }
}

#[derive(Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
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
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
                        return Err(ParseError::InvalidValue(format!(
                            "Failed to resolve code: {} {}",
                            code_space, code
                        )));
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
}

impl CityGMLElement for i64 {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

impl CityGMLElement for u64 {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

impl CityGMLElement for f64 {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

impl CityGMLElement for bool {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct Measure {
    pub value: f64,
    // pub uom: Option<String>,
}

impl CityGMLElement for Measure {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

impl CityGMLElement for Date {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Clone)]
pub struct Point {
    // TODO
}

impl CityGMLElement for Point {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, _st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        // TODO
        todo!();
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Point(self))
    }
}

impl<T: CityGMLElement + Default + std::fmt::Debug> CityGMLElement for Option<T> {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
}

impl<T: CityGMLElement + Default> CityGMLElement for Vec<T> {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

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
    const ELEMENT_TYPE: ElementType = ElementType::DataType;

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
}

fn parse_value<T, R: BufRead>(st: &mut SubTreeReader<R>) -> Result<(String, T), ParseError>
where
    T: CityGMLElement + Default,
{
    let mut name = None;
    let mut value = None;
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
