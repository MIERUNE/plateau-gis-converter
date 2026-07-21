//! Intermediate data models shared by vector tile sinks.

use flatgeom::MultiPolygon2;
use nusamai_citygml::object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct SlicedFeature<'a> {
    pub(crate) geometry: MultiPolygon2<'a>,
    pub(crate) properties: object::Value,
}

pub(crate) fn hash_feature_id(id: &str) -> u64 {
    id.as_bytes().iter().fold(5381u64, |hash, byte| {
        hash.wrapping_mul(33) ^ u64::from(*byte)
    })
}

#[cfg(test)]
mod tests {
    use super::hash_feature_id;

    #[test]
    fn feature_id_hash_is_stable() {
        assert_eq!(hash_feature_id("gml_123"), 229_371_511_026_220);
    }
}
