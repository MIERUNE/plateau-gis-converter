use crate::{schema, CityGmlElement, LocalId, ParseError, SubTreeReader};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum TextureAssociation {
    #[default]
    Unknown,
    TexCoordList(TexCoordList),
    // TODO: TexCoordGen
    // #[citygml(path = b"app:TexCoordGen")]
    // TexCoordGen(TexCoordGen),
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TexCoordList {
    pub target: LocalId,
    pub rings: Vec<LocalId>,
    pub coords_list: Vec<Vec<f64>>,
}

impl CityGmlElement for TextureAssociation {
    fn parse<R: std::io::BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        *self = st.parse_texture_association()?;
        Ok(())
    }

    #[inline(never)]
    fn into_object(self) -> Option<crate::object::Value> {
        None
    }

    fn collect_schema(_schema: &mut schema::Schema) -> schema::Attribute {
        // do nothing ?
        todo!();
    }
}
