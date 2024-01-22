//! Subtree JSON for the Implicit Tiling.

use std::collections::HashMap;

use nusamai_gltf::extensions::gltf::ext_structural_metadata;
use serde::{Deserialize, Serialize};

use super::tileset;
use serde_json::Value;

/// An object describing the availability of tiles and content in a subtree, as well as availability of children subtrees. May also store metadata for available tiles and content.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Subtree {
    /// An array of buffers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,

    /// An array of buffer views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<BufferView>>,

    /// An array of property tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_tables: Option<Vec<ext_structural_metadata::PropertyTable>>,

    /// The availability of tiles in the subtree. The availability bitstream is a 1D boolean array where tiles are ordered by their level in the subtree and Morton index within that level. A tile's availability is determined by a single bit, 1 meaning a tile exists at that spatial index, and 0 meaning it does not. The number of elements in the array is `(N^subtreeLevels - 1)/(N - 1)` where N is 4 for subdivision scheme `QUADTREE` and 8 for `OCTREE`. Availability may be stored in a buffer view or as a constant value that applies to all tiles. If a non-root tile's availability is 1 its parent tile's availability shall also be 1. `tileAvailability.constant: 0` is disallowed, as subtrees shall have at least one tile.
    pub tile_availability: Availability, // required

    /// An array of content availability objects. If the tile has a single content this array will have one element; if the tile has multiple contents - as supported by 3DTILES_multiple_contents and 3D Tiles 1.1 - this array will have multiple elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_availability: Option<Vec<Availability>>,

    /// The availability of children subtrees. The availability bitstream is a 1D boolean array where subtrees are ordered by their Morton index in the level of the tree immediately below the bottom row of the subtree. A child subtree's availability is determined by a single bit, 1 meaning a subtree exists at that spatial index, and 0 meaning it does not. The number of elements in the array is `N^subtreeLevels` where N is 4 for subdivision scheme `QUADTREE` and 8 for `OCTREE`. Availability may be stored in a buffer view or as a constant value that applies to all child subtrees. If availability is 0 for all child subtrees, then the tileset does not subdivide further.
    pub child_subtree_availability: Availability, // required

    /// Index of the property table containing tile metadata. Tile metadata only exists for available tiles and is tightly packed by increasing tile index. To access individual tile metadata, implementations may create a mapping from tile indices to tile metadata indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_metadata: Option<u32>,

    /// An array of indexes to property tables containing content metadata. If the tile has a single content this array will have one element; if the tile has multiple contents - as supported by 3DTILES_multiple_contents and 3D Tiles 1.1 - this array will have multiple elements. Content metadata only exists for available contents and is tightly packed by increasing tile index. To access individual content metadata, implementations may create a mapping from tile indices to content metadata indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_metadata: Option<Vec<u32>>,

    /// Subtree metadata encoded in JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtree_metadata: Option<tileset::MetadataEntity>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// An object describing the availability of a set of elements.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Availability {
    /// Index of a buffer view that indicates whether each element is available. The bitstream conforms to the boolean array encoding described in the 3D Metadata specification. If an element is available, its bit is 1, and if it is unavailable, its bit is 0.
    ///
    /// Either bitstream or constant is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream: Option<u32>,

    /// A number indicating how many 1 bits exist in the availability bitstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_count: Option<u32>,

    /// Integer indicating whether all of the elements are available (1) or all are unavailable (0).
    ///
    /// Either bitstream or constant is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<u8>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// A buffer is a binary blob. It is either the binary chunk of the subtree file, or an external buffer referenced by a URI.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Buffer {
    /// The URI (or IRI) of the file that contains the binary buffer data. Relative paths are relative to the file containing the buffer JSON. `uri` is required when using the JSON subtree format and not required when using the binary subtree format - when omitted the buffer refers to the binary chunk of the subtree file. Data URIs are not allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The length of the buffer in bytes.
    pub byte_length: u32, // >= 1, required

    /// The name of the buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

///A contiguous subset of a buffer
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    // The index of the buffer.
    pub buffer: u32, // required

    /// The offset into the buffer in bytes.
    pub byte_offset: u32, // required

    /// The total byte length of the buffer view.
    pub byte_length: u32, // >= 1, required

    /// The name of the `bufferView`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}
