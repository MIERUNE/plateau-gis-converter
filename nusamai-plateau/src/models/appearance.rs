use std::io::BufRead;

use nusamai_citygml::appearance::TextureAssociation;
use nusamai_citygml::{
    citygml_feature, citygml_property, schema, CityGmlElement, Code, LocalId, ParseError, Point,
    SubTreeReader, Value, URI,
};

type Double01 = f64; // TODO?
type TextureType = String; // TODO?
type WrapMode = String; // TODO?
type TransformationMatrix2x2 = String; // FIXME

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl CityGmlElement for Color {
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
                    "Failed to parse color value: {}",
                    text
                )))
            }
        }
        Ok(())
    }

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
        }
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ColorPlusOpacity {
    pub r: Double01,
    pub g: Double01,
    pub b: Double01,
    pub a: Double01,
}

impl CityGmlElement for ColorPlusOpacity {
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
                    "Failed to parse color value: {}",
                    text
                )))
            }
        }
        Ok(())
    }

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
        }
    }
}

#[citygml_property(name = "_:_AppearanceProperty")]
pub enum AppearanceProperty {
    #[citygml(path = b"app:Appearance")]
    Appearance(Appearance),
}

#[citygml_feature(name = "app:Appearance", noncityobj)]
pub struct Appearance {
    #[citygml(path = b"app:theme")]
    pub theme: Option<String>,

    #[citygml(path = b"app:surfaceDataMember")]
    pub surface_data_member: Vec<SurfaceDataProperty>, // -> app:_SurfaceData
}

#[citygml_property(name = "app:_SurfaceDataProperty")]
pub enum SurfaceDataProperty {
    #[citygml(path = b"app:X3DMaterial")]
    X3DMaterial(X3DMaterial),
    #[citygml(path = b"app:ParameterizedTexture")]
    ParameterizedTexture(ParameterizedTexture),
    #[citygml(path = b"app:GeoreferencedTexture")]
    GeoreferencedTexture(GeoreferencedTexture),
}

#[citygml_feature(name = "app:X3DMaterial", noncityobj)]
#[derive(Clone)]
pub struct X3DMaterial {
    #[citygml(path = b"app:isFront")]
    pub is_front: Option<bool>,

    #[citygml(path = b"app:ambientIntensity")]
    pub ambient_intensity: Option<Double01>,

    #[citygml(path = b"app:diffuseColor")]
    pub diffuse_color: Option<Color>,

    #[citygml(path = b"app:emissiveColor")]
    pub emissive_color: Option<Color>,

    #[citygml(path = b"app:specularColor")]
    pub specular_color: Option<Color>,

    #[citygml(path = b"app:shininess")]
    pub shininess: Option<Double01>,

    #[citygml(path = b"app:transparency")]
    pub transparency: Option<Double01>,

    #[citygml(path = b"app:isSmooth")]
    pub is_smooth: Option<bool>,

    #[citygml(path = b"app:target")]
    pub target: Vec<LocalId>,
}

#[citygml_feature(name = "app:ParameterizedTexture", noncityobj)]
#[citygml(allow_extra)]
#[derive(Clone)]
pub struct ParameterizedTexture {
    #[citygml(path = b"app:isFront")]
    pub is_front: Option<bool>,

    #[citygml(path = b"app:imageURI", required)]
    pub image_uri: Option<URI>,

    #[citygml(path = b"app:mimeType")]
    pub mime_type: Option<Code>,

    #[citygml(path = b"app:textureType")]
    pub texture_type: Option<TextureType>,

    #[citygml(path = b"app:wrapMode")]
    pub wrap_mode: Option<WrapMode>,

    #[citygml(path = b"app:borderColor")]
    pub border_color: Option<ColorPlusOpacity>,

    #[citygml(path = b"app:target")]
    pub target: Vec<TextureAssociation>,
}

#[citygml_feature(name = "app:GeoreferencedTexture", noncityobj)]
pub struct GeoreferencedTexture {
    #[citygml(path = b"app:isFront")]
    pub is_front: Option<bool>,

    #[citygml(path = b"app:imageURI", required)]
    pub image_uri: Option<URI>,

    #[citygml(path = b"app:mimeType")]
    pub mime_type: Option<Code>,

    #[citygml(path = b"app:textureType")]
    pub texture_type: Option<TextureType>,

    #[citygml(path = b"app:wrapMode")]
    pub wrap_mode: Option<WrapMode>,

    #[citygml(path = b"app:borderColor")]
    pub border_color: Option<ColorPlusOpacity>,

    #[citygml(path = b"app:preferWorldFile")]
    pub prefer_world_file: Option<bool>,

    #[citygml(path = b"app:referencePoint/gml:Point")]
    pub reference_point: Option<Point>,

    #[citygml(path = b"app:orientation")]
    pub orientation: Option<TransformationMatrix2x2>,

    #[citygml(path = b"app:target")]
    pub target: Vec<URI>,
}
