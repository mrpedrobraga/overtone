use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeader {
    info: ArrangementHeaderInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderInfo {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderEditorInfo {
    requires_version: String,
}
