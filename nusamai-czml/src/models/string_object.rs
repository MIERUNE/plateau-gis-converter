use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StringObject {
    string: Option<String>,
    reference: Option<String>,
}
