use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Rules specified by the user in a JSON file
#[derive(Serialize, Deserialize, Debug)]
pub struct MappingRules {
    pub rename: RenameRules,
}

/// Rules specified by the user to rename the attributes
/// Used by the `EditFieldNamesTransform` transformer
pub type RenameRules = HashMap<String, String>;
