use std::io::BufRead;

use nusamai_citygml::{
    citygml_data, citygml_feature, citygml_property, schema, CityGMLElement, Code, LocalHref,
    ParseError, Point, SubTreeReader, Value, URI,
};

type Double01 = f64; // TODO?
type Color = String; // FIXME
type ColorPlusOpacity = String; // FIXME
type TextureType = String; // TODO?
type WrapMode = String; // TODO?
type TransformationMatrix2x2 = String; // FIXME

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
    pub target: Vec<LocalHref>,
}

#[citygml_feature(name = "app:ParameterizedTexture", noncityobj)]
#[citygml(allow_extra)]
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

#[citygml_property(name = "app:_TextureAssociation", noncityobj)]
pub enum TextureAssociation {
    #[citygml(path = b"app:TexCoordList")]
    TexCoordList(TexCoordList),
    #[citygml(path = b"app:TexCoordGen")]
    TexCoordGen(TexCoordGen),
}

// pub struct TexCoordList {
//     #[citygml(path = b"@app:ring")]
//     pub ring: Option<LocalHref>,
//
//     #[citygml(path = b"app:textureCoordinates")]
//     pub texture_coordinates: Option<String>,
// }

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TexCoordList {
    ring: LocalHref,
    coords: Vec<f64>,
}

impl CityGMLElement for TexCoordList {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        st.parse_attributes(|k, v, st| {
            if k == b"@gml:ring" {
                todo!();
            }
            Ok(())
        })?;
        self.coords = todo!();
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        None
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        // do nothing ?
    }
}

#[citygml_data(name = "app:TexCoordGen", noncityobj)]
pub struct TexCoordGen {
    // TODO?
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
