pub mod option;
pub mod parameters;
pub mod pipeline;
pub mod sink;
pub mod source;
pub mod transformer;

pub static BUILTIN_SINKS: &[&dyn sink::DataSinkProvider] = &[
    &sink::cesiumtiles::CesiumTilesSinkProvider {},
    &sink::gpkg::GpkgSinkProvider {},
    &sink::mvt::MvtSinkProvider {},
    &sink::geojson::GeoJsonSinkProvider {},
    &sink::czml::CzmlSinkProvider {},
    &sink::gltf::GltfSinkProvider {},
    &sink::kml::KmlSinkProvider {},
    &sink::ply::StanfordPlySinkProvider {},
    &sink::serde::SerdeSinkProvider {},
    &sink::shapefile::ShapefileSinkProvider {},
    &sink::noop::NoopSinkProvider {},
    &sink::minecraft::MinecraftSinkProvider {},
    &sink::obj::ObjSinkProvider {},
];
