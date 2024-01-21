use crate::{schema, CityGMLElement, LocalHref, ParseError, SubTreeReader};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum TextureAssociation {
    #[default]
    Unknown,
    TexCoordList(TexCoordList),
    // TODO: TexCoordGen
    // #[citygml(path = b"app:TexCoordGen")]
    // TexCoordGen(TexCoordGen),
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TexCoordList {
    pub target: LocalHref,
    pub rings: Vec<LocalHref>,
    pub coords_list: Vec<Vec<f64>>,
}

impl CityGMLElement for TextureAssociation {
    fn parse<R: std::io::BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        *self = st.parse_texture_association()?;
        Ok(())
    }

    fn into_object(self) -> Option<crate::object::Value> {
        None
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        // do nothing ?
        todo!();
    }
}
