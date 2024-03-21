use nusamai_citygml::{
    appearance::TextureAssociation, citygml_feature, citygml_property, CityGmlElement, Code, Color,
    ColorPlusOpacity, Double01, LocalId, Point, Uri,
};

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
    pub image_uri: Option<Uri>,

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
    pub image_uri: Option<Uri>,

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
    pub target: Vec<Uri>,
}
