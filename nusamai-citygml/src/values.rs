use std::io::BufRead;

pub use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    object::{self, Value},
    parser::{ParseError, SubTreeReader},
    schema, CityGmlAttribute, CityGmlElement, ParseContext,
};

// type aliases
pub type Date = chrono::NaiveDate;
pub type Length = Measure; // Length is almost same as Measure
pub type GYear = String; // TODO?
pub type GYearMonth = String; // TODO?
pub type MeasureOrNullList = String; // TODO?
pub type BuildingLODType = String; // TODO?
pub type DoubleList = String; // TODO?
pub type LODType = u64; // TODO?
pub type Double01 = f64; // TODO?

impl CityGmlElement for String {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.parse_text()?);
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::String(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::String)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Uri(url::Url);

impl Uri {
    pub fn new(s: url::Url) -> Self {
        Self(s)
    }
    pub fn value(&self) -> &Url {
        &self.0
    }
    pub fn into_inner(self) -> Url {
        self.0
    }
}

impl From<url::Url> for Uri {
    fn from(url: url::Url) -> Self {
        Self(url)
    }
}

impl Default for Uri {
    fn default() -> Self {
        Self(Url::parse("file:///default").unwrap())
    }
}

impl CityGmlElement for Uri {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?.to_string();
        let base_url = st.context().source_url();
        self.0 = base_url
            .join(&text)
            .map_err(|_| ParseError::InvalidValue("Invalid URI: {text}".to_string()))?;
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::String(self.0.to_string()))
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

impl CityGmlElement for Code {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let code_space = st.find_codespace_attr();
        let code = st.parse_text()?.to_string();
        self.code.clone_from(&code);

        if let Some(code_space) = code_space {
            let base_url = st.context().source_url();
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
                    log::warn!("Failed to lookup code {code} form {code_space}");
                    self.value = code;
                    return Ok(());
                }
            }
        }
        self.value = code;
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Code(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CodeOrString {
    Code(Code),
    String(String),
}

impl Default for CodeOrString {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl CityGmlElement for CodeOrString {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let code_space = st.find_codespace_attr();
        let text = st.parse_text()?.to_string();

        if let Some(code_space) = code_space {
            // Has codeSpace attribute, parse as Code
            let mut code = Code::new(text.clone(), text.clone());
            let base_url = st.context().source_url();
            match st
                .context()
                .code_resolver()
                .resolve(base_url, &code_space, &text)
            {
                Ok(Some(v)) => {
                    code.value = v;
                }
                Ok(None) => {}
                Err(_) => {
                    log::warn!("Failed to lookup code {text} form {code_space}");
                }
            }
            *self = Self::Code(code);
        } else {
            // No codeSpace attribute, parse as String
            *self = Self::String(text);
        }
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(match self {
            Self::Code(code) => Value::Code(code),
            Self::String(string) => Value::String(string),
        })
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::String)
    }
}

impl CityGmlElement for i64 {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected an integer, got {text}"
            ))),
        }
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Integer(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Integer)
    }
}

impl CityGmlElement for u64 {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected an integer, got {text}"
            ))),
        }
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::NonNegativeInteger(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::NonNegativeInteger)
    }
}

impl CityGmlElement for f64 {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a floating point number, got {text}"
            ))),
        }
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Double(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Double)
    }
}

impl CityGmlElement for bool {
    #[inline(never)]
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
                "Expected a boolean value, got {text}"
            ))),
        }
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Boolean(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Boolean)
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Measure {
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uom: Option<String>,
}

impl Measure {
    pub fn new(value: String, uom: Option<String>) -> Self {
        Self { value, uom }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn uom(&self) -> Option<&str> {
        self.uom.as_deref()
    }
}

impl CityGmlElement for Measure {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        // Parse attributes to extract 'uom' if present
        st.parse_attributes(|k, v, _| {
            if k == b"@uom" {
                self.uom = Some(String::from_utf8_lossy(v).into());
            }
            Ok(())
        })?;

        let text = st.parse_text()?;
        // check if the value is a valid float, but store as string
        if text.parse::<f64>().is_err() {
            return Err(ParseError::InvalidValue(format!(
                "Expected a floating point number, got {text}"
            )));
        }
        self.value = text.to_string();
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Measure(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Measure)
    }
}

impl CityGmlElement for Date {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match Date::parse_from_str(text, "%Y-%m-%d") {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a date in the format YYYY-MM-DD, got {text}"
            ))),
        }
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Date(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Date)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct Point {
    pub coords: [f64; 3],
}

pub type Vector = Point;

impl CityGmlElement for Point {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let s = st.parse_text()?;
        for (i, s) in s.split_ascii_whitespace().enumerate() {
            let Ok(v) = s.parse() else {
                return Err(ParseError::InvalidValue(format!(
                    "Point coordinate must be numeric value, but found: {s}"
                )));
            };
            self.coords[i] = v;
        }
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Point(self))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::Point)
    }
}

impl Point {
    pub fn coordinates(&self) -> &[f64; 3] {
        &self.coords
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LocalId(pub String);

impl LocalId {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Self(s.as_ref().to_string())
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for LocalId {
    fn from(id: String) -> Self {
        LocalId(id.trim_start_matches('#').to_string())
    }
}

impl CityGmlElement for LocalId {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let s = st.parse_text()?;
        if let Some(id) = s.strip_prefix('#') {
            let s = id.to_string();
            *self = LocalId::from(s.to_string());
            Ok(())
        } else {
            Err(ParseError::InvalidValue(format!(
                "Expected a reference starts with '#' but got {s}"
            )))
        }
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::String(self.0))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::String)
    }
}

impl CityGmlAttribute for LocalId {
    fn parse_attribute_value(value: &str, _st: &mut ParseContext) -> Result<Self, ParseError> {
        let s = value;
        if let Some(id) = s.strip_prefix('#') {
            Ok(Self::from(id.to_string()))
        } else {
            Err(ParseError::InvalidValue(format!(
                "Expected a reference starts with '#' but got {s}"
            )))
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> [f32; 4] {
        [c.r as f32, c.g as f32, c.b as f32, 1.]
    }
}

impl std::hash::Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
    }
}

impl CityGmlElement for Color {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        let r: Result<Vec<_>, _> = text
            .split_ascii_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();
        match r {
            Ok(v) if v.len() == 3 => {
                (self.r, self.g, self.b) = (v[0], v[1], v[2]);
            }
            _ => {
                return Err(ParseError::InvalidValue(format!(
                    "Failed to parse color value: {text}"
                )))
            }
        }
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Array(vec![
            Value::Double(self.r),
            Value::Double(self.g),
            Value::Double(self.b),
        ]))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute {
            type_ref: schema::TypeRef::Double,
            min_occurs: 3,
            max_occurs: Some(3),
            original_name: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ColorPlusOpacity {
    pub r: Double01,
    pub g: Double01,
    pub b: Double01,
    pub a: Double01,
}

impl std::hash::Hash for ColorPlusOpacity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.a.to_bits().hash(state);
    }
}

impl ColorPlusOpacity {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

impl CityGmlElement for ColorPlusOpacity {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        let r: Result<Vec<_>, _> = text
            .split_ascii_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();
        match r {
            Ok(v) if v.len() == 3 => {
                (self.r, self.g, self.b, self.a) = (v[0], v[1], v[2], 1.0);
            }
            Ok(v) if v.len() == 4 => {
                (self.r, self.g, self.b, self.a) = (v[0], v[1], v[2], v[3]);
            }
            _ => {
                return Err(ParseError::InvalidValue(format!(
                    "Failed to parse color value: {text}"
                )))
            }
        }
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        Some(Value::Array(vec![
            Value::Double(self.r),
            Value::Double(self.g),
            Value::Double(self.b),
            Value::Double(self.a),
        ]))
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute {
            type_ref: schema::TypeRef::Double,
            min_occurs: 4,
            max_occurs: Some(4),
            original_name: None,
        }
    }
}

impl<T: CityGmlElement + Default> CityGmlElement for Option<T> {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        if self.is_some() {
            let current_path: &[u8] = &st.current_path();
            return Err(ParseError::SchemaViolation(format!(
                "{} must not occur two or more times.",
                String::from_utf8_lossy(current_path),
            )));
        }
        let mut v: T = Default::default();
        T::parse(&mut v, st)?;
        *self = Some(v);
        Ok(())
    }

    #[inline(never)]
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

impl<T: CityGmlElement + Default> CityGmlElement for Vec<T> {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let mut v: T = Default::default();
        <T as CityGmlElement>::parse(&mut v, st)?;
        self.push(v);
        Ok(())
    }

    #[inline(never)]
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

impl<T: CityGmlElement + Default> CityGmlElement for Box<T> {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        <T as CityGmlElement>::parse(self, st)?;
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        (*self).into_object()
    }

    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute {
        T::collect_schema(schema)
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Envelope {
    lower_corner: Point,
    upper_corner: Point,
    pub crs_uri: Option<String>,
}

impl CityGmlElement for Envelope {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        st.parse_attributes(|k, v, _| {
            if k == b"@srsName" {
                self.crs_uri = Some(String::from_utf8_lossy(v).into());
            }
            Ok(())
        })?;

        st.parse_children(|st| {
            let current_path: &[u8] = &st.current_path();
            match current_path {
                b"gml:lowerCorner" => self.lower_corner.parse(st)?,
                b"gml:upperCorner" => self.upper_corner.parse(st)?,
                _ => {
                    return Err(ParseError::SchemaViolation(format!(
                        "Expected gml:lowerCorner or gml:upperCorner, but got {}",
                        String::from_utf8_lossy(current_path),
                    )))
                }
            }
            Ok(())
        })?;
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<Value> {
        // todo
        None
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        schema::Attribute::new(schema::TypeRef::String)
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
    pub uri_attrs: Vec<(String, Uri)>,
    pub generic_attr_set: Vec<(String, GenericAttribute)>,
}

impl CityGmlElement for GenericAttribute {
    #[inline(never)]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let current_path: &[u8] = &st.current_path();
        match current_path {
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
                    String::from_utf8_lossy(current_path),
                )))
            }
        }
        Ok(())
    }

    #[inline(never)]
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
        map.extend(self.uri_attrs.into_iter().map(|(k, v)| (k, Value::Uri(v))));
        map.extend(
            self.generic_attr_set
                .into_iter()
                .flat_map(|(k, v)| match v.into_object() {
                    Some(Value::Object(data)) => Some((k, Value::Object(data))),
                    _ => None,
                }),
        );

        if map.is_empty() {
            None
        } else {
            Some(Value::Object(object::Object {
                typename: "gen:genericAttribute".into(),
                stereotype: object::ObjectStereotype::Data,
                attributes: map,
            }))
        }
    }

    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute {
        let key = "gen:genericAttribute";
        if schema.types.get(key).is_none() {
            schema.types.insert(
                key.into(),
                schema::TypeDef::Data(schema::DataTypeDef {
                    attributes: Default::default(),
                    additional_attributes: true,
                }),
            );
        }
        schema::Attribute::new(schema::TypeRef::Named(key.into()))
    }
}

fn parse_value<T, R: BufRead>(st: &mut SubTreeReader<R>) -> Result<(String, T), ParseError>
where
    T: CityGmlElement + Default,
{
    let mut name = None;
    let mut value = None;
    st.parse_attributes(|k, v, _| {
        // CityGML 2.0
        if k == b"@name" {
            name = Some(String::from_utf8_lossy(v).into());
        }
        Ok(())
    })?;
    st.parse_children(|st| {
        let current_path: &[u8] = &st.current_path();
        match current_path {
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
    st.parse_attributes(|k, v, _| {
        if k == b"@name" {
            name = Some(String::from_utf8_lossy(v).into());
        }
        Ok(())
    })?;
    st.parse_children(|st| {
        let current_path: &[u8] = &st.current_path();
        match current_path {
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
