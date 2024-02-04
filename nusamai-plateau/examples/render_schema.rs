use std::io;

use nusamai_citygml::{schema, schema::Schema, CityGmlElement};
use nusamai_plateau::models::TopLevelCityObject;

fn main() {
    let mut schema = Schema::default();
    TopLevelCityObject::collect_schema(&mut schema);
    schema.types.sort_keys();
    serde_json::to_writer_pretty(io::stdout(), &schema).unwrap();
    eprintln!(
        "{} Feature types",
        schema
            .types
            .iter()
            .filter(|(_, v)| matches!(v, schema::TypeDef::Feature(_)))
            .count()
    );
    eprintln!(
        "{} Data types",
        schema
            .types
            .iter()
            .filter(|(_, v)| matches!(v, schema::TypeDef::Data(_)))
            .count()
    );
    eprintln!(
        "{} Property types",
        schema
            .types
            .iter()
            .filter(|(_, v)| matches!(v, schema::TypeDef::Property(_)))
            .count()
    );
}
