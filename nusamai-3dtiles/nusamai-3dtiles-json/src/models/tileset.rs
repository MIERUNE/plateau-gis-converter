//! Tileset JSON

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::Value;

/// Metadata about the entire tileset.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// The 3D Tiles version. The version defines the JSON schema for the tileset JSON and the base set of tile formats.
    pub version: String,

    /// Application-specific version of this tileset, e.g., for when an existing tileset is updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tileset_version: Option<String>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            version: "1.1".to_string(),
            tileset_version: None,
            extensions: None,
            extra: None,
        }
    }
}

/// An object containing a reference to a class from a metadata schema, and property values that conform to the properties of that class.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MetadataEntity {
    /// The class that property values conform to. The value shall be a class ID declared in the `classes` dictionary of the metadata schema.
    #[serde(rename = "class")]
    pub class_: String,

    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value contains the property values. The type of the value shall match the property definition: For `BOOLEAN` use `true` or `false`. For `STRING` use a JSON string. For numeric types use a JSON number. For `ENUM` use a valid enum `name`, not an integer value. For `ARRAY`, `VECN`, and `MATN` types use a JSON array containing values matching the `componentType`. Required properties shall be included in this dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// A bounding volume that encloses a tile or its content. At least one bounding volume property is required. Bounding volumes include `box`, `region`, or `sphere`.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BoundingVolume {
    /// An array of 12 numbers that define an oriented bounding box. The first three elements define the x, y, and z values for the center of the box. The next three elements (with indices 3, 4, and 5) define the x axis direction and half-length. The next three elements (indices 6, 7, and 8) define the y axis direction and half-length. The last three elements (indices 9, 10, and 11) define the z axis direction and half-length.
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_: Option<[f64; 12]>,

    /// An array of six numbers that define a bounding geographic region in EPSG:4979 coordinates with the order [west, south, east, north, minimum height, maximum height]. Longitudes and latitudes are in radians. The range for latitudes is [-PI/2,PI/2]. The range for longitudes is [-PI,PI]. The value that is given as the 'south' of the region shall not be larger than the value for the 'north' of the region. The heights are in meters above (or below) the WGS84 ellipsoid. The 'minimum height' shall not be larger than the 'maximum height'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<[f64; 6]>,

    /// An array of four numbers that define a bounding sphere. The first three elements define the x, y, and z values for the center of the sphere. The last element (with index 3) defines the radius in meters. The radius shall not be negative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sphere: Option<[f64; 4]>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl BoundingVolume {
    pub fn new_box(box_: [f64; 12]) -> Self {
        Self {
            box_: Some(box_),
            ..Default::default()
        }
    }

    pub fn new_region(region: [f64; 6]) -> Self {
        Self {
            region: Some(region),
            ..Default::default()
        }
    }

    pub fn new_sphere(sphere: [f64; 4]) -> Self {
        Self {
            sphere: Some(sphere),
            ..Default::default()
        }
    }
}

/// An object describing the location of subtree files.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Subtrees {
    /// A template URI pointing to subtree files. A subtree is a fixed-depth (defined by `subtreeLevels`) portion of the tree to keep memory use bounded. The URI of each file is substituted with the subtree root's global level, x, and y. For subdivision scheme `OCTREE`, z shall also be given. Relative paths are relative to the tileset JSON.
    pub uri: String,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[serde(deny_unknown_fields)]
pub enum SubdivisionScheme {
    Quadtree,
    Octree,
}

/// This object allows a tile to be implicitly subdivided. Tile and content availability and metadata is stored in subtrees which are referenced externally.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ImplicitTiling {
    /// A string describing the subdivision scheme used within the tileset.
    pub subdivision_scheme: SubdivisionScheme,

    /// The number of distinct levels in each subtree. For example, a quadtree with `subtreeLevels = 2` will have subtrees with 5 nodes (one root and 4 children).
    pub subtree_levels: u32,

    /// The numbers of the levels in the tree with available tiles.
    pub available_levels: u32,

    /// An object describing the location of subtree files.
    pub subtrees: Subtrees,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,
}

/// Metadata about the tile's content and a link to the content.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Content {
    /// An optional bounding volume that tightly encloses tile content. tile.boundingVolume provides spatial coherence and tile.content.boundingVolume enables tight view frustum culling. When this is omitted, tile.boundingVolume is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_volume: Option<BoundingVolume>,

    /// A uri that points to tile content. When the uri is relative, it is relative to the referring tileset JSON file.
    pub uri: String,

    /// Metadata that is associated with this content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,

    /// The group this content belongs to. The value is an index into the array of `groups` that is defined for the containing tileset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Refine {
    Add,
    Replace,
}

/// A tile in a 3D Tiles tileset.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct Tile {
    /// The bounding volume that encloses the tile.
    pub bounding_volume: BoundingVolume,

    /// Optional bounding volume that defines the volume the viewer shall be inside of before the tile's content will be requested and before the tile will be refined based on geometricError.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_request_volume: Option<BoundingVolume>,

    /// The error, in meters, introduced if this tile is rendered and its children are not. At runtime, the geometric error is used to compute screen space error (SSE), i.e., the error measured in pixels.
    pub geometric_error: f64, // non-negative

    /// Specifies if additive or replacement refinement is used when traversing the tileset for rendering. This property is required for the root tile of a tileset; it is optional for all other tiles. The default is to inherit from the parent tile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refine: Option<Refine>,

    /// A floating-point 4x4 affine transformation matrix, stored in column-major order, that transforms the tile's content--i.e., its features as well as content.boundingVolume, boundingVolume, and viewerRequestVolume--from the tile's local coordinate system to the parent tile's coordinate system, or, in the case of a root tile, from the tile's local coordinate system to the tileset's coordinate system. `transform` does not apply to any volume property when the volume is a region, defined in EPSG:4979 coordinates. `transform` scales the `geometricError` by the maximum scaling factor from the matrix.
    #[serde(skip_serializing_if = "is_identity_matrix")]
    pub transform: [f64; 16],

    /// Metadata about the tile's content and a link to the content. When this is omitted the tile is just used for culling. When this is defined, then `contents` shall be undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// An array of contents. When this is defined, then `content` shall be undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<Content>>,

    /// A metadata entity that is associated with this tile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,

    /// An object that describes the implicit subdivision of this tile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_tiling: Option<ImplicitTiling>,

    /// An array of objects that define child tiles. Each child tile content is fully enclosed by its parent tile's bounding volume and, generally, has a geometricError less than its parent tile's geometricError. For leaf tiles, there are no children, and this property may not be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Tile>>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            bounding_volume: BoundingVolume::default(),
            viewer_request_volume: None,
            geometric_error: 0.0,
            refine: None,
            transform: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            content: None,
            contents: None,
            metadata: None,
            implicit_tiling: None,
            children: None,
            extensions: None,
            extra: None,
        }
    }
}

fn is_identity_matrix(a: &[f64; 16]) -> bool {
    *a == [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

/// An object containing metadata about a group.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GroupMetadata {
    /// The class that property values conform to. The value shall be a class ID declared in the `classes` dictionary of the metadata schema.
    #[serde(rename = "class")]
    pub class_: String,

    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value contains the property values. The type of the value shall match the property definition: For `BOOLEAN` use `true` or `false`. For `STRING` use a JSON string. For numeric types use a JSON number. For `ENUM` use a valid enum `name`, not an integer value. For `ARRAY`, `VECN`, and `MATN` types use a JSON array containing values matching the `componentType`. Required properties shall be included in this dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// A 3D Tiles tileset.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Tileset {
    /// Metadata about the entire tileset.
    pub asset: Asset,

    /// (deprecated) A dictionary object of metadata about per-feature properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,

    /// An object defining the structure of metadata classes and enums. When this is defined, then `schemaUri` shall be undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<nusamai_gltf_json::extensions::gltf::ext_structural_metadata::Schema>,

    /// The URI (or IRI) of the external schema file. When this is defined, then `schema` shall be undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_uri: Option<String>,

    /// An object containing statistics about metadata entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,

    /// An array of groups that tile content may belong to. Each element of this array is a metadata entity that describes the group. The tile content `group` property is an index into this array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupMetadata>>,

    /// A metadata entity that is associated with this tileset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,

    /// The error, in meters, introduced if this tileset is not rendered. At runtime, the geometric error is used to compute screen space error (SSE), i.e., the error measured in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometric_error: Option<f64>,

    /// The root tile.
    pub root: Tile,

    /// Names of 3D Tiles extensions used somewhere in this tileset.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions_used: Vec<String>,

    /// Names of 3D Tiles extensions required to properly load this tileset. Each element of this array shall also be contained in `extensionsUsed`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions_required: Vec<String>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// Statistics about entities.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Statistics {
    /// A dictionary, where each key corresponds to a class ID in the `classes` dictionary of the metatata schema that was defined for the tileset that contains these statistics. Each value is an object containing statistics about entities that conform to the class.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<HashMap<String, StatisticsClass>>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// Statistics about entities that conform to a class that was defined in a metadata schema.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StatisticsClass {
    /// The number of entities that conform to the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value is an object containing statistics about property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, StatisticsProperty>>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// Statistics about property values.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StatisticsProperty {
    /// The minimum property value occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub min: Option<f64>,

    /// The maximum property value occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub max: Option<f64>,

    /// The arithmetic mean of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the mean of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub mean: Option<f64>,

    /// The median of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the median of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub median: Option<f64>,

    /// The standard deviation of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the standard deviation of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub standard_deviation: Option<f64>,

    /// The variance of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the variance of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub variance: Option<f64>,

    /// The sum of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the sum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub sum: Option<f64>,

    /// A dictionary, where each key corresponds to an enum `name` and each value is the number of occurrences of that enum. Only applicable when `type` is `ENUM`. For fixed-length arrays, this is an array of component-wise occurrences.
    pub occurrences: Option<Value>, // integer or array of integer

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile() {
        let tile = Tile::default();
        assert_eq!(
            tile.transform,
            [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert!(!serde_json::to_string(&tile.transform)
            .unwrap()
            .contains("transform"));
    }
}
